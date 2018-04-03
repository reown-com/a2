use error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CollapseId {
    pub value: String,
}

/// A collapse-id container. Will not allow bigger id's than 64 bytes.
impl CollapseId {
    pub fn new<S: Into<String>>(value: S) -> Result<CollapseId, Error> {
        let s = value.into();
        if s.len() > 64 {
            Err(Error::InvalidOptions(String::from(
                "The collapse-id is too big. Maximum 64 bytes.",
            )))
        } else {
            Ok(CollapseId { value: s })
        }
    }
}

/// Headers to specify options to the notification.
#[derive(Debug, Clone)]
pub struct NotificationOptions {
    /// A canonical UUID that identifies the notification. If there is an error
    /// sending the notification, APNs uses this value to identify the
    /// notification to your server.
    pub apns_id: Option<String>,

    /// A UNIX epoch date expressed in seconds (UTC). This header identifies the
    /// date when the notification is no longer valid and can be discarded.
    ///
    /// If this value is nonzero, APNs stores the notification and tries to
    /// deliver it at least once, repeating the attempt as needed if it is unable
    /// to deliver the notification the first time. If the value is 0, APNs
    /// treats the notification as if it expires immediately and does not store
    /// the notification or attempt to redeliver it.
    pub apns_expiration: Option<u64>,

    /// The priority of the notification.
    pub apns_priority: Priority,

    /// The topic of the remote notification, which is typically the bundle ID
    /// for your app. The certificate you create in your developer account must
    /// include the capability for this topic.
    ///
    /// If your certificate includes multiple topics, you must specify a value
    /// for this header.
    ///
    /// If you omit this request header and your APNs certificate does not
    /// specify multiple topics, the APNs server uses the certificate’s Subject
    /// as the default topic.
    ///
    /// If you are using a provider token instead of a certificate, you must
    /// specify a value for this request header. The topic you provide should be
    /// provisioned for the your team named in your developer account.
    pub apns_topic: Option<String>,

    /// Multiple notifications with the same collapse identifier are displayed to the
    /// user as a single notification. The value of this key must not exceed 64
    /// bytes.
    pub apns_collapse_id: Option<CollapseId>,
}

impl Default for NotificationOptions {
    fn default() -> NotificationOptions {
        NotificationOptions {
            apns_id: None,
            apns_expiration: None,
            apns_priority: Priority::Normal,
            apns_topic: None,
            apns_collapse_id: None,
        }
    }
}

/// The importance how fast to bring the notification for the user..
#[derive(Debug, Clone)]
pub enum Priority {
    /// Send the push message immediately. Notifications with this priority must
    /// trigger an alert, sound, or badge on the target device. Cannot be used
    /// with the silent notification.
    High,

    /// Send the push message at a time that takes into account power
    /// considerations for the device. Notifications with this priority might be
    /// grouped and delivered in bursts. They are throttled, and in some cases
    /// are not delivered.
    Normal,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let priority = match self {
            &Priority::High => "10",
            &Priority::Normal => "5",
        };

        write!(f, "{}", priority)
    }
}
