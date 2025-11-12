use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, APSSound, Payload, APS};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct WebPushAlert {
    pub title: String,
    pub body: String,
    pub action: String,
}

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{NotificationBuilder, WebNotificationBuilder, WebPushAlert};
/// # use a2::request::payload::PayloadLike;
/// # fn main() {
/// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".to_owned(), body: "World".to_owned(), action: "View".to_owned()}, vec!["arg1".to_owned()]);
/// builder.set_sound("prööt".to_owned());
/// let payload = builder.build("device_id".to_owned(), Default::default())
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct WebNotificationBuilder {
    alert: WebPushAlert,
    sound: Option<String>,
    url_args: Vec<String>,
}

impl WebNotificationBuilder {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".to_owned(), body: "World".to_owned(), action: "View".to_owned()}, vec!["arg1".to_owned()]);
    /// let payload = builder.build("token".to_owned(), Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new(alert: WebPushAlert, url_args: Vec<String>) -> WebNotificationBuilder {
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
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".to_owned(), body: "World".to_owned(), action: "View".to_owned()}, vec!["arg1".to_owned()]);
    /// builder.set_sound("meow".to_owned());
    /// let payload = builder.build("token".to_owned(), Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"sound\":\"meow\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(&mut self, sound: impl Into<String>) -> &mut Self {
        self.sound = Some(sound.into());
        self
    }
}

impl NotificationBuilder for WebNotificationBuilder {
    fn build(self, device_token: impl Into<String>, options: NotificationOptions) -> Payload {
        Payload {
            aps: APS {
                alert: Some(APSAlert::WebPush(self.alert)),
                badge: None,
                sound: self.sound.map(APSSound::Sound),
                content_available: None,
                category: None,
                mutable_content: None,
                url_args: Some(self.url_args),
            },
            device_token: device_token.into(),
            options,
            data: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::payload::PayloadLike;
    use serde_json::Value;

    #[test]
    fn test_webpush_notification() {
        let payload = WebNotificationBuilder::new(
            WebPushAlert {
                action: "View".to_string(),
                title: "Hello".to_string(),
                body: "world".to_string(),
            },
            vec!["arg1".to_string()],
        )
        .build("device-token".to_string(), Default::default())
        .to_json_string()
        .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "title": "Hello",
                    "body": "world",
                    "action": "View",
                },
                "url-args": ["arg1"]
            }
        });

        assert_eq!(expected_payload, serde_json::from_str::<Value>(&payload).unwrap());
    }
}
