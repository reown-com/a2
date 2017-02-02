use solicit::http::client::tls::TlsConnector;
use solicit::client::{Client};
use time::precise_time_ns;
use std::result::Result;

use client::response::ProviderResponse;
use client::headers::{default_headers, create_header};
use client::error::ProviderError;
use notification::Notification;

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
