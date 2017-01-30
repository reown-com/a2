use solicit::http::Header;
use std::fmt::Display;
use notification::Notification;

pub fn default_headers<'a, 'b>(notification: &'a Notification) -> Vec<Header<'b, 'b>> {
    let mut headers = Vec::new();
    headers.push(create_header("content_length", notification.payload.len()));

    if let Some(apns_id) = notification.options.apns_id {
        headers.push(create_header("apns-id", apns_id));
    }

    if let Some(apns_expiration) = notification.options.apns_expiration {
        headers.push(create_header("apns-expiration", apns_expiration));
    }

    if let Some(apns_priority) = notification.options.apns_priority {
        headers.push(create_header("apns-priority", apns_priority));
    }

    if let Some(apns_topic) = notification.options.apns_topic {
        headers.push(create_header("apns-topic", apns_topic));
    }

    headers
}

pub fn create_header<'a, T: Display>(key: &'a str, value: T) -> Header<'a, 'a> {
    Header::new(key.as_bytes(), format!("{}", value).into_bytes())
}
