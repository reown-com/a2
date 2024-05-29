use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, APSSound, Payload, APS};

use std::{borrow::Cow, collections::BTreeMap};

/// Represents a bool that serializes as a u8 0/1 for false/true respectively
mod bool_as_u8 {
    use serde::{
        de::{self, Deserializer, Unexpected},
        ser::Serializer,
        Deserialize,
    };

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(other as u64),
                &"zero or one",
            )),
        }
    }

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(match value {
            false => 0,
            true => 1,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DefaultSound<'a> {
    #[serde(skip_serializing_if = "std::ops::Not::not", with = "bool_as_u8")]
    critical: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DefaultAlert<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,

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

/// A builder to create an APNs payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
/// # use a2::request::payload::PayloadLike;
/// # fn main() {
/// let mut builder = DefaultNotificationBuilder::new()
///     .set_title("Hi there")
///     .set_subtitle("From bob")
///     .set_body("What's up?")
///     .set_badge(420)
///     .set_category("cat1")
///     .set_sound("prööt")
///     .set_thread_id("my-thread")
///     .set_critical(false, None)
///     .set_mutable_content()
///     .set_action_loc_key("PLAY")
///     .set_launch_image("foo.jpg")
///     .set_loc_args(&["argh", "narf"])
///     .set_title_loc_key("STOP")
///     .set_title_loc_args(&["herp", "derp"])
///     .set_loc_key("PAUSE")
///     .set_loc_args(&["narf", "derp"]);
/// let payload = builder.build("device_id", Default::default())
///   .to_json_string().unwrap();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct DefaultNotificationBuilder<'a> {
    alert: DefaultAlert<'a>,
    badge: Option<u32>,
    sound: DefaultSound<'a>,
    thread_id: Option<&'a str>,
    category: Option<&'a str>,
    mutable_content: u8,
    content_available: Option<u8>,
    has_edited_alert: bool,
}

impl<'a> DefaultNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let payload = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_body("a body")
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"body\":\"a body\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new() -> DefaultNotificationBuilder<'a> {
        DefaultNotificationBuilder {
            alert: DefaultAlert {
                title: None,
                subtitle: None,
                body: None,
                title_loc_key: None,
                title_loc_args: None,
                action_loc_key: None,
                loc_key: None,
                loc_args: None,
                launch_image: None,
            },
            badge: None,
            sound: DefaultSound {
                critical: false,
                name: None,
                volume: None,
            },
            thread_id: None,
            category: None,
            mutable_content: 0,
            content_available: None,
            has_edited_alert: false,
        }
    }

    /// Set the title of the notification.
    /// Apple Watch displays this string in the short look notification interface.
    /// Specify a string that’s quickly understood by the user.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.alert.title = Some(title);
        self.has_edited_alert = true;
        self
    }

    /// Set critical alert value for this notification
    /// Volume can only be set when the notification is marked as critcial
    /// Note: You'll need the [critical alerts entitlement](https://developer.apple.com/contact/request/notifications-critical-alerts-entitlement/) to use `true`!
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_critical(true, None);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"sound\":{\"critical\":1},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_critical(mut self, critical: bool, volume: Option<f64>) -> Self {
        if !critical {
            self.sound.volume = None;
            self.sound.critical = false;
        } else {
            self.sound.volume = volume;
            self.sound.critical = true;
        }
        self
    }

    /// Used to set the subtitle which should provide additional information that explains the purpose of the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_subtitle("a subtitle");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"subtitle\":\"a subtitle\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_subtitle(mut self, subtitle: &'a str) -> Self {
        self.alert.subtitle = Some(subtitle);
        self.has_edited_alert = true;
        self
    }

    /// Sets the content of the alert message.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_body("a body");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":\"a body\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_body(mut self, body: &'a str) -> Self {
        self.alert.body = Some(body);
        self
    }

    /// A number to show on a badge on top of the app icon.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_badge(4);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"badge\":4,\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_badge(mut self, badge: u32) -> Self {
        self.badge = Some(badge);
        self
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_sound("ping");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"sound\":\"ping\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(mut self, sound: &'a str) -> Self {
        self.sound.name = Some(sound);
        self
    }

    /// An application-specific name that allows notifications to be grouped together.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_thread_id("my-thread");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"thread-id\":\"my-thread\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_thread_id(mut self, thread_id: &'a str) -> Self {
        self.thread_id = Some(thread_id);
        self
    }

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_category("cat1");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"category\":\"cat1\",\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_category(mut self, category: &'a str) -> Self {
        self.category = Some(category);
        self
    }

    /// The localization key for the notification title.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_title_loc_key("play");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"title-loc-key\":\"play\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_title_loc_key(mut self, key: &'a str) -> Self {
        self.alert.title_loc_key = Some(key);
        self.has_edited_alert = true;
        self
    }

    /// Arguments for the title localization.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_title_loc_args(&["foo", "bar"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"title-loc-args\":[\"foo\",\"bar\"]},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_title_loc_args<S>(mut self, args: &'a [S]) -> Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = args.iter().map(|a| a.as_ref().into()).collect();

        self.alert.title_loc_args = Some(converted);
        self.has_edited_alert = true;
        self
    }

    /// The localization key for the action.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_action_loc_key("stop");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"action-loc-key\":\"stop\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_action_loc_key(mut self, key: &'a str) -> Self {
        self.alert.action_loc_key = Some(key);
        self.has_edited_alert = true;
        self
    }

    /// The localization key for the push message body.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_loc_key("lol");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"loc-key\":\"lol\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_loc_key(mut self, key: &'a str) -> Self {
        self.alert.loc_key = Some(key);
        self.has_edited_alert = true;
        self
    }

    /// Arguments for the content localization.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_loc_args(&["omg", "foo"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"loc-args\":[\"omg\",\"foo\"]},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_loc_args<S>(mut self, args: &'a [S]) -> Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = args.iter().map(|a| a.as_ref().into()).collect();

        self.alert.loc_args = Some(converted);
        self.has_edited_alert = true;
        self
    }

    /// Image to display in the rich notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_launch_image("cat.png");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\",\"launch-image\":\"cat.png\"},\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_launch_image(mut self, image: &'a str) -> Self {
        self.alert.launch_image = Some(image);
        self.has_edited_alert = true;
        self
    }

    /// Allow client to modify push content before displaying.
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_mutable_content();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"mutable-content\":1}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_mutable_content(mut self) -> Self {
        self.mutable_content = 1;
        self
    }

    /// Used for adding custom data to push notifications
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = DefaultNotificationBuilder::new()
    ///     .set_title("a title")
    ///     .set_content_available();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"a title\"},\"content-available\":1,\"mutable-content\":0}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_content_available(mut self) -> Self {
        self.content_available = Some(1);
        self
    }
}

