use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

pub struct Payload {
    pub aps: APS,
}

pub struct APS {
    pub alert: Option<APSAlert>,

    // The number to display as the badge of the app icon.
    pub badge: Option<u32>,

    // The name of a sound file in the app bundle or in the Library/Sounds folder of
    // the app’s data container.
    pub sound: Option<String>,

    // Provide this key with a value of 1 to indicate that new content is available.
    pub content_available: Option<u32>,

    // Provide this key with a string value that represents the identifier property.
    pub category: Option<String>,
}

pub enum APSAlert {
    Plain(String),
    Localized(APSLocalizedAlert),
}

pub struct APSLocalizedAlert {
    pub title: String,
    pub body: String,
    pub title_loc_key: Option<String>,
    pub title_loc_args: Option<Vec<String>>,
    pub action_loc_key: Option<String>,
    pub loc_key: String,
    pub loc_args: Vec<String>,
    pub launch_image: Option<String>,
}

impl Payload {
    pub fn new<S>(alert: APSAlert, badge: u32, sound: S, category: Option<String>) -> Payload
        where S: Into<String>
    {
        Payload {
            aps: APS {
                alert: Some(alert),
                badge: Some(badge),
                sound: Some(sound.into()),
                content_available: None,
                category: category,
            },
        }
    }

    pub fn new_silent_notification() -> Payload {
        Payload {
            aps: APS {
                alert: None,
                badge: None,
                sound: None,
                content_available: Some(1),
                category: None,
            },
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
        Json::Object(d)
    }
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
        if self.badge.is_some() {
            d.insert("badge".to_string(), self.badge.to_json());
        }
        if self.sound.is_some() {
            d.insert("sound".to_string(), self.sound.to_json());
        }
        if self.content_available.is_some() {
            d.insert("content-available".to_string(),
                     self.content_available.to_json());
        }
        if self.category.is_some() {
            d.insert("category".to_string(), self.category.to_json());
        }
        Json::Object(d)
    }
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

        d.insert("loc-key".to_string(), self.loc_key.to_json());
        d.insert("loc-args".to_string(), self.loc_args.to_json());

        if let Some(ref launch_image) = self.launch_image {
            d.insert("launch-image".to_string(), launch_image.to_json());
        }

        Json::Object(d)
    }
}
