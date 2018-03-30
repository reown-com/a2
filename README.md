# apns2

[![Travis Build Status](https://travis-ci.org/pimeys/apns2.svg?branch=master)](https://travis-ci.org/pimeys/apns2)

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

## Gotchas

We've been pushing some millions of notifications daily through this library and are quite happy with it. Some things to know, if you're evaluating the library for production use:

* The connection is meant to kept up when having constant traffic and should stop Apple's DDOS blocks. Sometimes one might experience `TimeoutError`s or `ConnectionError`s, so keeping track of connections and restarting them is a good idea.

* It seems to be Apple doesn't like when sending tons of notifications with faulty device tokens and it might lead to `ConnectionError`s. Do not send more notifications with tokens that return `Unregistered`, `BadDeviceToken` or `DeviceTokenNotForTopic`.

* If using a token connection, the connection should handle renewal of the signature before it's too late and for now I haven't seen any errors related to invalid tokens.

* Hyper, h2 and tokio are going through big changes, so expect the API of this library to change before it gets released to https://crates.io

## Tests

`cargo test`