impl<'a> NotificationBuilder<'a> for DefaultNotificationBuilder<'a> {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a> {
        Payload {
            aps: APS {
                alert: match self.has_edited_alert {
                    true => Some(APSAlert::Default(self.alert)),
                    false => self.alert.body.map(APSAlert::Body),
                },
                badge: self.badge,
                sound: if self.sound.critical {
                    Some(APSSound::Critical(self.sound))
                } else {
                    self.sound.name.map(APSSound::Sound)
                },
                thread_id: self.thread_id,
                content_available: self.content_available,
                category: self.category,
                mutable_content: Some(self.mutable_content),
                url_args: None,
            },
            device_token,
            options,
            data: BTreeMap::new(),
        }
    }
}

impl<'a> Default for DefaultNotificationBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::value::to_value;

    #[test]
    fn test_default_notification_with_minimal_required_values() {
        let payload = DefaultNotificationBuilder::new()
            .set_title("the title")
            .set_body("the body")
            .build("device-token", Default::default());

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "body": "the body",
                    "title": "the title",
                },
                "mutable-content": 0
            }
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_default_notification_with_full_data() {
        let builder = DefaultNotificationBuilder::new()
            .set_title("the title")
            .set_body("the body")
            .set_badge(420)
            .set_category("cat1")
            .set_sound("prööt")
            .set_critical(true, Some(1.0))
            .set_mutable_content()
            .set_action_loc_key("PLAY")
            .set_launch_image("foo.jpg")
            .set_loc_args(&["argh", "narf"])
            .set_title_loc_key("STOP")
            .set_title_loc_args(&["herp", "derp"])
            .set_loc_key("PAUSE")
            .set_loc_args(&["narf", "derp"]);

        let payload = builder.build("device-token", Default::default());

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
                "sound": {
                    "critical": 1,
                    "name": "prööt",
                    "volume": 1.0,
                },
                "category": "cat1",
                "mutable-content": 1,
            }
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_notification_with_custom_data_1() {
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

        let mut payload = DefaultNotificationBuilder::new()
            .set_title("the title")
            .set_body("the body")
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
                    "body": "the body",
                    "title": "the title",
                },
                "mutable-content": 0,
            },
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_notification_with_custom_data_2() {
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

        let mut payload = DefaultNotificationBuilder::new()
            .set_body("kulli")
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
                "alert": "kulli",
                "mutable-content": 0
            }
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_silent_notification_with_no_content() {
        let payload = DefaultNotificationBuilder::new()
            .set_content_available()
            .build("device-token", Default::default());

        let expected_payload = json!({
            "aps": {
                "content-available": 1,
                "mutable-content": 0
            }
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_silent_notification_with_custom_data() {
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

        let mut payload = DefaultNotificationBuilder::new()
            .set_content_available()
            .build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let expected_payload = json!({
            "aps": {
                "content-available": 1,
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
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }

    #[test]
    fn test_silent_notification_with_custom_hashmap() {
        let mut test_data = BTreeMap::new();
        test_data.insert("key_str", "foo");
        test_data.insert("key_str2", "bar");

        let mut payload = DefaultNotificationBuilder::new()
            .set_content_available()
            .build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let expected_payload = json!({
            "aps": {
                "content-available": 1,
                "mutable-content": 0,
            },
            "custom": {
                "key_str": "foo",
                "key_str2": "bar"
            }
        });

        assert_eq!(expected_payload, to_value(payload).unwrap());
    }
}
