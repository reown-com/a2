use solicit::http::client::tls::TlsConnector;
use solicit::client::{Client};
use time::precise_time_ns;
use std::result::Result;

use client::response::ProviderResponse;
use client::headers::{default_headers, create_header};
use client::error::ProviderError;
use notification::Notification;

/// Creates a new connection to APNs using the system certificates. When sending
/// notifications through this type of connection, one must attach a valid JWT
/// token with every request.
///
/// Sends a push notification. Responds with a channel, which can be handled in the same thread or
/// sent out to be handled elsewhere.
///
/// # Examples
/// ```no_run
/// # extern crate apns2;
/// # fn main() {
/// use apns2::client::TokenClient;
/// use apns2::apns_token::ApnsToken;
/// use apns2::device_token::DeviceToken;
/// use apns2::payload::{Payload, APSAlert};
/// use apns2::notification::{Notification, NotificationOptions};
/// use std::fs::File;
/// use std::time::Duration;
///
/// let der_file = File::open("/path/to/key.der").unwrap();
/// let apns_token = ApnsToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
/// let client = TokenClient::new(false, "/etc/ssl/cert.pem").unwrap();
/// let device_token = DeviceToken::new("apple_device_token");
/// let payload = Payload::new(APSAlert::Plain("Hi there!"), 1u32, "default", None, None);
///
/// let options = NotificationOptions {
///     ..Default::default()
/// };
///
/// let request = client.push(Notification::new(payload, device_token, options), apns_token.signature());
/// let response = request.recv_timeout(Duration::from_millis(2000));
/// println!("{:?}", response);
/// # }
///```

static DEVELOPMENT: &'static str = "api.development.push.apple.com";
static PRODUCTION: &'static str = "api.push.apple.com";

pub struct TokenClient {
    pub client: Client,
}

impl TokenClient {
    pub fn new<'a>(sandbox: bool, certificates: &str) -> Result<TokenClient, ProviderError<'a>> {
        let host = if sandbox { DEVELOPMENT } else { PRODUCTION };
        let connector = TlsConnector::new(host, &certificates);
        let client = match Client::with_connector(connector) {
            Ok(client) => client,
            Err(_) => return Err(ProviderError::ClientConnectError("Couldn't connect to APNs service"))
        };

        Ok(TokenClient {
            client: client,
        })
    }

    pub fn push(&self, notification: Notification, apns_token: &str) -> ProviderResponse {
        let path = format!("/3/device/{}", notification.device_token).into_bytes();
        let mut headers = default_headers(&notification);
        let body = notification.payload.to_string().into_bytes();

        headers.push(create_header("authorization", format!("bearer {}", apns_token)));

        let request = self.client.post(&path, headers.as_slice(), body);

        ProviderResponse::new(request, precise_time_ns())
    }
}
