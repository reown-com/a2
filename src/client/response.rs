use rustc_serialize::json::{Json, Object};
use std::str;
use time::{Tm, Timespec, at};
use solicit::http::{Header, Response as HttpResponse};
use std::time::{Duration, Instant};
use std::error::Error;
use std::fmt;
use std::thread;
use std::sync::mpsc::Receiver;

use self::APNSError::*;

// The APNS reasons.
pub enum APNSError {
    /// The message payload was empty.
    PayloadEmpty,

    /// The message payload was too large. The maximum payload size is 4096
    /// bytes.
    PayloadTooLarge,

    /// The apns-topic was invalid.
    BadTopic,

    /// Pushing to this topic is not allowed.
    TopicDisallowed,

    /// The apns-id value is bad.
    BadMessageId,

    /// The apns-expiration value is bad.
    BadExpirationDate,

    /// The apns-priority value is bad.
    BadPriority,

    /// The device token is not specified in the request `path`. Verify that the
    /// `path` header contains the device token.
    MissingDeviceToken,

    /// The specified device token was bad. Verify that the request contains a
    /// valid token and that the token matches the environment.
    BadDeviceToken,

    /// The device token does not match the specified topic.
    DeviceTokenNotForTopic,

    /// The device token is inactive for the specified topic.
    Unregistered,

    /// One or more headers were repeated.
    DuplicateHeaders,

    /// The client certificate was for the wrong environment.
    BadCertificateEnvironment,

    /// The certificate was bad.
    BadCertificate,

    /// The specified action is not allowed.
    Forbidden,

    /// The provider token is not valid or the token signature could not be
    /// verified.
    InvalidProviderToken,

    /// No provider certificate was used to connect to APNs and Authorization
    /// header was missing or no provider token was specified.
    MissingProviderToken,

    /// The provider token is stale and a new token should be generated.
    ExpiredProviderToken,

    /// The request contained a bad `path` value.
    BadPath,

    /// The specified `method` was not `POST`.
    MethodNotAllowed,

    /// Too many requests were made consecutively to the same device token.
    TooManyRequests,

    /// Idle timeout.
    IdleTimeout,

    /// The server is shutting down.
    Shutdown,

    /// An internal server error occurred.
    InternalServerError,

    /// The service is unavailable.
    ServiceUnavailable,

    /// The apns-topic header of the request was not specified and was required.
    /// The apns-topic header is mandatory when the client is connected using a
    /// certificate that supports multiple topics.
    MissingTopic,
}

impl fmt::Debug for APNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}


