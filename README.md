# a2

[![Travis Build Status](https://travis-ci.org/pimeys/a2.svg?branch=master)](https://travis-ci.org/pimeys/a2)

HTTP/2 Apple Push Notification Service for Rust using Tokio and async sending.

## Status

The library is based on master branch of
[Hyper](https://github.com/hyperium/hyper) that includes support for Http2
protocol. The [h2 crate](https://github.com/carllerche/h2) underneath is already
quite stable and passes h2spec 100%. Please consider this library highly
experimental until it's released to crates.io and breaking every possible way.

## Features

* Fast asynchronous sending, based on [h2](https://github.com/carllerche/h2) and
  [hyper](https://github.com/hyperium/hyper) crates.
* Payload serialization/deserialization with
  [serde](https://github.com/serde-rs/serde).
* Provides a type-safe way of constructing different types of payloads. Custom
  data through `Serialize`, allowing use of structs or dynamic hashmaps.
* Supports `.p12` certificate databases to connect using a custom certificate.
* Supports `.p8` private keys to connect using authentication tokens.
* If using authentication tokens, handles signature renewing for Apple's guidelines
  and caching for maximum performance.

## Usage

Add this to `Cargo.toml`:

```
[dependencies]
a2 = { git = "https://github.com/pimeys/a2" }
tokio-core = "0.1"
futures = "0.1"
```

then add to your crate root:

```rust
extern crate a2;
extern crate tokio_core;
extern crate futures;
```

## Examples

The library supports connecting to Apple Push Notification service [either using
a
certificate](https://github.com/pimeys/a2/blob/master/examples/certificate_client.rs)
with a password [or a private
key](https://github.com/pimeys/a2/blob/master/examples/token_client.rs) with
a team id and key id. Both are available from your Apple account and with both
it is possible to send push notifications to one application.

## Gotchas

We've been pushing some millions of notifications daily through this library and
are quite happy with it. Some things to know, if you're evaluating the library
for production use:

* For one app, one connections is quite enough already for certain kind of
  loads. With http2 protocol, the events are asynchronous and the pipeline can
  hold several outgoing requests at the same time. The biggest reason to open
  several connections is for redundancy, running your sender service on different
  machines.

* It seems to be Apple doesn't like when sending tons of notifications with
  faulty device tokens and it might lead to `ConnectionError`s. Do not send more
  notifications with tokens that return `Unregistered`, `BadDeviceToken` or
  `DeviceTokenNotForTopic`.

* Hyper, h2 and tokio are going through big changes, so expect the API of this
  library to change before it gets released to https://crates.io

## Tests

`cargo test`
