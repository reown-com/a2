# apns2
HTTP/2 Apple Push Notification Service for Rust

Supports certificate-based and token-based authentication. Depends on a forked
solicit to support rust-openssl 0.7 and btls which is not yet released or
stable. We use this right now, but use at your own risk. Plans are to get rid
of solicit and use the tokio http2 client when stable and available.

### Certificate & Private Key Authentication

If having the certificate straight from Apple as PKCS12 database, it must be
converted to PEM files containing the certificate and the private key.

```shell
openssl pkcs12 -in push_key.p12 -nodes -out push_key.key -nocerts
openssl pkcs12 -in push_cert.p12 -out push_cert.pem
openssl x509 -outform der -in push_cert.pem -out push_cert.crt
```

The connection is now open for push notifications and should be kept open for
multiple notifications to prevent Apple treating the traffic as DOS. The connection
is only valid for the application where the certificate was created to.

### JWT Token Authentication

To use the PKCS8 formatted private key for token generation, one must
convert it into DER format.

```shell
openssl pkcs8 -nocrypt -in key.p8 -out newtest.der -outform DER
```

The connection can be used to send push notifications into any application
by changing the token. The token is valid for one hour until it has to be
renewed.

All responses are channels which can be blocked to receive the response. For better
throughput it is a good idea to handle the responses in another thread.

## License
[MIT License](https://github.com/tkabit/apns2/blob/master/LICENSE)
