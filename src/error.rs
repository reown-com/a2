//! Error and result module

use std::error::Error as StdError;
use std::io::Error as IoError;
use serde_json::Error as SerdeError;
use openssl::error::ErrorStack;
use std::fmt;
use std::convert::From;
use response::Response;

#[derive(Debug)]
pub enum Error {
    SerializeError,
    ConnectionError,
    SignerError(String),
    ResponseError(Response),
    InvalidOptions(String),
    TlsError(String),
    ReadError(String),
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl<'a> StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::SerializeError => "Error serializing to JSON",
            &Error::ConnectionError => "Error connecting to APNs",
            &Error::SignerError(_) => "Error creating a signature",
            &Error::ResponseError(_) => "Notification was not accepted by APNs",
            &Error::InvalidOptions(_) => "Invalid options for APNs payload",
            &Error::TlsError(_) => "Error in creating a TLS connection",
            &Error::ReadError(_) => "Error in reading a certificate file",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl<'a> From<SerdeError> for Error {
    fn from(_: SerdeError) -> Error {
        Error::SerializeError
    }
}

impl<'a> From<ErrorStack> for Error {
    fn from(e: ErrorStack) -> Error {
        Error::SignerError(format!("{}", e.description()))
    }
}

impl<'a> From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::ReadError(format!("{}", e.description()))
    }
}
