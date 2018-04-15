//! Payload with `aps` and custom data

use request::notification::{LocalizedAlert, NotificationOptions};
use error::Error;
use serde_json::{self, Map, Value};
use std::collections::HashMap;
use erased_serde::Serialize;

/// The data and options for a push notification.
#[derive(Debug, Clone)]
pub struct Payload {
    /// Send options
    pub options: NotificationOptions,
    /// The token for the receiving device
    pub device_token: String,
    /// The pre-defined notification payload
    pub aps: APS,
    /// Application specific payload
    pub custom_data: Option<HashMap<String, Value>>,
}

impl Payload {
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
    /// # extern crate a2;
    /// # extern crate serde;
    /// # use a2::request::notification::{SilentNotificationBuilder, NotificationBuilder};
    /// # use std::collections::HashMap;
    /// # fn main() {
    /// let mut payload = SilentNotificationBuilder::new().build("token", Default::default());
    /// let mut custom_data = HashMap::new();
    /// custom_data.insert("foo", "bar");
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    /// # }
    /// ```
    ///
    /// Using a custom struct:
    ///
    /// ```rust
    /// # extern crate a2;
    /// # extern crate serde;
    /// # #[macro_use] extern crate serde_derive;
    /// # use a2::request::notification::{SilentNotificationBuilder, NotificationBuilder};
    /// # fn main() {
    /// #[derive(Serialize)]
    /// struct CompanyData {
    ///     foo: &'static str,
    /// }
    ///
    /// let mut payload = SilentNotificationBuilder::new().build("token", Default::default());
    /// let mut custom_data = CompanyData { foo: "bar" };
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    /// # }
    /// ```
    pub fn add_custom_data<S: Into<String>>(
        &mut self,
        root_key: S,
        data: &Serialize,
    ) -> Result<(), Error> {
        if let Some(ref mut map) = self.custom_data {
            map.insert(root_key.into(), serde_json::to_value(data)?);
        } else {
            let mut map = HashMap::new();
            map.insert(root_key.into(), serde_json::to_value(data)?);
            self.custom_data = Some(map);
        }

        Ok(())
    }

    /// Combine the APS payload and the custom data to a final payload JSON.
    /// Returns an error if serialization fails.
    pub fn to_json_string(&self) -> Result<String, Error> {
        let mut payload = Map::new();

        if let Some(ref data) = self.custom_data {
            for (key, value) in data {
                payload.insert(key.clone(), value.clone());
            }
        }

        payload.insert(String::from("aps"), serde_json::to_value(&self.aps)?);

        Ok(serde_json::to_string(&payload)?)
    }
}

/// The pre-defined notification data.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct APS {
    /// The notification content. Can be empty for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<APSAlert>,

    /// A number shown on top of the app icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,

    /// The name of the sound file to play when user receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,

    /// Set to one for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_available: Option<u8>,

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// If set to one, the app can change the notification content before
    /// displaying it to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<u8>,
}

/// Different notification content types.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum APSAlert {
    /// Text-only notification.
    Plain(String),
    /// A rich localized notification.
    Localized(LocalizedAlert),
}
