use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

pub struct Payload {
    /// The standard APNS payload data.
    pub aps: APS,

    /// Custom data to be handled by the app.
    pub custom: Option<CustomData>,
}

pub struct CustomData {
    /// The JSON root key for app specific custom data.
    pub key: String,

    /// The custom data.
    pub body: Json,
}

impl Payload {
    pub fn new<S>(alert: APSAlert, sound: S, badge: Option<u32>, category: Option<String>,
                  custom_data: Option<CustomData>) -> Payload
        where S: Into<String>
    {
        Payload {
            aps: APS {
                alert: Some(alert),
                badge: badge,
                sound: Some(sound.into()),
                content_available: None,
                category: category,
            },
            custom: custom_data,
        }
    }

    pub fn new_silent_notification(custom_data: Option<CustomData>) -> Payload {
        Payload {
            aps: APS {
                alert: None,
                badge: None,
                sound: None,
                content_available: Some(1),
                category: None,
            },
            custom: custom_data,
        }
    }

    pub fn to_string(&self) -> String {
        self.to_json().to_string()
    }

    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

impl ToJson for Payload {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("aps".to_string(), self.aps.to_json());

        if let Some(ref custom) = self.custom {
            d.insert(custom.key.to_string(), custom.body.clone());
        }

        Json::Object(d)
    }
}

/// The APS can contain one or more properties that specify the following user notification types:
/// - an alert message to display to the user
/// - a number to badge the app icon with
/// - a sound to play
pub struct APS {
    /// If this property is included, the system displays a standard alert or a banner,
    /// based on the user’s setting.
    pub alert: Option<APSAlert>,

    /// The number to display as the badge of the app icon.
    pub badge: Option<u32>,

    /// The name of a sound file in the app bundle or in the Library/Sounds folder of
    /// the app’s data container.
    pub sound: Option<String>,

    /// Provide this key with a value of 1 to indicate that new content is available.
    pub content_available: Option<u32>,

    /// Provide this key with a string value that represents the identifier property.
    pub category: Option<String>,
}

impl ToJson for APS {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match self.alert {
            Some(APSAlert::Plain(ref s)) => {
                d.insert("alert".to_string(), s.to_json());
            }
            Some(APSAlert::Localized(ref l)) => {
                d.insert("alert".to_string(), l.to_json());
            }
            None => {}
        };
        if let Some(ref badge) = self.badge {
            d.insert("badge".to_string(), badge.to_json());
        }
        if let Some(ref sound) = self.sound {
            d.insert("sound".to_string(), sound.to_json());
        }
        if let Some(ref content_available) = self.content_available {
            d.insert("content-available".to_string(), content_available.to_json());
        }
        if let Some(ref category) = self.category {
            d.insert("category".to_string(), category.to_json());
        }
        Json::Object(d)
    }
}

/// Can specify a string or a dictionary as the value of alert.
pub enum APSAlert {
    Plain(String),
    Localized(APSLocalizedAlert),
}

/// Child properties of the alert property.
pub struct APSLocalizedAlert {
    /// A short string describing the purpose of the notification
    pub title: String,

    /// The text of the alert message.
    pub body: String,

    /// The key to a title string in the Localizable.strings file for the current localization.
    pub title_loc_key: Option<String>,

    /// Variable string values to appear in place of the format specifiers in title-loc-key.
    pub title_loc_args: Option<Vec<String>>,

    /// If a string is specified, the system displays an alert that includes the Close and View buttons.
    pub action_loc_key: Option<String>,

    /// A key to an alert-message string in a Localizable.strings file for the current localization.
    pub loc_key: Option<String>,

    /// Variable string values to appear in place of the format specifiers in loc-key.
    pub loc_args: Option<Vec<String>>,

    /// The filename of an image file in the app bundle.
    /// The image is used as the launch image when users tap the action button or move the action slider.
    pub launch_image: Option<String>,
}

impl ToJson for APSLocalizedAlert {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("title".to_string(), self.title.to_json());
        d.insert("body".to_string(), self.body.to_json());

        if let Some(ref title_loc_key) = self.title_loc_key {
            d.insert("title-loc-key".to_string(), title_loc_key.to_json());
        } else {
            d.insert("title-loc-key".to_string(), Json::Null);
        }

        if let Some(ref title_loc_args) = self.title_loc_args {
            d.insert("title-loc-args".to_string(), title_loc_args.to_json());
        } else {
            d.insert("title-loc-args".to_string(), Json::Null);
        }

        if let Some(ref action_loc_key) = self.action_loc_key {
            d.insert("action-loc-key".to_string(), action_loc_key.to_json());
        } else {
            d.insert("action-loc-key".to_string(), Json::Null);
        }

        if let Some(ref loc_key) = self.loc_key {
            d.insert("loc-key".to_string(), loc_key.to_json());
        } else {
            d.insert("loc-key".to_string(), Json::Null);
        }

        if let Some(ref loc_args) = self.loc_args {
            d.insert("loc-args".to_string(), loc_args.to_json());
        } else {
            d.insert("loc-args".to_string(), Json::Null);
        }

        if let Some(ref launch_image) = self.launch_image {
            d.insert("launch-image".to_string(), launch_image.to_json());
        }

        Json::Object(d)
    }
}
