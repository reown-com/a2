use request::notification::{NotificationBuilder, NotificationOptions};
use request::payload::{Payload, APS};

/// A builder to create an APNs silent notification payload which can be used to
/// send custom data to the user's phone if the user hasn't been running the app
/// for a while. The custom data needs to be implementing `Serialize` from Serde.
///
/// # Example
///
/// ```rust
/// # extern crate a2;
/// # use std::collections::HashMap;
/// # use a2::request::notification::{NotificationOptions, NotificationBuilder, SilentNotificationBuilder};
/// # fn main() {
/// let mut test_data = HashMap::new();
/// test_data.insert("a", "value");
/// test_data.insert("another", "value");
///
/// let options = NotificationOptions::default();
/// let mut payload = SilentNotificationBuilder::new()
///    .build("device_id", &options);
///
/// payload.add_custom_data("custom", &test_data);
/// payload.to_json_string().unwrap();
/// # }
/// ```
pub struct SilentNotificationBuilder {
    content_available: u8,
}

impl SilentNotificationBuilder {
    pub fn new() -> SilentNotificationBuilder {
        SilentNotificationBuilder {
            content_available: 1,
        }
    }
}

impl NotificationBuilder for SilentNotificationBuilder {
    fn build<'a, 'b>(
        self,
        device_token: &'a str,
        options: &'b NotificationOptions,
    ) -> Payload<'a, 'b> {
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
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_silent_notification_with_no_content() {
        let options = NotificationOptions::default();
        let payload = SilentNotificationBuilder::new()
            .build("device-token", &options)
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

        let options = NotificationOptions::default();
        let mut payload =
            SilentNotificationBuilder::new().build("device-token", &options);

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
        let mut test_data = HashMap::new();
        test_data.insert("key_str", "foo");
        test_data.insert("key_str2", "bar");

        let options = NotificationOptions::default();
        let mut payload =
            SilentNotificationBuilder::new().build("device-token", &options);

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
