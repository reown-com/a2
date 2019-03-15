use tokio;
use pretty_env_logger;
use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};
use std::fs::File;

use a2::{
    Client,
    Endpoint,
    NotificationBuilder,
    NotificationOptions,
    PlainNotificationBuilder,
};

use futures::{
    future::lazy,
    Future,
};

// An example client connectiong to APNs with a JWT token
fn main() {
    pretty_env_logger::init();

    let mut key_file = String::new();
    let mut team_id = String::new();
    let mut key_id = String::new();
    let mut device_token = String::new();
    let mut message = String::from("Ch-check it out!");
    let mut sandbox = false;
    let mut topic: Option<String> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("APNs token-based push");
        ap.refer(&mut key_file)
            .add_option(&["-p", "--pkcs8"], Store, "Private key PKCS8");
        ap.refer(&mut team_id)
            .add_option(&["-t", "--team_id"], Store, "APNs team ID");
        ap.refer(&mut key_id)
            .add_option(&["-k", "--key_id"], Store, "APNs key ID");
        ap.refer(&mut device_token).add_option(
            &["-d", "--device_token"],
            Store,
            "APNs device token",
        );
        ap.refer(&mut message)
            .add_option(&["-m", "--message"], Store, "Notification message");
        ap.refer(&mut sandbox).add_option(
            &["-s", "--sandbox"],
            StoreTrue,
            "Use the development APNs servers",
        );
        ap.refer(&mut topic)
            .add_option(&["-o", "--topic"], StoreOption, "APNS topic");
        ap.parse_args_or_exit();
    }

    // Read the private key from disk
    let mut private_key = File::open(key_file).unwrap();

    // Which service to call, test or production?
    let endpoint = if sandbox {
        Endpoint::Sandbox
    } else {
        Endpoint::Production
    };

    // Connecting to APNs
    let client = Client::token(
        &mut private_key,
        key_id.as_ref(),
        team_id.as_ref(),
        endpoint,
    ).unwrap();

    let options = NotificationOptions {
        apns_topic: topic.as_ref().map(|s| &**s),
        ..Default::default()
    };

    // Notification payload
    let mut builder = PlainNotificationBuilder::new(message.as_ref());
    builder.set_sound("default");
    builder.set_badge(1u32);

    let payload = builder.build(device_token.as_ref(), options);
    let sending = client.send(payload);

    // Send the notification, parse response
    tokio::run(lazy(move || {
        sending
            .map(|response| {
                println!("Sent: {:?}", response);
            })
            .map_err(|error| {
                println!("Error: {:?}", error);
            })
    }));
}
