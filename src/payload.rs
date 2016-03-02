use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

pub struct Payload {
    pub aps: APS,
}

pub struct APS {
    pub alert: APSAlert,
    pub badge: u32,
    pub sound: String,
    //pub content_available: Option<u32>,
    //pub category: Option<String>,
}

pub enum APSAlert {
    Plain(String),
    Localized(APSLocalizedAlert),
}

pub struct APSLocalizedAlert {
    pub title: String,
    pub body: String,
    pub title_loc_key: String,
    pub title_loc_args: Vec<String>,//or nil
    pub action_loc_key: String,     //or nil
    pub loc_key: String,
    pub loc_args: Vec<String>,      //or nil
    pub launch_image: String,
}

impl Payload {
    pub fn new<S>(alert: APSAlert, badge: u32, sound: S) -> Payload where S: Into<String> {
        Payload {
            aps: APS {
                badge: badge,
                sound: sound.into(),
                alert: alert,
                //content_available: None,
                //category: None,
            }
        }
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
        d.insert("alert".to_string(), match self.alert {
            APSAlert::Plain(ref s) => s.to_json(),
            APSAlert::Localized(ref l) => l.to_json()
        });
        d.insert("badge".to_string(), self.badge.to_json());
        d.insert("sound".to_string(), self.sound.to_json());
        Json::Object(d)
    }
}

impl ToJson for APSLocalizedAlert {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("title".to_string(), self.title.to_json());
        d.insert("body".to_string(), self.body.to_json());
        d.insert("title-loc-key".to_string(), self.title_loc_key.to_json());
        d.insert("title-loc-args".to_string(), self.title_loc_args.to_json());
        d.insert("action-loc-key".to_string(), self.action_loc_key.to_json());
        d.insert("loc-key".to_string(), self.loc_key.to_json());
        d.insert("loc-args".to_string(), self.loc_args.to_json());
        d.insert("launch-image".to_string(), self.launch_image.to_json());
        Json::Object(d)
    }
}
