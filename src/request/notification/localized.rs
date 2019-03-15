use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, Payload, APS};

use std::{
    collections::BTreeMap,
    borrow::Cow,
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct LocalizedAlert<'a> {
    title: &'a str,
    body: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    loc_args: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    launch_image: Option<&'a str>,
}

/// A builder to create a localized APNs payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
/// # fn main() {
/// let mut builder = LocalizedNotificationBuilder::new("Hi there", "What's up?");
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
/// # }
/// ```
pub struct LocalizedNotificationBuilder<'a> {
    alert: LocalizedAlert<'a>,
    badge: Option<u32>,
    sound: Option<&'a str>,
    category: Option<&'a str>,
    mutable_content: u8,
}

impl<'a> LocalizedNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let payload = LocalizedNotificationBuilder::new("a title", "a body")
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new(
        title: &'a str,
        body: &'a str
    ) -> LocalizedNotificationBuilder<'a>
    {
        LocalizedNotificationBuilder {
            alert: LocalizedAlert {
                title: title,
                body: body,
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
    ///
    /// ```rust
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_badge(4);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\"},\"badge\":4,\"mutable-content\":0}}",
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
    /// # use a2::request::notification::{LocalizedNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_sound("ping");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\"},\"mutable-content\":0,\"sound\":\"ping\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_category("cat1");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\"},\"category\":\"cat1\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_title_loc_key("play");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\",\"title-loc-key\":\"play\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_title_loc_args(&["foo", "bar"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\",\"title-loc-args\":[\"foo\",\"bar\"]},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_action_loc_key("stop");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"action-loc-key\":\"stop\",\"body\":\"a body\",\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_loc_key("lol");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"loc-key\":\"lol\",\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_loc_args(&["omg", "foo"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"loc-args\":[\"omg\",\"foo\"],\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_launch_image("cat.png");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"launch-image\":\"cat.png\",\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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
    /// # fn main() {
    /// let mut builder = LocalizedNotificationBuilder::new("a title", "a body");
    /// builder.set_mutable_content();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"body\":\"a body\",\"title\":\"a title\"},\"mutable-content\":1}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
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

        let mut payload = LocalizedNotificationBuilder::new("the title", "the body")
            .build("device-token", Default::default());

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
