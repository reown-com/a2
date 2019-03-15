use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, Payload, APS};
use std::collections::BTreeMap;

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{NotificationBuilder, PlainNotificationBuilder};
/// # fn main() {
/// let mut builder = PlainNotificationBuilder::new("Hi there");
/// builder.set_badge(420);
/// builder.set_category("cat1");
/// builder.set_sound("prööt");
/// let payload = builder.build("device_id", Default::default())
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct PlainNotificationBuilder<'a> {
    body: &'a str,
    badge: Option<u32>,
    sound: Option<&'a str>,
    category: Option<&'a str>,
}

impl<'a> PlainNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{PlainNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let payload = PlainNotificationBuilder::new("a body")
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":\"a body\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new(body: &'a str) -> PlainNotificationBuilder<'a>
    {
        PlainNotificationBuilder {
            body: body,
            badge: None,
            sound: None,
            category: None,
        }
    }

    /// A number to show on a badge on top of the app icon.
    ///
    /// ```rust
    /// # use a2::request::notification::{PlainNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let mut builder = PlainNotificationBuilder::new("a body");
    /// builder.set_badge(4);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":\"a body\",\"badge\":4}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_badge(&mut self, badge: u32) -> &mut Self
    {
        self.badge = Some(badge);
        self
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{PlainNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let mut builder = PlainNotificationBuilder::new("a body");
    /// builder.set_sound("meow");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":\"a body\",\"sound\":\"meow\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(&mut self, sound: &'a str) -> &mut Self
    {
        self.sound = Some(sound.into());
        self
    }

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    ///
    /// ```rust
    /// # use a2::request::notification::{PlainNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let mut builder = PlainNotificationBuilder::new("a body");
    /// builder.set_category("cat1");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":\"a body\",\"category\":\"cat1\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_category(&mut self, category: &'a str) -> &mut Self
    {
        self.category = Some(category);
        self
    }
}

impl<'a> NotificationBuilder<'a> for PlainNotificationBuilder<'a> {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a>
    {
        Payload {
            aps: APS {
                alert: Some(APSAlert::Plain(self.body)),
                badge: self.badge,
                sound: self.sound,
                content_available: None,
                category: self.category,
                mutable_content: None,
            },
            device_token: device_token,
            options: options,
            data: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_notification_with_text_only() {
        let payload = PlainNotificationBuilder::new("kulli")
            .build("device-token", Default::default())
            .to_json_string()
            .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": "kulli",
            }
        }).to_string();

        assert_eq!(expected_payload, payload);
    }

    #[test]
    fn test_plain_notification_with_full_data() {
        let mut builder = PlainNotificationBuilder::new("Hi there");
        builder.set_badge(420);
        builder.set_category("cat1");
        builder.set_sound("prööt");

        let payload = builder
            .build("device-token", Default::default())
            .to_json_string()
            .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": "Hi there",
                "badge": 420,
                "category": "cat1",
                "sound": "prööt"
            }
        }).to_string();

        assert_eq!(expected_payload, payload);
    }

    #[test]
    fn test_plain_notification_with_custom_data() {
        #[derive(Serialize, Debug)]
        struct SubData {
            nothing: &'static str,
        }

        #[derive(Serialize, Debug)]
        struct TestData {
            key_str: &'static str,
            key_num: u32,
            key_bool: bool,
            key_struct: SubData,
        }

        let test_data = TestData {
            key_str: "foo",
            key_num: 42,
            key_bool: false,
            key_struct: SubData { nothing: "here" },
        };

        let mut payload =
            PlainNotificationBuilder::new("kulli").build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let payload_json = payload.to_json_string().unwrap();

        let expected_payload = json!({
            "custom": {
                "key_str": "foo",
                "key_num": 42,
                "key_bool": false,
                "key_struct": {
                    "nothing": "here"
                }
            },
            "aps": {
                "alert": "kulli",
            }
        }).to_string();

        assert_eq!(expected_payload, payload_json);
    }
}
