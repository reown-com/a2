use rustls::ClientConfig;
use tokio_rustls::ClientConfigExt;
use rustls::internal::pemfile;
use tokio_core::reactor::Handle;
use tokio_service::Service;
use hyper::Uri;
use hyper::client::HttpConnector;
use std::io;
use std::fmt;
use std::sync::Arc;
use webpki::{DNSName, DNSNameRef};
use webpki_roots;
use stream::MaybeHttpsStream;
use futures::{Future, Poll};

/// Connector for Application-Layer Protocol Negotiation to form a TLS
/// connection for Hyper.
pub struct AlpnConnector {
    tls: Arc<ClientConfig>,
    http: HttpConnector,
}

impl AlpnConnector {
    /// Construct a new `AlpnConnector`.
    pub fn new(handle: &Handle) -> Self {
        Self::with_client_config(handle, ClientConfig::new())
    }

    /// Construct a new `AlpnConnector` with a custom certificate and private
    /// key, which should be in PEM format.
    pub fn with_client_cert(
        cert_pem: &[u8],
        key_pem: &[u8],
        handle: &Handle,
    ) -> Result<Self, io::Error> {
        let parsed_keys = pemfile::pkcs8_private_keys(&mut io::BufReader::new(key_pem)).or(Err(
            io::Error::new(io::ErrorKind::InvalidData, "private key"),
        ))?;

        if let Some(key) = parsed_keys.first() {
            let mut config = ClientConfig::new();
            let parsed_cert = pemfile::certs(&mut io::BufReader::new(cert_pem)).or(Err(
                io::Error::new(io::ErrorKind::InvalidData, "certificate"),
            ))?;

            config.set_single_client_cert(parsed_cert, key.clone());

            Ok(Self::with_client_config(handle, config))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "private key"))
        }
    }

    fn with_client_config(handle: &Handle, mut config: ClientConfig) -> Self {
        config.alpn_protocols.push("h2".to_owned());
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let mut http = HttpConnector::new(4, handle);
        http.enforce_http(false);

        AlpnConnector {
            tls: Arc::new(config),
            http: http,
        }
    }
}

impl fmt::Debug for AlpnConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AlpnConnectorr").finish()
    }
}

impl Service for AlpnConnector {
    type Request = Uri;
    type Response = MaybeHttpsStream;
    type Error = io::Error;
    type Future = HttpsConnecting;

    fn call(&self, uri: Uri) -> Self::Future {
        trace!("AlpnConnector::call ({:?})", uri);
        let host: DNSName = match uri.host() {
            Some(host) => match DNSNameRef::try_from_ascii_str(host) {
                Ok(host) => host.into(),
                Err(err) => {
                    return HttpsConnecting(Box::new(::futures::future::err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("invalid url: {:?}", err),
                    ))))
                }
            },
            None => {
                return HttpsConnecting(Box::new(::futures::future::err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid url, missing host",
                ))))
            }
        };

        let tls = self.tls.clone();
        let connecting = self.http
            .call(uri)
            .and_then(move |tcp| {
                trace!("AlpnConnector::call got TCP, trying TLS");
                tls.connect_async(host.as_ref(), tcp)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            })
            .map(|tls| MaybeHttpsStream::Https(tls))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));

        HttpsConnecting(Box::new(connecting))
    }
}

pub struct HttpsConnecting(Box<Future<Item = MaybeHttpsStream, Error = io::Error>>);

impl Future for HttpsConnecting {
    type Item = MaybeHttpsStream;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl fmt::Debug for HttpsConnecting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("HttpsConnecting")
    }
}
