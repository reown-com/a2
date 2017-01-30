use openssl::ssl::error::SslError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProviderError<'a> {
    ClientConnectError(&'a str),
    SslError(&'a str)
}

impl<'a> From<SslError> for ProviderError<'a> {
    fn from(_: SslError) -> ProviderError<'a> {
        ProviderError::SslError("Error generationg SSL context")
    }
}

impl<'a> Error for ProviderError<'a> {
    fn description(&self) -> &str {
        "Error in APNs connection"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl<'a> fmt::Display for ProviderError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
