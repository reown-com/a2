use rustls::ClientConfig;
use tokio_rustls::ClientConfigExt;
use rustls::internal::pemfile;
use rustls::ClientSession;
use hyper::client::{HttpConnector};
use hyper::client::connect::{Destination, Connected, Connect};
use std::io;
use std::fmt;
use std::sync::Arc;
use webpki::{DNSName, DNSNameRef};
use webpki_roots;
use futures::{Future, Poll};
use tokio_rustls::TlsStream;
use tokio::net::TcpStream;

/// Connector for Application-Layer Protocol Negotiation to form a TLS
/// connection for Hyper.
pub struct AlpnConnector {
    tls: Arc<ClientConfig>,
    http: HttpConnector,
}

type AlpnStream = TlsStream<TcpStream, ClientSession>;

impl AlpnConnector {
    /// Construct a new `AlpnConnector`.
    pub fn new() -> Self {
        Self::with_client_config(ClientConfig::new())
    }

    /// Construct a new `AlpnConnector` with a custom certificate and private
    /// key, which should be in PEM format.
    pub fn with_client_cert(
        cert_pem: &[u8],
        key_pem: &[u8],
    ) -> Result<Self, io::Error> {
        let parsed_keys = pemfile::pkcs8_private_keys(&mut io::BufReader::new(key_pem)).or({
            trace!("AlpnConnector::with_client_cert error reading private key");
            Err(io::Error::new(io::ErrorKind::InvalidData, "private key"))
        })?;

        if let Some(key) = parsed_keys.first() {
            let mut config = ClientConfig::new();
            let parsed_cert = pemfile::certs(&mut io::BufReader::new(cert_pem)).or({
                trace!("AlpnConnector::with_client_cert error reading certificate");
                Err(io::Error::new(io::ErrorKind::InvalidData, "certificate"))
            })?;

            config.set_single_client_cert(parsed_cert, key.clone());

            Ok(Self::with_client_config(config))
        } else {
            trace!("AlpnConnector::with_client_cert no private keys found from the given PEM");
            Err(io::Error::new(io::ErrorKind::InvalidData, "private key"))
        }
    }

    fn with_client_config(mut config: ClientConfig) -> Self {
        config.alpn_protocols.push("h2".to_owned());
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let mut http = HttpConnector::new(4);
        http.enforce_http(false);

        AlpnConnector {
            tls: Arc::new(config),
            http: http,
        }
    }
}

impl fmt::Debug for AlpnConnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AlpnConnector").finish()
    }
}

impl Connect for AlpnConnector {
    type Transport = AlpnStream;
    type Error = io::Error;
    type Future = AlpnConnecting;

    fn connect(&self, dst: Destination) -> Self::Future {
        trace!("AlpnConnector::call ({:?})", dst);

        let host: DNSName = match DNSNameRef::try_from_ascii_str(dst.host()) {
            Ok(host) => host.into(),
            Err(err) => {
                return AlpnConnecting(Box::new(::futures::future::err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid url: {:?}", err),
                ))))
            }
        };

        let tls = self.tls.clone();
        let connecting = self.http
            .connect(dst)
            .and_then(move |(tcp, connected)| {
                trace!("AlpnConnector::call got TCP, trying TLS");
                tls.connect_async(host.as_ref(), tcp)
                    .map_err(|e| {
                        trace!("AlpnConnector::call got error forming a TLS connection.");
                        io::Error::new(io::ErrorKind::Other, e)
                    })
                    .map(move |tls| {
                        (tls, connected)
                    })
            })
            .map_err(|e| {
                trace!("AlpnConnector::call got error reading a TLS stream (#{}).", e);
                io::Error::new(io::ErrorKind::Other, e)
            });

        AlpnConnecting(Box::new(connecting))
    }
}

pub struct AlpnConnecting(Box<Future<Item = (AlpnStream, Connected), Error = io::Error> + Send + 'static>);

impl Future for AlpnConnecting {
    type Item = (AlpnStream, Connected);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl fmt::Debug for AlpnConnecting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("AlpnConnecting")
    }
}
