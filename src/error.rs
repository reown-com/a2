//! Error and result module

use crate::response::{ErrorBody, Response};
use openssl::error::ErrorStack;
use serde_json::Error as SerdeError;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    /// User request or Apple response JSON data was faulty.
    SerializeError,

    /// A problem connecting to APNs servers.
    ConnectionError,

    /// APNs couldn't response in a timely manner, if using
    /// [send_with_timeout](client/struct.Client.html#method.send_with_timeout)
    TimeoutError,

    /// Couldn't generate an APNs token with the given key.
    SignerError(String),

    /// APNs couldn't accept the notification. Contains
    /// [Response](response/struct.Response.html) with additional
    /// information.
    ResponseError(Response),

    /// Invalid option values given in
    /// [NotificationOptions](request/notification/struct.NotificationOptions.html)
    InvalidOptions(String),

    /// TLS connection failed
    TlsError(String),

    /// Error reading the certificate or private key.
    ReadError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ResponseError(Response {
                error: Some(ErrorBody { ref reason, .. }),
                ..
            }) => write!(
                fmt,
                "Notification was not accepted by APNs (reason: {})",
                reason
            ),
            Error::SerializeError => fmt.write_str("Error serializing to JSON"),
            Error::ConnectionError => fmt.write_str("Error connecting to APNs"),
            Error::SignerError(_) => fmt.write_str("Error creating a signature"),
            Error::ResponseError(_) => fmt.write_str("Notification was not accepted by APNs"),
            Error::InvalidOptions(_) => fmt.write_str("Invalid options for APNs payload"),
            Error::TlsError(_) => fmt.write_str("Error in creating a TLS connection"),
            Error::ReadError(_) => fmt.write_str("Error in reading a certificate file"),
            Error::TimeoutError => fmt.write_str("Timeout in sending a push notification"),
        }
    }
}

impl StdError for Error {}

impl From<SerdeError> for Error {
    fn from(_: SerdeError) -> Error {
        Error::SerializeError
    }
}

impl From<ErrorStack> for Error {
    fn from(e: ErrorStack) -> Error {
        Error::SignerError(format!("{}", e))
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::ReadError(format!("{}", e))
    }
}

impl From<hyper::error::Error> for Error {
    fn from(_: hyper::error::Error) -> Error {
        Error::ConnectionError
    }
}
