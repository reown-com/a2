use solicit::client::ClientConnectError;
use solicit::http::client::tls::TlsConnectError;
use openssl::ssl::error::SslError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProviderError {
    ClientConnectError(String),
    SslError(String)
}

impl From<SslError> for ProviderError {
    fn from(e: SslError) -> ProviderError {
        ProviderError::SslError(format!("Error generating an SSL context: {}", e.description()))
    }
}

impl From<ClientConnectError<TlsConnectError>> for ProviderError {
    fn from(e: ClientConnectError<TlsConnectError>) -> ProviderError {
        ProviderError::ClientConnectError(format!("Error connecting to the APNs servers: {}", e.description()))
    }
}

impl Error for ProviderError {
    fn description(&self) -> &str {
        "APNs connection failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
