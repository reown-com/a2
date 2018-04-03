//! Payload with `aps` and custom data

use request::notification::{LocalizedAlert, NotificationOptions};
use error::Error;
use serde_json::{self, Map, Value};
use std::borrow::Cow;
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
    pub sound: Option<Cow<'static, str>>,

    /// Set to one for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_available: Option<u8>,

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Cow<'static, str>>,

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
