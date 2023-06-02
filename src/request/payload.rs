/// Payload with `aps` and custom data
use crate::error::Error;
use crate::request::notification::{DefaultAlert, NotificationOptions, WebPushAlert};
use erased_serde::Serialize;
use serde_json::{self, Value};
use std::collections::BTreeMap;

/// The data and options for a push notification.
#[derive(Debug, Clone)]
pub struct Payload<'a> {
    /// Send options
    pub options: NotificationOptions<'a>,
    /// The token for the receiving device
    pub device_token: &'a str,
    /// The pre-defined notification payload
    pub aps: APS<'a>,
    /// Application specific payload
    pub data: BTreeMap<&'a str, Value>,
}

impl<'a> Payload<'a> {
    /// Client-specific custom data to be added in the payload.
    /// The `root_key` defines the JSON key in the root of the request
    /// data, and `data` the object containing custom data. The `data`
    /// should implement `Serialize`, which allows using of any Rust
    /// collection or if needing more strict type definitions, any struct
    /// that has `#[derive(Serialize)]` from [Serde](https://serde.rs).
    ///
    /// Using a `HashMap`:
    ///
    /// ```rust
    /// # use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use std::collections::HashMap;
    /// # fn main() {
    /// let mut payload = DefaultNotificationBuilder::new()
    ///     .set_content_available()
    ///     .build("token", Default::default());
    /// let mut custom_data = HashMap::new();
    ///
    /// custom_data.insert("foo", "bar");
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"content-available\":1,\"mutable-content\":0},\"foo_data\":{\"foo\":\"bar\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// Using a custom struct:
    ///
    /// ```rust
    /// #[macro_use] extern crate serde;
    /// use a2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// fn main() {
    /// #[derive(Serialize)]
    /// struct CompanyData {
    ///     foo: &'static str,
    /// }
    ///
    /// let mut payload = DefaultNotificationBuilder::new()
    ///     .set_content_available()
    ///     .build("token", Default::default());
    /// let mut custom_data = CompanyData { foo: "bar" };
    ///
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"content-available\":1,\"mutable-content\":0},\"foo_data\":{\"foo\":\"bar\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// }
    /// ```
    pub fn add_custom_data(&mut self, root_key: &'a str, data: &dyn Serialize) -> Result<&mut Self, Error> {
        self.data.insert(root_key, serde_json::to_value(data)?);

        Ok(self)
    }

    /// Combine the APS payload and the custom data to a final payload JSON.
    /// Returns an error if serialization fails.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_json_string(mut self) -> Result<String, Error> {
        let aps_data = serde_json::to_value(&self.aps)?;

        self.data.insert("aps", aps_data);

        Ok(serde_json::to_string(&self.data)?)
    }
}

/// The pre-defined notification data.
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::upper_case_acronyms)]
pub struct APS<'a> {
    /// The notification content. Can be empty for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<APSAlert<'a>>,

    /// A number shown on top of the app icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,

    /// The name of the sound file to play when user receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<&'a str>,

    /// Set to one for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_available: Option<u8>,

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<&'a str>,

    /// If set to one, the app can change the notification content before
    /// displaying it to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_args: Option<&'a [&'a str]>,
}

/// Different notification content types.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum APSAlert<'a> {
    /// A notification that supports all of the iOS features
    Default(DefaultAlert<'a>),
    /// Safari web push notification
    WebPush(WebPushAlert<'a>),
}
