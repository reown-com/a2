//! The `aps` notification content builders

mod localized;
mod options;
mod plain;
mod silent;

pub use self::localized::{LocalizedAlert, LocalizedNotificationBuilder};
pub use self::options::{CollapseId, NotificationOptions, Priority};
pub use self::plain::PlainNotificationBuilder;
pub use self::silent::SilentNotificationBuilder;

use crate::request::payload::Payload;

pub trait NotificationBuilder<'a> {
    /// Generates the request payload to be send with the `Client`.
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a>;
}
