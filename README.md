# apns2
HTTP/2 Apple Push Notification Service for Rust using Tokio and async sending.

## Status

The library is based on an experimental version of
[Hyper](https://github.com/hyperium/hyper) that includes support for Http2
protocol. The [h2 crate](https://github.com/carllerche/h2) underneath is already
quite stable and passes h2spec 100%. Regarding the status of Hyper and its h2
integration not yet even merged to the master branch, consider this crate to be
alpha and breaking in every possible way.

## Usage

Add this to `Cargo.toml`:

```
[dependencies]
apns2 = { git = "https://github.com/pimeys/apns2" }
tokio-core = "0.1"
futures = "0.1"
```

then add to your crate root:

```rust
extern crate apns2;
extern crate tokio_core;
extern crate futures;
```

## Examples

The library supports connecting to Apple Push Notification service [either using
a
certificate](https://github.com/pimeys/apns2/blob/master/examples/certificate_client.rs)
with a password [or a private
key](https://github.com/pimeys/apns2/blob/master/examples/token_client.rs) with
a team id and key id. Both are available from your Apple account and with both
it is possible to send push notifications to one application.

## Tests

`cargo test`
