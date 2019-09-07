//! The client module for sending requests and parsing responses

use crate::signer::Signer;
use hyper_alpn::AlpnConnector;
use crate::error::Error;
use crate::error::Error::ResponseError;

use futures::stream::TryStreamExt;
use hyper::{
    self,
    Client as HttpClient,
    StatusCode,
    Body
};
use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use crate::request::payload::Payload;
use crate::response::Response;
use serde_json;
use std::{fmt, str};
use std::time::Duration;
use openssl::pkcs12::Pkcs12;
use std::io::Read;

/// The APNs service endpoint to connect.
#[derive(Debug, Clone)]
pub enum Endpoint {
    /// The production environment (api.push.apple.com)
    Production,
    /// The development/test environment (api.development.push.apple.com)
    Sandbox,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = match self {
            Endpoint::Production => "api.push.apple.com",
            Endpoint::Sandbox => "api.development.push.apple.com",
        };

        write!(f, "{}", host)
    }
}

/// Handles requests to and responses from Apple Push Notification service.
/// Connects using a given connector. Handles the needed authentication and
/// maps responses.
///
/// The `send` method returns a future, which is successful when APNs receives
/// the notification and responds with a status OK. In any other case the future
/// fails. If APNs gives a reason for the failure, the returned `Err`
/// holds the response for handling.
pub struct Client {
    endpoint: Endpoint,
    signer: Option<Signer>,
    http_client: HttpClient<AlpnConnector>,
}

impl Client {
    fn new(
        connector: AlpnConnector,
        signer: Option<Signer>,
        endpoint: Endpoint,
    ) -> Client {
        let mut builder = HttpClient::builder();
        builder.keep_alive_timeout(Some(Duration::from_secs(600)));
        builder.http2_only(true);
        builder.keep_alive(true);

        Client {
            http_client: builder.build(connector),
            signer: signer,
            endpoint: endpoint,
        }
    }

    /// Create a connection to APNs using the provider client certificate which
    /// you obtain from your [Apple developer
    /// account](https://developer.apple.com/account/).
    pub fn certificate<R>(
        certificate: &mut R,
        password: &str,
        endpoint: Endpoint,
    ) -> Result<Client, Error>
    where
        R: Read,
    {
        let mut cert_der: Vec<u8> = Vec::new();
        certificate.read_to_end(&mut cert_der)?;

        let pkcs = Pkcs12::from_der(&cert_der)?.parse(password)?;
        let connector = AlpnConnector::with_client_cert(
            &pkcs.cert.to_pem()?,
            &pkcs.pkey.private_key_to_pem_pkcs8()?,
        )?;

        Ok(Self::new(connector, None, endpoint))
    }

    /// Create a connection to APNs using system certificates, signing every
    /// request with a signature using a private key, key id and team id
    /// provisioned from your [Apple developer
    /// account](https://developer.apple.com/account/).
    pub fn token<S, T, R>(
        pkcs8_pem: R,
        key_id: S,
        team_id: T,
        endpoint: Endpoint,
    ) -> Result<Client, Error>
    where
        S: Into<String>,
        T: Into<String>,
        R: Read,
    {
        let connector = AlpnConnector::new();
        let signature_ttl = 60 * 45; // seconds
        let signer = Signer::new(pkcs8_pem, key_id, team_id, signature_ttl)?;

        Ok(Self::new(connector, Some(signer), endpoint))
    }

