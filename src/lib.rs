#[macro_use]
extern crate hyper;
extern crate rustc_serialize;

use std::path::{Path};

use hyper::Client;
use hyper::http::h2::Http2Protocol;
use hyper::net::{HttpsConnector, Openssl};
use rustc_serialize::json;

static DEVELOPMENT: &'static str = "https://api.development.push.apple.com";
static PRODUCTION:  &'static str = "https://api.push.apple.com";

// Request headers
header! { (APNSId, "apns-id") => [String] }
header! { (APNSExpiration, "apns-expiration") => [String] }
header! { (APNSPriority, "apns-priority") => [String] }
header! { (APNSTopic, "apns-topic") => [String] }

pub struct DeviceToken {
    pub token: String
}

#[derive(Debug, RustcEncodable)]
pub struct Payload {
    pub alert: String,
    pub badge: u32,
    pub sound: String,
}

impl Payload {
    pub fn to_json(&self) -> String {
        let alert = json::encode(&self).unwrap();
        format!("{{\"aps\":{}}}", alert)
    }
}

pub struct Service {
    client: Client,
    path: String,
}

impl Service {
    pub fn new(sandbox: bool, certificate_path: &str, private_key_path: &str) -> Service {
        let ssl = Openssl::with_cert_and_key(Path::new(certificate_path), Path::new(private_key_path)).unwrap();
        let ssl_connector = HttpsConnector::new(ssl);
        let client = Client::with_protocol(Http2Protocol::with_connector(ssl_connector));
        let enviropment = if sandbox {
            DEVELOPMENT
        } else {
            PRODUCTION
        };
        let path = format!("{}{}", enviropment, "/3/device/");
        Service {client: client, path: path}
    }

    pub fn push(&self, payload: Payload, token: DeviceToken) {
        let url = format!("{}{}", self.path, token.token);
        let url_str: &str = &url;   // .as_str() waiting on RFC revision (see issue #27729)
        let pay = payload.to_json();
        let pay_str: &str = &pay;
        let _ = self.client.post(url_str)
            .body(pay_str)
            .send().unwrap();
    }
}

#[test]
fn it_works() {
}
