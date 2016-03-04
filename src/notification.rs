use ::payload::*;
use ::device_token::*;

pub struct Notification {
    // The Remote Notification Payload.
    pub payload: Payload,

    // Specify the hexadecimal string of the device token for the target device.
    pub device_token: DeviceToken,

    // A canonical UUID that identifies the notification.
    pub apns_id: Option<String>,

    // A UNIX epoch date expressed in seconds (UTC).
    pub apns_expiration: Option<String>,

    // The priority of the notification.
    pub apns_priority: Option<u32>,

    // The length of the body content.
    pub content_length: Option<u32>,

    // The topic of the remote notification, which is typically the bundle ID for your app.
    pub apns_topic: Option<String>,
}

impl Notification {
    pub fn new(payload: Payload, token: DeviceToken) -> Notification {
        Notification {
            payload: payload,
            device_token: token,
            apns_id: None,
            apns_expiration: None,
            apns_priority: None,
            content_length: None,
            apns_topic: None,
        }
    }
}