    /// Send a notification payload. Returns a future that needs to be given to
    /// an executor.
    pub async fn send<'a>(&self, payload: Payload<'a>) -> Result<Response, Error> {
        let request = self.build_request(payload).await;
        let path = format!("{}", request.uri());

        let response = self.http_client.request(request).await?;
        trace!("Client::call got response status {} from ({:?})", response.status(), path);

        let apns_id = response
            .headers()
            .get("apns-id")
            .and_then(|s| s.to_str().ok())
            .map(|id| String::from(id));

        match response.status() {
            StatusCode::OK => {
                Ok(Response {
                    apns_id,
                    error: None,
                    code: response.status().as_u16(),
                })
            },
            status => {
                let body = response.into_body().try_concat().await?;

                Err(ResponseError(Response {
                    apns_id,
                    error: serde_json::from_slice(&body).ok(),
                    code: status.as_u16(),
                }))
            }
        }
    }

    async fn build_request(&self, payload: Payload<'_>) -> hyper::Request<Body> {
        let path = format!(
            "https://{}/3/device/{}",
            self.endpoint, payload.device_token
        );

        let mut builder = hyper::Request::builder();

        builder.uri(&path);
        builder.method("POST");
        builder.header(CONTENT_TYPE, "application/json");
        builder.header("apns-priority", format!("{}", payload.options.apns_priority).as_bytes());

        if let Some(ref apns_id) = payload.options.apns_id {
            builder.header("apns-id", apns_id.as_bytes());
        }
        if let Some(ref apns_expiration) = payload.options.apns_expiration {
            builder.header("apns-expiration", format!("{}", apns_expiration).as_bytes());
        }
        if let Some(ref apns_collapse_id) = payload.options.apns_collapse_id {
            builder.header("apns-collapse-id", format!("{}", apns_collapse_id.value).as_bytes());
        }
        if let Some(ref apns_topic) = payload.options.apns_topic {
            builder.header("apns-topic", apns_topic.as_bytes());
        }
        if let Some(ref signer) = self.signer {
            signer.with_signature(|signature| {
                builder.header(AUTHORIZATION, format!("Bearer {}", signature).as_bytes());
            }).await.unwrap();
        }

        let payload_json = payload.to_json_string().unwrap();
        builder.header(CONTENT_LENGTH, format!("{}", payload_json.len()).as_bytes());

        let request_body = Body::from(payload_json);
        builder.body(request_body).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::notification::PlainNotificationBuilder;
    use crate::request::notification::NotificationBuilder;
    use crate::request::notification::{NotificationOptions, Priority, CollapseId};
    use hyper_alpn::AlpnConnector;
    use hyper::Method;
    use http::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
    use crate::signer::Signer;

    const PRIVATE_KEY: &'static str = indoc!(
        "-----BEGIN PRIVATE KEY-----
        MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg8g/n6j9roKvnUkwu
        lCEIvbDqlUhA5FOzcakkG90E8L+hRANCAATKS2ZExEybUvchRDuKBftotMwVEus3
        jDwmlD1Gg0yJt1e38djFwsxsfr5q2hv0Rj9fTEqAPr8H7mGm0wKxZ7iQ
        -----END PRIVATE KEY-----"
    );

    #[tokio::test]
    async fn test_production_request_uri() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let uri = format!("{}", request.uri());

        assert_eq!("https://api.push.apple.com/3/device/a_test_id", &uri);
    }

    #[tokio::test]
    async fn test_sandbox_request_uri() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Sandbox);
        let request = client.build_request(payload).await;
        let uri = format!("{}", request.uri());

        assert_eq!("https://api.development.push.apple.com/3/device/a_test_id", &uri);
    }

    #[tokio::test]
    async fn test_request_method() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;

        assert_eq!(&Method::POST, request.method());
    }

    #[tokio::test]
    async fn test_request_content_type() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;

        assert_eq!("application/json", request.headers().get(CONTENT_TYPE).unwrap());
    }

    #[tokio::test]
    async fn test_request_content_length() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload.clone()).await;
        let payload_json = payload.to_json_string().unwrap();
        let content_length = request.headers().get(CONTENT_LENGTH).unwrap().to_str().unwrap();

        assert_eq!(
            &format!("{}", payload_json.len()),
            content_length
        );
    }

    #[tokio::test]
    async fn test_request_authorization_with_no_signer() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;

        assert_eq!(None, request.headers().get(AUTHORIZATION));
    }

    #[tokio::test]
    async fn test_request_authorization_with_a_signer() {
        let signer = Signer::new(PRIVATE_KEY.as_bytes(), "89AFRD1X22", "ASDFQWERTY", 100).unwrap();
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), Some(signer), Endpoint::Production);
        let request = client.build_request(payload).await;

        assert_ne!(None, request.headers().get(AUTHORIZATION));
    }

    #[tokio::test]
    async fn test_request_with_normal_priority() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_priority = request.headers().get("apns-priority").unwrap();

        assert_eq!("5", apns_priority);
    }

    #[tokio::test]
    async fn test_request_with_high_priority() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            NotificationOptions {
                apns_priority: Priority::High,
                ..Default::default()
            }
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_priority = request.headers().get("apns-priority").unwrap();

        assert_eq!("10", apns_priority);
    }

    #[tokio::test]
    async fn test_request_with_default_apns_id() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            Default::default(),
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_id = request.headers().get("apns-id");

        assert_eq!(None, apns_id);
    }

    #[tokio::test]
    async fn test_request_with_an_apns_id() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            NotificationOptions {
                apns_id: Some("a-test-apns-id"),
                ..Default::default()
            },
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_id = request.headers().get("apns-id").unwrap();

        assert_eq!("a-test-apns-id", apns_id);
    }

    #[tokio::test]
    async fn test_request_with_default_apns_expiration() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            Default::default(),
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_expiration = request.headers().get("apns-expiration");

        assert_eq!(None, apns_expiration);
    }

    #[tokio::test]
    async fn test_request_with_an_apns_expiration() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            NotificationOptions {
                apns_expiration: Some(420),
                ..Default::default()
            },
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_expiration = request.headers().get("apns-expiration").unwrap();

        assert_eq!("420", apns_expiration);
    }

    #[tokio::test]
    async fn test_request_with_default_apns_collapse_id() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            Default::default(),
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_collapse_id = request.headers().get("apns-collapse-id");

        assert_eq!(None, apns_collapse_id);
    }

    #[tokio::test]
    async fn test_request_with_an_apns_collapse_id() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            NotificationOptions {
                apns_collapse_id: Some(CollapseId::new("a_collapse_id").unwrap()),
                ..Default::default()
            },
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_collapse_id = request.headers().get("apns-collapse-id").unwrap();

        assert_eq!("a_collapse_id", apns_collapse_id);
    }

    #[tokio::test]
    async fn test_request_with_default_apns_topic() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            Default::default(),
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_topic = request.headers().get("apns-topic");

        assert_eq!(None, apns_topic);
    }

    #[tokio::test]
    async fn test_request_with_an_apns_topic() {
        let builder = PlainNotificationBuilder::new("test");

        let payload = builder.build(
            "a_test_id",
            NotificationOptions {
                apns_topic: Some("a_topic"),
                ..Default::default()
            },
        );

        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload).await;
        let apns_topic = request.headers().get("apns-topic").unwrap();

        assert_eq!("a_topic", apns_topic);
    }

    #[tokio::test]
    async fn test_request_body() {
        let builder = PlainNotificationBuilder::new("test");
        let payload = builder.build("a_test_id", Default::default());
        let client = Client::new(AlpnConnector::new(), None, Endpoint::Production);
        let request = client.build_request(payload.clone()).await;
        let body_chunk = request.into_body().try_concat().await.unwrap();
        let body_str = String::from_utf8(body_chunk.to_vec()).unwrap();

        assert_eq!(
            payload.to_json_string().unwrap(),
            body_str,
        );
    }
}
