use std::error::Error;
use std::fmt;
use time::Tm;

use self::APNSError::*;
// The APNS reasons.
pub enum APNSError {
    PayloadEmpty,
    PayloadTooLarge,
    BadTopic,
    TopicDisallowed,
    BadMessageId,
    BadExpirationDate,
    BadPriority,
    MissingDeviceToken,
    BadDeviceToken,
    DeviceTokenNotForTopic,
    Unregistered,
    DuplicateHeaders,
    BadCertificateEnvironment,
    BadCertificate,
    Forbidden,
    BadPath,
    MethodNotAllowed,
    TooManyRequests,
    IdleTimeout,
    Shutdown,
    InternalServerError,
    ServiceUnavailable,
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
            PayloadTooLarge => "The message payload was too large. The maximum payload size is 4096 bytes",
            BadTopic => "The apns-topic was invalid",
            TopicDisallowed => "Pushing to this topic is not allowed",
            BadMessageId => "The apns-id value is bad",
            BadExpirationDate => "The apns-expiration value is bad",
            BadPriority => "The apns-priority value is bad",
            MissingDeviceToken => "The device token is not specified in the request :path. Verify that the :path header contains the device token",
            BadDeviceToken => "The specified device token was bad. Verify that the request contains a valid token and that the token matches the environment",
            DeviceTokenNotForTopic => "The device token does not match the specified topic",
            Unregistered => "The device token is inactive for the specified topic",
            DuplicateHeaders => "One or more headers were repeated",
            BadCertificateEnvironment => "The client certificate was for the wrong environment",
            BadCertificate => "The certificate was bad",
            Forbidden => "The specified action is not allowed",
            BadPath => "The request contained a bad :path value",
            MethodNotAllowed => "The specified :method was not POST",
            TooManyRequests => "Too many requests were made consecutively to the same device token",
            IdleTimeout => "Idle time out",
            Shutdown => "The server is shutting down",
            InternalServerError => "An internal server error occurred",
            ServiceUnavailable => "The service is unavailable",
            MissingTopic => "The apns-topic header of the request was not specified and was required. The apns-topic header is mandatory when the client is connected using a certificate that supports multiple topics",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}

// The HTTP status code.
pub enum APNSStatus {
    Success = 200,              // Success
    BadRequest = 400,           // Bad request
    Forbidden = 403,            // There was an error with the certificate.
    MethodNotAllowed = 405,     // The request used a bad :method value. Only POST requests are supported.
    Unregistered = 410,         // The device token is no longer active for the topic.
    PayloadTooLarge = 413,      // The notification payload was too large.
    TooManyRequests = 429,      // The server received too many requests for the same device token.
    InternalServerError = 500,  // Internal server error
    ServiceUnavailable = 503,   // The server is shutting down and unavailable.
}

pub struct Response {
    // Status codes for a response
    pub status: APNSStatus,

    // The apns-id value from the request.
    // If no value was included in the request, the server creates a new UUID and returns it in this header.
    pub apns_id: Option<String>,

    // The error indicating the reason for the failure.
    pub reason: Option<APNSError>,

    // If the value in the :status header is 410, the value of this key is the last time at which APNs confirmed that the device token was no longer valid for the topic.
    // Stop pushing notifications until the device registers a token with a later timestamp with your provider.
    pub timestamp: Option<Tm>,
}
