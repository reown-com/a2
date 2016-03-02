[![Build Status](https://travis-ci.org/tkabit/apns2.svg?branch=master)](https://travis-ci.org/tkabit/apns2)
# apns2
HTTP/2 Apple Push Notification Service for Rust

### Example

At first you need export APNs Certificate and private key from KeyChain to .p12 format. And convert to .crt, .key:
```shell
openssl pkcs12 -in PushKey.p12 -nodes -out PushKey.key -nocerts
openssl pkcs12 -in PushCert.p12 -out PushCert.pem
openssl x509 -outform der -in PushCert.pem -out PushCert.crt
```

```rust
let service = apns2::Service::new(true, "/path/to/PushCert.pem", "/path/to/PushKey.key");
let alert = apns2::APSAlert::Plain("Message!".to_string());
let payload = apns2::Payload::new(alert, 1, "default");
let dev_token = apns2::DeviceToken::new("xxx...xxx");
service.push(payload, dev_token);
```
