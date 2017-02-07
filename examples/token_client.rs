extern crate apns2;
extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use apns2::client::TokenClient;
use apns2::apns_token::APNSToken;
use apns2::payload::{Payload, APSAlert};
use apns2::notification::{Notification, NotificationOptions};
use std::fs::File;
use std::time::Duration;

// An example client connectiong to APNs with a JWT token
fn main() {
    let mut der_file_location = String::new();
    let mut team_id = String::new();
    let mut key_id = String::new();
    let mut device_token = String::new();
    let mut message = String::from("Ch-check it out!");
    let mut ca_certs = String::from("/etc/ssl/cert.pem");
    let mut sandbox = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("APNs token-based push");
        ap.refer(&mut der_file_location).add_option(&["-e", "--der"], Store, "Private key file in DER format");
        ap.refer(&mut team_id).add_option(&["-t", "--team_id"], Store, "APNs team ID");
        ap.refer(&mut key_id).add_option(&["-k", "--key_id"], Store, "APNs key ID");
        ap.refer(&mut device_token).add_option(&["-d", "--device_token"], Store, "APNs device token");
        ap.refer(&mut message).add_option(&["-m", "--message"], Store, "Notification message");
        ap.refer(&mut sandbox).add_option(&["-s", "--sandbox"], StoreTrue, "Use the development APNs servers");
        ap.refer(&mut ca_certs).add_option(&["-c", "--ca_certs"], Store, "The system CA certificates PEM file");
        ap.parse_args_or_exit();
    }

    // Read the private key from disk
    let der_file = File::open(der_file_location).unwrap();

    // Create a new token struct with the private key, team id and key id
    // The token is valid for an hour and needs to be renewed after that
    let apns_token = APNSToken::new(der_file, team_id.as_ref(), key_id.as_ref()).unwrap();

    // Create a new client to APNs, giving the system CA certs
    let client = TokenClient::new(sandbox, &ca_certs).unwrap();

    // APNs payload
    let payload = Payload::new(APSAlert::Plain(message), "default", Some(1u32), None, None);

    let options = NotificationOptions {
        ..Default::default()
    };

    // Fire the request, return value is a mpsc rx channel
    let request = client.push(Notification::new(payload, &device_token, options), apns_token.signature());

    // Read the response and block maximum of 2000 milliseconds, throwing an exception for a timeout
    let response = request.recv_timeout(Duration::from_millis(2000));

    println!("{:?}", response);
}
