use request::notification::{NotificationBuilder, NotificationOptions};
use request::payload::{APSAlert, Payload, APS};

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # extern crate a2;
/// # use a2::request::notification::{NotificationOptions, NotificationBuilder, PlainNotificationBuilder};
/// # fn main() {
/// let mut builder = PlainNotificationBuilder::new("Hi there");
/// builder.set_badge(420);
/// builder.set_category("cat1");
/// builder.set_sound("prööt");
/// let options = NotificationOptions::default();
/// let payload = builder.build("device_id", &options)
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct PlainNotificationBuilder {
    body: String,
    badge: Option<u32>,
    sound: Option<String>,
    category: Option<String>,
}

impl PlainNotificationBuilder {
    pub fn new<S>(body: S) -> PlainNotificationBuilder
    where
        S: Into<String>,
    {
        PlainNotificationBuilder {
            body: body.into(),
            badge: None,
            sound: None,
            category: None,
        }
    }

    /// A number to show on a badge on top of the app icon.
    pub fn set_badge(&mut self, badge: u32) {
        self.badge = Some(badge);
    }

    /// File name of the custom sound to play when receiving the notification.
    pub fn set_sound<S>(&mut self, sound: S)
    where
        S: Into<String>,
    {
        self.sound = Some(sound.into());
    }

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    pub fn set_category<S>(&mut self, category: S)
    where
        S: Into<String>,
    {
        self.category = Some(category.into());
    }
}

impl NotificationBuilder for PlainNotificationBuilder {
    fn build<'a, 'b>(
        self,
        device_token: &'a str,
        options: &'b NotificationOptions,
    ) -> Payload<'a, 'b> {
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
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_notification_with_text_only() {
        let options = NotificationOptions::default();
        let payload = PlainNotificationBuilder::new("kulli")
            .build("device-token", &options)
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

        let device_token = "device-token".to_string();

        let options = NotificationOptions::default();
        let payload = builder
            .build(&device_token, &options)
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

        let options = NotificationOptions::default();
        let mut payload =
            PlainNotificationBuilder::new("kulli").build("device-token", &options);

        payload.add_custom_data("custom", &test_data).unwrap();

        let payload_json = payload.to_json_string().unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": "kulli",
            },
            "custom": {
                "key_str": "foo",
                "key_num": 42,
                "key_bool": false,
                "key_struct": {
                    "nothing": "here"
                }
            }
        }).to_string();

        assert_eq!(expected_payload, payload_json);
    }
}
