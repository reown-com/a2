//! A helper struct for generating an APNS request.

use payload::*;

/// The Remote Notification.
pub struct Notification<'a> {
    /// The Remote Notification Payload.
    pub payload: Payload,

    /// Specify the hexadecimal string of the device token for the target
    /// device.
    pub device_token: &'a str,

    /// The optional settings for the notification
    pub options: NotificationOptions<'a>,
}

impl<'a> Notification<'a> {
    pub fn new(payload: Payload, token: &'a str, options: NotificationOptions<'a>) -> Notification<'a> {
        Notification {
            payload: payload,
            device_token: token,
            options: options,
        }
    }
}

/// Request headers.
pub struct NotificationOptions<'a> {
    /// A canonical UUID that identifies the notification.
    pub apns_id: Option<&'a str>,

    /// A UNIX epoch date expressed in seconds (UTC).
    pub apns_expiration: Option<i64>,

    /// The priority of the notification.
    pub apns_priority: Option<u32>,

    /// The topic of the remote notification, which is typically the bundle ID
    /// for your app.
    pub apns_topic: Option<&'a str>,
}

impl<'a> Default for NotificationOptions<'a> {
    fn default() -> NotificationOptions<'a> {
        NotificationOptions {
            apns_id: None,
            apns_expiration: None,
            apns_priority: None,
            apns_topic: None,
        }
    }
}
