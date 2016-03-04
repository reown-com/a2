[![Build Status](https://travis-ci.org/tkabit/apns2.svg?branch=master)](https://travis-ci.org/tkabit/apns2)
# apns2
HTTP/2 Apple Push Notification Service for Rust

### Example

At first you need export APNs Certificate and private key from KeyChain to .p12 format. And convert to .crt, .key:
```shell
openssl pkcs12 -in push_key.p12 -nodes -out push_key.key -nocerts
openssl pkcs12 -in push_cert.p12 -out push_cert.pem
openssl x509 -outform der -in push_cert.pem -out push_cert.crt
```

```rust
let provider = apns2::Provider::new(true, "/path/to/push_cert.pem", "/path/to/push_key.key");
let alert = apns2::APSAlert::Plain("Message!".to_string());
let payload = apns2::Payload::new(alert, 1, "default");
let token = apns2::DeviceToken::new("xxx...xxx");
let notification = apns2::Notification::new(payload, token);
provider.push(notification);
```
