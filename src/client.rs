//! The client module for sending requests and parsing responses

use signer::Signer;
use alpn::AlpnConnector;
use error::Error;
use error::Error::ResponseError;
use futures::{Future, Poll};
use futures::future::{err, ok};
use futures::stream::Stream;
use hyper::{Client as HttpClient, HttpVersion};
use hyper::{Post, StatusCode};
use hyper::client::{Request, Response as HttpResponse};
use hyper::header::{Authorization, Bearer, ContentLength, ContentType};
use request::payload::Payload;
use response::Response;
use serde_json;
use std::{fmt, str};
use std::time::Duration;
use tokio_core::reactor::Handle;
use tokio_service::Service;
use openssl::pkcs12::Pkcs12;
use std::io::Read;
use tokio_timer::{Timeout, Timer};

#[derive(Debug, Clone)]
pub enum Endpoint {
    Production,
    Sandbox,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let host = match self {
            &Endpoint::Production => "api.push.apple.com",
            &Endpoint::Sandbox => "api.development.push.apple.com",
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
    timer: Timer,
}

impl Client {
    fn new(
        connector: AlpnConnector,
        signer: Option<Signer>,
        endpoint: Endpoint,
        handle: &Handle,
    ) -> Client {
        let timeout = 15 * 60;

        let builder = HttpClient::configure()
            .keep_alive_timeout(Some(Duration::from_secs(timeout)))
            .http2_only()
            .keep_alive(true);

        Client {
            http_client: builder.connector(connector).build(handle),
            signer: signer,
            endpoint: endpoint,
            timer: Timer::default(),
        }
    }

    pub fn certificate<R>(
        certificate: &mut R,
        password: &str,
        handle: &Handle,
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
            handle,
        )?;

        Ok(Self::new(connector, None, endpoint, handle))
    }

    pub fn token<S, R>(
        pkcs8_pem: R,
        key_id: S,
        team_id: S,
        handle: &Handle,
        endpoint: Endpoint,
    ) -> Result<Client, Error>
    where
        S: Into<String>,
        R: Read,
    {
        let connector = AlpnConnector::new(handle);
        let signature_ttl = 60 * 45; // seconds
        let signer = Signer::new(pkcs8_pem, key_id, team_id, signature_ttl)?;

        Ok(Self::new(connector, Some(signer), endpoint, handle))
    }

    /// Send a notification payload. Returns a future that needs to be given to
    /// an executor.
    #[inline]
    pub fn send(&self, payload: Payload) -> FutureResponse {
        self.call(payload)
    }

    /// Sends a notification with a timeout. Triggers `Error::TimeoutError` if the request takes too long.
    #[inline]
    pub fn send_with_timeout(
        &self,
        message: Payload,
        timeout: Duration,
    ) -> Timeout<FutureResponse> {
        self.timer.timeout(self.send(message), timeout)
    }
}

pub struct FutureResponse(Box<Future<Item = Response, Error = Error> + 'static>);

impl fmt::Debug for FutureResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Future<Response>")
    }
}

impl Future for FutureResponse {
    type Item = Response;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl Service for Client {
    type Request = Payload;
    type Response = Response;
    type Error = Error;
    type Future = FutureResponse;

    fn call(&self, payload: Self::Request) -> Self::Future {
        let path = format!(
            "https://{}/3/device/{}",
            self.endpoint, payload.device_token
        );
        let payload_json = payload.to_json_string().unwrap();
        let mut request = Request::new(Post, path.parse().unwrap());

        request.set_version(HttpVersion::Http2);
        request.headers_mut().set(ContentType::json());

        request
            .headers_mut()
            .set(ContentLength(payload_json.len() as u64));

        request.headers_mut().set_raw(
            "apns-priority",
            format!("{}", payload.options.apns_priority),
        );

        if let Some(ref apns_id) = payload.options.apns_id {
            request
                .headers_mut()
                .set_raw("apns-id", format!("{}", apns_id));
        }
        if let Some(ref apns_expiration) = payload.options.apns_expiration {
            request
                .headers_mut()
                .set_raw("apns-expiration", format!("{}", apns_expiration));
        }
        if let Some(ref apns_collapse_id) = payload.options.apns_collapse_id {
            request
                .headers_mut()
                .set_raw("apns-collapse-id", format!("{}", apns_collapse_id.value))
        }

        if let Some(ref apns_topic) = payload.options.apns_topic {
            request
                .headers_mut()
                .set_raw("apns-topic", apns_topic.as_bytes());
        }
        if let Some(ref signer) = self.signer {
            signer
                .with_signature(|signature| {
                    request.headers_mut().set(Authorization(Bearer {
                        token: signature.to_owned(),
                    }));
                })
                .unwrap();
        }

        let request_body = payload.to_json_string().unwrap();
        request.set_body(request_body.into_bytes());

        let request_f = self.http_client
            .request(request)
            .map_err(|e| {
                trace!("Request error: {}", e);
                Error::ConnectionError
            });

        trace!("Client::call requesting ({:?})", path);
        let apns_f = request_f.and_then(move |response: HttpResponse| {
            let response_status = response.status().clone();

            trace!(
                "Client::call got response status {} from ({:?})",
                response_status,
                path
            );

            let apns_id = match response.headers().get_raw("apns-id").and_then(|h| h.one()) {
                Some(apns_id) => String::from_utf8(apns_id.to_vec()).ok(),
                None => None,
            };

            response
                .body()
                .map_err(|e| {
                    trace!("Body error: {}", e);
                    Error::ConnectionError
                })
                .concat2()
                .and_then(move |body_chunk| match response_status {
                    StatusCode::Ok => ok(Response {
                        apns_id: apns_id,
                        error: None,
                        code: response_status.as_u16(),
                    }),
                    _ => {
                        if let Ok(body) = str::from_utf8(&body_chunk.to_vec()) {
                            err(ResponseError(Response {
                                apns_id: apns_id,
                                error: serde_json::from_str(body).ok(),
                                code: response_status.as_u16(),
                            }))
                        } else {
                            err(ResponseError(Response {
                                apns_id: None,
                                error: None,
                                code: response_status.as_u16(),
                            }))
                        }
                    }
                })
        });

        FutureResponse(Box::new(apns_f))
    }
}
