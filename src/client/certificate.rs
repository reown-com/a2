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
use client::{DEVELOPMENT, PRODUCTION};

pub struct CertificateClient {
    pub client: Client,
}

/// Creates a new connection to APNs using a certificate and private key to an
/// application. The connection is only valid for one application.
///
/// The response for `push` is asynchorous for better throughput.
///
/// # Example
/// ```no_run
/// # extern crate apns2;
/// # fn main() {
/// use apns2::client::CertificateClient;
/// use apns2::payload::{Payload, APSAlert};
/// use apns2::notification::{Notification, NotificationOptions};
/// use std::fs::File;
/// use std::time::Duration;
///
/// // Can be anything that implements the `Read` trait.
/// let mut cert_file = File::open("/path/to/certificate.pem").unwrap();
/// let mut key_file = File::open("/path/to/key.pem").unwrap();
///
/// let client = CertificateClient::new(false, &mut cert_file, &mut key_file).unwrap();
/// let alert = APSAlert::Plain(String::from("Hi there!"));
/// let payload = Payload::new(alert, "default", Some(1u32), None, None);
/// let options = NotificationOptions { ..Default::default() };
/// let request = client.push(Notification::new(payload, "apple_device_token", options));
///
/// // Block here to get the response.
/// let response = request.recv_timeout(Duration::from_millis(2000));
///
/// println!("{:?}", response);
/// # }
/// ```
impl CertificateClient {
    /// Create a new connection to APNs with custom certificate and key. Can be
    /// used to send notification to only one app.
    pub fn new<'a, R: Read>(sandbox: bool, certificate: &mut R, private_key: &mut R)
                            -> Result<CertificateClient, ProviderError> {
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
        let client = Client::with_connector(connector)?;

        Ok(CertificateClient {
            client: client,
        })
    }

    /// Send a notification.
    pub fn push<'a>(&self, notification: Notification) -> ProviderResponse {
        let path = format!("/3/device/{}", notification.device_token).into_bytes();
        let body = notification.payload.to_string().into_bytes();
        let headers = default_headers(&notification);
        let request = self.client.post(&path, headers.as_slice(), body);

        ProviderResponse::new(request, precise_time_ns())
    }
}
