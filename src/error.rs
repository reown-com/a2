//! Error and result module

use crate::client::FutureResponse;
use tokio_timer::TimeoutError;
use std::error::Error as StdError;
use std::io::Error as IoError;
use serde_json::Error as SerdeError;
use openssl::error::ErrorStack;
use std::fmt;
use std::convert::From;
use crate::response::{Response, ErrorBody};

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

impl<'a> fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ResponseError(Response { error: Some(ErrorBody { ref reason, .. }), .. }) => {
                write!(fmt, "{} (reason: {:?})", self.description(), reason)
            },
            _ => write!(fmt, "{}", self.description()),
        }
    }
}

impl<'a> StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::SerializeError => "Error serializing to JSON",
            Error::ConnectionError => "Error connecting to APNs",
            Error::SignerError(_) => "Error creating a signature",
            Error::ResponseError(_) => "Notification was not accepted by APNs",
            Error::InvalidOptions(_) => "Invalid options for APNs payload",
            Error::TlsError(_) => "Error in creating a TLS connection",
            Error::ReadError(_) => "Error in reading a certificate file",
            Error::TimeoutError => "Timeout in sending a push notification",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}

impl From<TimeoutError<FutureResponse>> for Error {
    fn from(_: TimeoutError<FutureResponse>) -> Error {
        Error::TimeoutError
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
