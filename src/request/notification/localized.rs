use request::notification::{NotificationBuilder, NotificationOptions};
use request::payload::{APSAlert, Payload, APS};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct LocalizedAlert {
    title: String,
    body: String,

    #[serde(skip_serializing_if = "Option::is_none")] title_loc_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] title_loc_args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")] action_loc_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] loc_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")] loc_args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")] launch_image: Option<String>,
}

/// A builder to create a localized APNs payload.
///
/// # Example
///
/// ```rust
/// # extern crate apns2;
/// # use apns2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
/// # fn main() {
/// let mut builder = LocalizedNotificationBuilder::new("Hi there", "What's up?");
/// builder.set_badge(420);
/// builder.set_category("cat1");
/// builder.set_sound("prööt");
/// builder.set_mutable_content();
/// builder.set_action_loc_key("PLAY");
/// builder.set_launch_image("foo.jpg");
/// builder.set_loc_args(vec!["argh", "narf"]);
/// builder.set_title_loc_key("STOP");
/// builder.set_title_loc_args(vec!["herp", "derp"]);
/// builder.set_loc_key("PAUSE");
/// builder.set_loc_args(vec!["narf", "derp"]);
/// let payload = builder.build("device_id", Default::default())
///   .to_json_string().unwrap();
/// # }
/// ```
pub struct LocalizedNotificationBuilder {
    alert: LocalizedAlert,
    badge: Option<u32>,
    sound: Option<String>,
    category: Option<String>,
    mutable_content: u8,
}

impl LocalizedNotificationBuilder {
    pub fn new<S>(title: S, body: S) -> LocalizedNotificationBuilder
    where
        S: Into<String>,
    {
        LocalizedNotificationBuilder {
            alert: LocalizedAlert {
                title: title.into(),
                body: body.into(),
                title_loc_key: None,
                title_loc_args: None,
                action_loc_key: None,
                loc_key: None,
                loc_args: None,
                launch_image: None,
            },
            badge: None,
            sound: None,
            category: None,
            mutable_content: 0,
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

    /// The localization key for the notification title.
    pub fn set_title_loc_key<S>(&mut self, key: S)
    where
        S: Into<String>,
    {
        self.alert.title_loc_key = Some(key.into());
    }

    /// Arguments for the title localization.
    pub fn set_title_loc_args<S>(&mut self, args: Vec<S>)
    where
        S: Into<String>,
    {
        self.alert.title_loc_args = Some(args.into_iter().map(|a| a.into()).collect());
    }

    /// The localization key for the action.
    pub fn set_action_loc_key<S>(&mut self, key: S)
    where
        S: Into<String>,
    {
        self.alert.action_loc_key = Some(key.into());
    }

    /// The localization key for the push message body.
    pub fn set_loc_key<S>(&mut self, key: S)
    where
        S: Into<String>,
    {
        self.alert.loc_key = Some(key.into());
    }

    /// Arguments for the content localization.
    pub fn set_loc_args<S>(&mut self, args: Vec<S>)
    where
        S: Into<String>,
    {
        self.alert.loc_args = Some(args.into_iter().map(|a| a.into()).collect());
    }

    /// Image to display in the rich notification.
    pub fn set_launch_image<S>(&mut self, image: S)
    where
        S: Into<String>,
    {
        self.alert.launch_image = Some(image.into());
    }

    /// Allow client to modify push content before displaying.
    pub fn set_mutable_content(&mut self) {
        self.mutable_content = 1;
    }
}

impl NotificationBuilder for LocalizedNotificationBuilder {
    fn build<S>(self, device_token: S, options: NotificationOptions) -> Payload
    where
        S: Into<String>,
    {
        Payload {
            aps: APS {
                alert: Some(APSAlert::Localized(self.alert)),
                badge: self.badge,
                sound: self.sound,
                content_available: None,
                category: self.category,
                mutable_content: Some(self.mutable_content),
            },
            device_token: device_token.into(),
            options: options,
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_notification_with_minimal_required_values() {
        let payload = LocalizedNotificationBuilder::new("the title", "the body")
            .build("device-token", Default::default())
            .to_json_string()
            .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "title": "the title",
                    "body": "the body",
                },
                "mutable-content": 0
            }
        }).to_string();

        assert_eq!(expected_payload, payload);
    }

    #[test]
    fn test_localized_notification_with_full_data() {
        let mut builder = LocalizedNotificationBuilder::new("the title", "the body");

        builder.set_badge(420);
        builder.set_category("cat1");
        builder.set_sound("prööt");
        builder.set_mutable_content();
        builder.set_action_loc_key("PLAY");
        builder.set_launch_image("foo.jpg");
        builder.set_loc_args(vec!["argh", "narf"]);
        builder.set_title_loc_key("STOP");
        builder.set_title_loc_args(vec!["herp", "derp"]);
        builder.set_loc_key("PAUSE");
        builder.set_loc_args(vec!["narf", "derp"]);

        let payload = builder
            .build("device-token", Default::default())
            .to_json_string()
            .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "action-loc-key": "PLAY",
                    "body": "the body",
                    "launch-image": "foo.jpg",
                    "loc-args": ["narf", "derp"],
                    "loc-key": "PAUSE",
                    "title": "the title",
                    "title-loc-args": ["herp", "derp"],
                    "title-loc-key": "STOP"
                },
                "badge": 420,
                "category": "cat1",
                "mutable-content": 1,
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

        let mut payload = LocalizedNotificationBuilder::new("the title", "the body")
            .build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "title": "the title",
                    "body": "the body",
                },
                "mutable-content": 0
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

        assert_eq!(expected_payload, payload.to_json_string().unwrap());
    }
}