impl fmt::Display for APNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for APNSError {
    fn description(&self) -> &str {
        match *self {
            PayloadEmpty => "The message payload was empty",
            PayloadTooLarge => {
                "The message payload was too large. \
                The maximum payload size is 4096 bytes"
            }
            BadTopic => "The apns-topic was invalid",
            TopicDisallowed => "Pushing to this topic is not allowed",
            BadMessageId => "The apns-id value is bad",
            BadExpirationDate => "The apns-expiration value is bad",
            BadPriority => "The apns-priority value is bad",
            MissingDeviceToken => {
                "The device token is not specified in the request :path. Verify that the :path \
                 header contains the device token"
            }
            BadDeviceToken => {
                "The specified device token was bad. Verify that the request contains a valid \
                 token and that the token matches the environment"
            }
            DeviceTokenNotForTopic => "The device token does not match the specified topic",
            Unregistered => "The device token is inactive for the specified topic",
            DuplicateHeaders => "One or more headers were repeated",
            BadCertificateEnvironment => "The client certificate was for the wrong environment",
            BadCertificate => "The certificate was bad",
            Forbidden => "The specified action is not allowed",
            InvalidProviderToken => "The provider token is not valid or the token signature could not be verified",
            MissingProviderToken => "No provider certificate was used to connect to APNs and Authorization header was missing or no provider token was specified",
            ExpiredProviderToken => "The provider token is stale and a new token should be generated",
            BadPath => "The request contained a bad :path value",
            MethodNotAllowed => "The specified :method was not POST",
            TooManyRequests => "Too many requests were made consecutively to the same device token",
            IdleTimeout => "Idle time out",
            Shutdown => "The server is shutting down",
            InternalServerError => "An internal server error occurred",
            ServiceUnavailable => "The service is unavailable",
            MissingTopic => {
                "The apns-topic header of the request was not specified and was required. The \
                 apns-topic header is mandatory when the client is connected using a certificate \
                 that supports multiple topics"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}

// The HTTP status code.
#[derive(Debug, PartialEq)]
pub enum APNSStatus {
    /// Success
    Success = 200,

    /// Bad request
    BadRequest = 400,

    /// There was an error with the certificate.
    Forbidden = 403,

    /// The request used a bad method value. Only POST requests are support
    MethodNotAllowed = 405,

    /// The device token is no longer active for the topic.
    Unregistered = 410,

    /// The notification payload was too large.
    PayloadTooLarge = 413,

    /// The server received too many requests for the same device token.
    TooManyRequests = 429,

    /// Internal server error
    InternalServerError = 500,

    /// The server is shutting down and unavailable.
    ServiceUnavailable = 503,

    /// The response channel died before getting a response
    MissingChannel = 997,

    /// The request timed out
    Timeout = 998,

    /// Unknown error
    Unknown = 999,
}

#[derive(Debug)]
pub struct Response {
    /// Status codes for a response
    pub status: APNSStatus,

    /// The apns-id value from the request.
    /// If no value was included in the request,
    /// the server creates a new UUID and returns it in this header.
    pub apns_id: Option<String>,

    /// The error indicating the reason for the failure.
    pub reason: Option<APNSError>,

    /// If the value in the :status header is 410,the value of this key is the last time
    /// at which APNs confirmed that the device token was no longer valid for the topic.
    /// Stop pushing notifications until the device registers a token with
    /// a later timestamp with your provider.
    pub timestamp: Option<Tm>,
}


pub type ResponseChannel = Receiver<HttpResponse<'static, 'static>>;

pub struct ProviderResponse {
    rx: Option<ResponseChannel>,
    pub requested_at: u64,
}

impl ProviderResponse {
    pub fn new(rx: Option<ResponseChannel>, requested_at: u64) -> ProviderResponse {
        ProviderResponse { rx: rx, requested_at: requested_at }
    }

    /// Blocks until having a response from APNS or the timeout is due.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Response, Response> {
        if let Some(ref rx) = self.rx {
            let now = Instant::now();

            while now.elapsed() < timeout {
                match rx.try_recv() {
                    Ok(http_response) => {
                        let status        = Self::fetch_status(http_response.status_code().ok());
                        let apns_id       = Self::fetch_apns_id(http_response.headers);
                        let json          = str::from_utf8(&http_response.body).ok().and_then(|v| Json::from_str(v).ok());
                        let object        = json.as_ref().and_then(|v| v.as_object());
                        let reason        = Self::fetch_reason(object);
                        let timestamp     = Self::fetch_timestamp(object);

                        let response = Response {
                            status: status,
                            reason: reason,
                            timestamp: timestamp,
                            apns_id: apns_id,
                        };

                        if response.status == APNSStatus::Success {
                            return Ok(response);
                        } else {
                            return Err(response);
                        }
                    },
                    _ => thread::park_timeout(Duration::from_millis(10)),
                }
            }

            Err(Response {
                status: APNSStatus::Timeout,
                reason: None,
                timestamp: None,
                apns_id: None,
            })
        } else {
            Err(Response {
                status: APNSStatus::MissingChannel,
                reason: None,
                timestamp: None,
                apns_id: None,
            })
        }
    }

    fn fetch_status(code: Option<u16>) -> APNSStatus {
        match code {
            Some(200) => APNSStatus::Success,
            Some(400) => APNSStatus::BadRequest,
            Some(403) => APNSStatus::Forbidden,
            Some(405) => APNSStatus::MethodNotAllowed,
            Some(410) => APNSStatus::Unregistered,
            Some(413) => APNSStatus::PayloadTooLarge,
            Some(429) => APNSStatus::TooManyRequests,
            Some(500) => APNSStatus::InternalServerError,
            Some(503) => APNSStatus::ServiceUnavailable,
            _         => APNSStatus::Unknown,
        }
    }

    fn fetch_apns_id(headers: Vec<Header>) -> Option<String> {
        headers.iter().find(|&header| {
            match str::from_utf8(header.name()).unwrap() {
                "apns-id" => true,
                _         => false,
            }
        }).map(|header| {
            String::from_utf8(header.value().to_vec()).unwrap()
        })
    }

    fn fetch_reason(js_object: Option<&Object>) -> Option<APNSError> {
        let raw_reason = js_object.and_then(|v| v.get("reason")).and_then(|v| v.as_string());

        match raw_reason {
            Some("PayloadEmpty")              => Some(APNSError::PayloadEmpty),
            Some("PayloadTooLarge")           => Some(APNSError::PayloadTooLarge),
            Some("BadTopic")                  => Some(APNSError::BadTopic),
            Some("TopicDisallowed")           => Some(APNSError::TopicDisallowed),
            Some("BadMessageId")              => Some(APNSError::BadMessageId),
            Some("BadExpirationDate")         => Some(APNSError::BadExpirationDate),
            Some("BadPriority")               => Some(APNSError::BadPriority),
            Some("MissingDeviceToken")        => Some(APNSError::MissingDeviceToken),
            Some("BadDeviceToken")            => Some(APNSError::BadDeviceToken),
            Some("DeviceTokenNotForTopic")    => Some(APNSError::DeviceTokenNotForTopic),
            Some("Unresgistered")             => Some(APNSError::Unregistered),
            Some("DuplicateHeaders")          => Some(APNSError::DuplicateHeaders),
            Some("BadCertificateEnvironment") => Some(APNSError::BadCertificateEnvironment),
            Some("BadCertificate")            => Some(APNSError::BadCertificate),
            Some("Forbidden")                 => Some(APNSError::Forbidden),
            Some("InvalidProviderToken")      => Some(APNSError::InvalidProviderToken),
            Some("MissingProviderToken")      => Some(APNSError::MissingProviderToken),
            Some("ExpiredProviderToken")      => Some(APNSError::ExpiredProviderToken),
            Some("BadPath")                   => Some(APNSError::BadPath),
            Some("MethodNotAllowed")          => Some(APNSError::MethodNotAllowed),
            Some("TooManyRequests")           => Some(APNSError::TooManyRequests),
            Some("IdleTimeout")               => Some(APNSError::IdleTimeout),
            Some("Shutdown")                  => Some(APNSError::Shutdown),
            Some("InternalServerError")       => Some(APNSError::InternalServerError),
            Some("ServiceUnavailable")        => Some(APNSError::ServiceUnavailable),
            Some("MissingTopic")              => Some(APNSError::MissingTopic),
            _                                 => None,
        }
    }

    fn fetch_timestamp(js_object: Option<&Object>) -> Option<Tm> {
        let raw_ts = js_object.and_then(|v| v.get("timestamp")).and_then(|v| v.as_i64());

        match raw_ts {
            Some(ts) => Some(at(Timespec::new(ts, 0))),
            None     => None,
        }
    }
}
