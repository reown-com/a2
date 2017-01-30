use solicit::http::client::tls::TlsConnector;
use solicit::client::{Client};
use openssl::ssl::{SslContext, SslMethod, SSL_VERIFY_NONE};
use openssl::x509::X509;
use openssl::crypto::pkey::PKey;
use time::precise_time_ns;
use std::str;
use std::result::Result;
use std::io::Read;

use client::response::ProviderResponse;
use client::headers::default_headers;
use client::error::ProviderError;
use notification::Notification;

static DEVELOPMENT: &'static str = "api.development.push.apple.com";
static PRODUCTION: &'static str = "api.push.apple.com";

pub struct CertificateClient {
    pub client: Client,
}

impl CertificateClient {
    pub fn new<'a, R: Read>(sandbox: bool, certificate: &mut R, private_key: &mut R)
                            -> Result<CertificateClient, ProviderError<'a>> {
        let host    = if sandbox { DEVELOPMENT } else { PRODUCTION };
        let mut ctx = SslContext::new(SslMethod::Sslv23).unwrap();

        let x509 = X509::from_pem(certificate)?;
        let pkey = PKey::private_key_from_pem(private_key)?;

        ctx.set_cipher_list("DEFAULT")?;
        ctx.set_certificate(&x509)?;
        ctx.set_private_key(&pkey)?;
        ctx.set_verify(SSL_VERIFY_NONE, None);
        ctx.set_alpn_protocols(&[b"h2"]);

        let connector = TlsConnector::with_context(host, &ctx);

        let client = match Client::with_connector(connector) {
            Ok(client) => client,
            Err(_) => return Err(ProviderError::ClientConnectError("Couldn't connect to APNs service"))
        };

        Ok(CertificateClient {
            client: client,
        })
    }

    pub fn push<'a>(&self, notification: Notification) -> ProviderResponse {
        let path = format!("/3/device/{}", notification.device_token).into_bytes();
        let body = notification.payload.to_string().into_bytes();
        let headers = default_headers(&notification);
        let request = self.client.post(&path, headers.as_slice(), body);

        ProviderResponse::new(request, precise_time_ns())
    }

}

