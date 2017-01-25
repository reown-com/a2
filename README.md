[![Build Status](https://travis-ci.org/polyitan/apns2.svg?branch=master)](https://travis-ci.org/polyitan/apns2)
# apns2
HTTP/2 Apple Push Notification Service for Rust

## Install (Important: not published in Cargo yet!!!)
Add this to your Cargo.toml:
```toml
[dependencies]
apns2 = "0.0.1"
```
and this to your crate root:
```rust
extern crate apns2;

use apns2::{Provider,  DeviceToken};
use apns2::{Notification, NotificationOptions};
use apns2::{Payload, APS, APSAlert, APSLocalizedAlert};
use apns2::{Response, APNSError};
```
## Generate cert and key files
At first you need export APNs Certificate from KeyChain to .p12 format. And convert to .pem:
```shell
openssl pkcs12 -in push.p12 -clcerts -out push_cert.pem
openssl pkcs12 -in push.p12 -nocerts -nodes | openssl rsa > push_key.pem
```

## Usage
#### Sending a push notification
```rust
let provider = Provider::new(true, "/path/to/push_cert.pem", "/path/to/push_key.key");
let alert = APSAlert::Plain("Message!".to_string());
let payload = Payload::new(alert, Some(1), "default");
let token = DeviceToken::new("xxxx...xxxx");
let options = NotificationOptions::default();
let notification = Notification::new(payload, token, options);
provider.push(notification, |result| {
    match result {
        Ok(res)  => {
            println!("Ok: {:?}", res);
        },
        Err(res) => {
            println!("Error: {:?}", res);
        }
    }
});
```

## License
[MIT License](https://github.com/tkabit/apns2/blob/master/LICENSE)
