use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{Payload, APS};
use std::collections::BTreeMap;

/// A builder to create an APNs silent notification payload which can be used to
/// send custom data to the user's phone if the user hasn't been running the app
/// for a while. The custom data needs to be implementing `Serialize` from Serde.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashMap;
/// # use a2::request::notification::{NotificationBuilder, SilentNotificationBuilder};
/// # fn main() {
/// let mut test_data = HashMap::new();
/// test_data.insert("a", "value");
///
/// let mut payload = SilentNotificationBuilder::new()
///    .build("device_id", Default::default());
///
/// payload.add_custom_data("custom", &test_data);
///
/// assert_eq!(
///     "{\"aps\":{\"content-available\":1},\"custom\":{\"a\":\"value\"}}",
///     &payload.to_json_string().unwrap()
/// );
/// # }
/// ```
pub struct SilentNotificationBuilder {
    content_available: u8,
}

impl SilentNotificationBuilder {
    /// Creates a new builder.
    ///
    /// ```rust
    /// # use a2::request::notification::{SilentNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// let payload = SilentNotificationBuilder::new()
    ///     .build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"content-available\":1}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new() -> SilentNotificationBuilder {
        SilentNotificationBuilder {
            content_available: 1,
        }
    }
}

impl<'a> NotificationBuilder<'a> for SilentNotificationBuilder {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a>
    {
        Payload {
            aps: APS {
                alert: None,
                badge: None,
                sound: None,
                content_available: Some(self.content_available),
                category: None,
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
    use std::collections::BTreeMap;

    #[test]
    fn test_silent_notification_with_no_content() {
        let payload = SilentNotificationBuilder::new()
            .build("device-token", Default::default())
            .to_json_string()
            .unwrap();

        let expected_payload = json!({
            "aps": {
                "content-available": 1
            }
        }).to_string();

        assert_eq!(expected_payload, payload);
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

        let mut payload =
            SilentNotificationBuilder::new().build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let expected_payload = json!({
            "aps": {
                "content-available": 1
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

    #[test]
    fn test_silent_notification_with_custom_hashmap() {
        let mut test_data = BTreeMap::new();
        test_data.insert("key_str", "foo");
        test_data.insert("key_str2", "bar");

        let mut payload =
            SilentNotificationBuilder::new().build("device-token", Default::default());

        payload.add_custom_data("custom", &test_data).unwrap();

        let expected_payload = json!({
            "aps": {
                "content-available": 1
            },
            "custom": {
                "key_str": "foo",
                "key_str2": "bar"
            }
        }).to_string();

        assert_eq!(expected_payload, payload.to_json_string().unwrap());
    }
}
