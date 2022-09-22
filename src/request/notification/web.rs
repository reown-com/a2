use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, Payload, APS};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct WebPushAlert<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub action: &'a str,
}

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{NotificationBuilder, WebNotificationBuilder, WebPushAlert};
/// # fn main() {
/// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
/// builder.set_sound("prööt");
/// let payload = builder.build("device_id", Default::default())
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct WebNotificationBuilder<'a> {
    alert: WebPushAlert<'a>,
    sound: Option<&'a str>,
    url_args: &'a [&'a str],
}

impl<'a> WebNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"action\":\"View\",\"body\":\"World\",\"title\":\"Hello\"},\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new(alert: WebPushAlert<'a>, url_args: &'a [&'a str]) -> WebNotificationBuilder<'a> {
        WebNotificationBuilder {
            alert,
            sound: None,
            url_args,
        }
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_sound("meow");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"action\":\"View\",\"body\":\"World\",\"title\":\"Hello\"},\"sound\":\"meow\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(&mut self, sound: &'a str) -> &mut Self {
        self.sound = Some(sound);
        self
    }
}

impl<'a> NotificationBuilder<'a> for WebNotificationBuilder<'a> {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a> {
        Payload {
            aps: APS {
                alert: Some(APSAlert::WebPush(self.alert)),
                badge: None,
                sound: self.sound,
                content_available: None,
                category: None,
                mutable_content: None,
                url_args: Some(self.url_args),
            },
            device_token,
            options,
            data: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webpush_notification() {
        let payload = WebNotificationBuilder::new(
            WebPushAlert {
                action: "View",
                title: "Hello",
                body: "world",
            },
            &["arg1"],
        )
        .build("device-token", Default::default())
        .to_json_string()
        .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "body": "world",
                    "action": "View",
                    "title": "Hello"
                },
                "url-args": ["arg1"]
            }
        })
        .to_string();

        assert_eq!(expected_payload, payload);
    }
}
