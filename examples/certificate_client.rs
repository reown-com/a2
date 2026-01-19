use apns_h2::{Client, DefaultNotificationBuilder, NotificationBuilder, NotificationOptions};
use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};

// An example client connectiong to APNs with a certificate and key
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().init();

    let mut certificate_file = String::new();
    let mut password = String::new();
    let mut device_token = String::new();
    let mut message = String::from("Ch-check it out!");
    let mut sandbox = false;
    let mut topic: Option<String> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("APNs certificate-based push");
        ap.refer(&mut certificate_file)
            .add_option(&["-c", "--certificate"], Store, "Certificate PKCS12 file location");
        ap.refer(&mut password)
            .add_option(&["-p", "--password"], Store, "Certificate password");
        ap.refer(&mut device_token)
            .add_option(&["-d", "--device_token"], Store, "APNs device token");
        ap.refer(&mut message)
            .add_option(&["-m", "--message"], Store, "Notification message");
        ap.refer(&mut sandbox)
            .add_option(&["-s", "--sandbox"], StoreTrue, "Use the development APNs servers");
        ap.refer(&mut topic)
            .add_option(&["-o", "--topic"], StoreOption, "APNS topic");
        ap.parse_args_or_exit();
    }

    // Connecting to APNs using a client certificate
    let new_client = || -> Result<Client, Box<dyn std::error::Error + Sync + Send>> {
        #[cfg(feature = "openssl")]
        {
            // Which service to call, test or production?
            let endpoint = if sandbox {
                apns_h2::Endpoint::Sandbox
            } else {
                apns_h2::Endpoint::Production
            };

            let mut certificate = std::fs::File::open(certificate_file)?;

            // Create config with the given endpoint and default timeouts
            let client_config = apns_h2::ClientConfig::new(endpoint);

            Ok(Client::certificate(&mut certificate, &password, client_config)?)
        }
        #[cfg(all(not(feature = "openssl"), feature = "ring"))]
        {
            Err("ring does not support loading of certificates".into())
        }
    };
    let client = new_client()?;

    let options = NotificationOptions {
        apns_topic: topic.as_deref(),
        ..Default::default()
    };

    // Notification payload
    let builder = DefaultNotificationBuilder::new()
        .set_body(message.as_ref())
        .set_sound("default")
        .set_badge(1u32);

    let payload = builder.build(device_token.as_ref(), options);
    let response = client.send(payload).await?;

    println!("Sent: {:?}", response);

    Ok(())
}
