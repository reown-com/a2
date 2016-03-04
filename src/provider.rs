use rustc_serialize::json::ToJson;
use std::path::{Path};
use hyper::Client;
use hyper::http::h2::Http2Protocol;
use hyper::net::{HttpsConnector, Openssl};
use notification::*;
use response::*;

static DEVELOPMENT: &'static str = "https://api.development.push.apple.com";
static PRODUCTION:  &'static str = "https://api.push.apple.com";

// Request headers
header! { (APNSId, "apns-id") => [String] }
header! { (APNSExpiration, "apns-expiration") => [String] }
header! { (APNSPriority, "apns-priority") => [String] }
header! { (APNSContentLength, "content-length") => [String] }
header! { (APNSTopic, "apns-topic") => [String] }

pub struct Provider {
    pub client: Client,
    pub path: String,
}

impl Provider {
    pub fn new(sandbox: bool, certificate_path: &str, private_key_path: &str) -> Provider {
        let ssl = Openssl::with_cert_and_key(Path::new(certificate_path), Path::new(private_key_path)).unwrap();
        let ssl_connector = HttpsConnector::new(ssl);
        let client = Client::with_protocol(Http2Protocol::with_connector(ssl_connector));
        let path = if sandbox {
            format!("{}{}", DEVELOPMENT, "/3/device/")
        } else {
            format!("{}{}", PRODUCTION, "/3/device/")
        };
        Provider {client: client, path: path}
    }

    pub fn push(&self, notification: Notification) -> Response {
        let url = format!("{}{}", self.path, notification.device_token);
        let url_str: &str = &url;   // .as_str() waiting on RFC revision (see issue #27729)
        let pay = notification.payload.to_json().to_string();
        let pay_str: &str = &pay;
        println!("{}", pay_str);
        let res = self.client.post(url_str)
            .body(pay_str)
            .send().unwrap();
        println!("{:?}", res);

        Response {status: APNSStatus::Success, reason: Some(APNSError::PayloadEmpty), timestamp: None, apns_id: None}
    }
}
