use solicit::http::client::tls::{TlsConnector};
use solicit::client::{Client};
use time::precise_time_ns;
use std::result::Result;

use client::response::ProviderResponse;
use client::headers::{default_headers, create_header};
use client::error::ProviderError;
use notification::Notification;
use client::{DEVELOPMENT, PRODUCTION};

pub struct TokenClient {
    pub client: Client,
}

/// Creates a new connection to APNs using the system certificates. When sending
/// notifications through this type of connection, one must attach a valid JWT
/// token with every request using either `APNSToken` or an own implementation.
/// The same connection can be used to send notifications to multiple applications.
///
/// The response for `push` is asynchronous for better throughput.
///
/// # Examples
/// ```no_run
/// # extern crate apns2;
/// # fn main() {
/// use apns2::client::TokenClient;
/// use apns2::apns_token::APNSToken;
/// use apns2::payload::{Payload, APSAlert};
/// use apns2::notification::{Notification, NotificationOptions};
/// use std::fs::File;
/// use std::time::Duration;
///
/// // Can be anything that implements the `Read` trait.
/// let der_file = File::open("/path/to/key.der").unwrap();
///
/// let apns_token = APNSToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
///
/// // Example certificate file from Ubuntu
/// let client = TokenClient::new(false, "/etc/ssl/certs/ca-certificates.crt").unwrap();
///
/// let alert = APSAlert::Plain(String::from("Hi there!"));
/// let payload = Payload::new(alert, "default", Some(1u32), None, None);
/// let options = NotificationOptions { ..Default::default() };
/// let request = client.push(Notification::new(payload, "Hi there!", options), apns_token.signature());
///
/// // Block here to get the response.
/// let response = request.recv_timeout(Duration::from_millis(2000));
///
/// println!("{:?}", response);
/// # }
/// ```
impl TokenClient {
    /// Create a new connection to APNs. `certificates` should point to system ca certificate
    /// file. In Ubuntu it's usually `/etc/ssl/certs/ca-certificates.crt`.
    pub fn new<'a>(sandbox: bool, certificates: &str) -> Result<TokenClient, ProviderError> {
        let host = if sandbox { DEVELOPMENT } else { PRODUCTION };
        let connector = TlsConnector::new(host, &certificates);
        let client = Client::with_connector(connector)?;

        Ok(TokenClient {
            client: client,
        })
    }

    /// Send a push notification with a JWT signature.
    pub fn push(&self, notification: Notification, apns_token: &str) -> ProviderResponse {
        let path = format!("/3/device/{}", notification.device_token).into_bytes();
        let mut headers = default_headers(&notification);
        let body = notification.payload.to_string().into_bytes();

        headers.push(create_header("authorization", format!("bearer {}", apns_token)));

        let request = self.client.post(&path, headers.as_slice(), body);

        ProviderResponse::new(request, precise_time_ns())
    }
}
