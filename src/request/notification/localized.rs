use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, Payload, APS};

use std::{
    collections::BTreeMap,
    borrow::Cow,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct LocalizedAlert<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_image: Option<&'a str>,
}

/// A builder to create a localized APNs payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
/// let mut builder = LocalizedNotificationBuilder::new();
/// builder.set_title("Hi there");
/// builder.set_body("What's up?");
/// builder.set_badge(420);
/// builder.set_category("cat1");
/// builder.set_sound("prööt");
/// builder.set_mutable_content();
/// builder.set_action_loc_key("PLAY");
/// builder.set_launch_image("foo.jpg");
/// builder.set_loc_args(&["argh", "narf"]);
/// builder.set_title_loc_key("STOP");
/// builder.set_title_loc_args(&["herp", "derp"]);
/// builder.set_loc_key("PAUSE");
/// builder.set_loc_args(&["narf", "derp"]);
/// let payload = builder.build("device_id", Default::default())
///   .to_json_string().unwrap();
/// ```
#[derive(Default)]
pub struct LocalizedNotificationBuilder<'a> {
    alert: LocalizedAlert<'a>,
    badge: Option<u32>,
    sound: Option<&'a str>,
    category: Option<&'a str>,
    mutable_content: u8,
}

impl<'a> LocalizedNotificationBuilder<'a> {
    /// Creates a new builder.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let payload = LocalizedNotificationBuilder::new()
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn new() -> LocalizedNotificationBuilder<'a> {
        Self::default()
    }

    /// Set the title
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let payload = LocalizedNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_title(&mut self, title: &'a str) -> &mut Self {
        self.alert.title = Some(title);
        self
    }

    /// Set the body
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let payload = LocalizedNotificationBuilder::new()
    ///     .set_body("a body")
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_body(&mut self, body: &'a str) -> &mut Self {
        self.alert.body = Some(body);
        self
    }

    /// A number to show on a badge on top of the app icon.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_badge(4);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{},\"badge\":4,\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_badge(&mut self, badge: u32) -> &mut Self
    {
        self.badge = Some(badge);
        self
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_sound("ping");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{},\"mutable-content\":0,\"sound\":\"ping\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_sound(&mut self, sound: &'a str) -> &mut Self
    {
        self.sound = Some(sound);
        self
    }

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_category("cat1");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{},\"category\":\"cat1\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_category(&mut self, category: &'a str) -> &mut Self
    {
        self.category = Some(category.into());
        self
    }

    /// The localization key for the notification title.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_title_loc_key("play");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title-loc-key\":\"play\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_title_loc_key(&mut self, key: &'a str) -> &mut Self
    {
        self.alert.title_loc_key = Some(key);
        self
    }

    /// Arguments for the title localization.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_title_loc_args(&["foo", "bar"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title-loc-args\":[\"foo\",\"bar\"]},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_title_loc_args<S>(
        &mut self,
        args: &'a [S]
    ) -> &mut Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>
    {
        let converted = args
            .iter()
            .map(|a| a.as_ref().into())
            .collect();

        self.alert.title_loc_args = Some(converted);
        self
    }

    /// The localization key for the action.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_action_loc_key("stop");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"action-loc-key\":\"stop\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_action_loc_key(&mut self, key: &'a str) -> &mut Self
    {
        self.alert.action_loc_key = Some(key);
        self
    }

    /// The localization key for the push message body.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_loc_key("lol");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"loc-key\":\"lol\",},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_loc_key(&mut self, key: &'a str) -> &mut Self
    {
        self.alert.loc_key = Some(key);
        self
    }

    /// Arguments for the content localization.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_loc_args(&["omg", "foo"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"loc-args\":[\"omg\",\"foo\"]},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_loc_args<S>(
        &mut self,
        args: &'a [S]
    ) -> &mut Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>
    {
        let converted = args
            .iter()
            .map(|a| a.as_ref().into())
            .collect();

        self.alert.loc_args = Some(converted);
        self
    }

    /// Image to display in the rich notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_launch_image("cat.png");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"launch-image\":\"cat.png\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_launch_image(&mut self, image: &'a str) -> &mut Self
    {
        self.alert.launch_image = Some(image);
        self
    }

    /// Allow client to modify push content before displaying.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// let mut builder = LocalizedNotificationBuilder::new();
    /// builder.set_mutable_content();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{},\"mutable-content\":1}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// ```
    pub fn set_mutable_content(&mut self) -> &mut Self
    {
        self.mutable_content = 1;
        self
    }
}

impl<'a> NotificationBuilder<'a> for LocalizedNotificationBuilder<'a> {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a>
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
    fn test_localized_notification_with_minimal_required_values() {
        let mut builder = LocalizedNotificationBuilder::new();
        builder
            .set_title("the title")
            .set_body("the body");
        let payload = builder
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
        let mut builder = LocalizedNotificationBuilder::new();

        builder.set_title("the title");
        builder.set_body("the body");
        builder.set_badge(420);
        builder.set_category("cat1");
        builder.set_sound("prööt");
        builder.set_mutable_content();
        builder.set_action_loc_key("PLAY");
        builder.set_launch_image("foo.jpg");
        builder.set_loc_args(&["argh", "narf"]);
        builder.set_title_loc_key("STOP");
        builder.set_title_loc_args(&["herp", "derp"]);
        builder.set_loc_key("PAUSE");
        builder.set_loc_args(&["narf", "derp"]);

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

        let mut builder = LocalizedNotificationBuilder::new();
        builder
            .set_title("the title")
            .set_body("the body");
        let mut payload = builder.build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

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
                "alert": {
                    "title": "the title",
                    "body": "the body",
                },
                "mutable-content": 0
            },
        }).to_string();

        assert_eq!(expected_payload, payload.to_json_string().unwrap());
    }
}
