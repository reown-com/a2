extern crate apns2;
extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use apns2::client::CertificateClient;
use apns2::payload::{Payload, APSAlert};
use apns2::notification::{Notification, NotificationOptions};
use std::fs::File;
use std::time::Duration;

// An example client connectiong to APNs with a certificate and key
fn main() {
    let mut certificate_pem_file = String::new();
    let mut key_pem_file = String::new();
    let mut device_token = String::new();
    let mut message = String::from("Ch-check it out!");
    let mut sandbox = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("APNs certificate-based push");
        ap.refer(&mut certificate_pem_file).add_option(&["-c", "--certificate"], Store, "Certificate PEM file location");
        ap.refer(&mut key_pem_file).add_option(&["-k", "--key"], Store, "Private key PEM file location");
        ap.refer(&mut device_token).add_option(&["-d", "--device_token"], Store, "APNs device token");
        ap.refer(&mut message).add_option(&["-m", "--message"], Store, "Notification message");
        ap.refer(&mut sandbox).add_option(&["-s", "--sandbox"], StoreTrue, "Use the development APNs servers");
        ap.parse_args_or_exit();
    }

    // Read the private key and certificate from the disk
    let mut cert_file = File::open(certificate_pem_file).unwrap();
    let mut key_file = File::open(key_pem_file).unwrap();

    // Create a new client to APNs
    let client = CertificateClient::new(sandbox, &mut cert_file, &mut key_file).unwrap();

    // APNs payload
    let payload = Payload::new(APSAlert::Plain(message), "default", Some(1u32), None, None);

    let options = NotificationOptions {
        ..Default::default()
    };

    // Fire the request, return value is a mpsc rx channel
    let request = client.push(Notification::new(payload, &device_token, options));

    // Read the response and block maximum of 2000 milliseconds, throwing an exception for a timeout
    let response = request.recv_timeout(Duration::from_millis(2000));

    println!("{:?}", response);
}
