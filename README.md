# a2

[![Cargo tests](https://github.com/pimeys/a2/actions/workflows/test.yml/badge.svg)](https://github.com/pimeys/a2/actions/workflows/test.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/a2)](https://crates.io/crates/a2)

[Matrix chat](https://matrix.to/#/#rust-push:nauk.io?via=nauk.io&via=matrix.org&via=shine.horse)

HTTP/2 Apple Push Notification Service for Rust using Tokio and async sending.

## Help needed

The main author is not currently owning any Apple phones, so would be nice to have some help from a co-author with needed devices and an Apple developer account. If you happen to have them and are willing to help, please contact!

## Requirements

Needs a Tokio executor version 0.2 or later and Rust compiler version 1.60 or later.

## Documentation

* [Released](https://docs.rs/a2/)
* [Master](https://pimeys.github.io/a2/master/)

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

## Examples

The library supports connecting to Apple Push Notification service [either using
a
certificate](https://github.com/pimeys/a2/blob/master/examples/certificate_client.rs)
with a password [or a private
key](https://github.com/pimeys/a2/blob/master/examples/token_client.rs) with
a team id and key id. Both are available from your Apple account and with both
it is possible to send push notifications to one application.

To see it used in a real project, take a look to the [XORC
Notifications](https://github.com/xray-tech/xorc-notifications), which is a
full-fledged consumer for sending push notifications.

## Gotchas

We've been pushing some millions of notifications daily through this library and
are quite happy with it. Some things to know, if you're evaluating the library
for production use:

* Do not open new connections for every request. Apple will treat it as Denial of Service attack and block the sending IP address. When using the same `Client` for multiple requests, the `Client` keeps the connection alive if pushing steady traffic through it.

* For one app, one connection is quite enough already for certain kind of
  loads. With http2 protocol, the events are asynchronous and the pipeline can
  hold several outgoing requests at the same time. The biggest reason to open
  several connections is for redundancy, running your sender service on different
  machines.

* It seems to be Apple doesn't like when sending tons of notifications with
  faulty device tokens and it might lead to `ConnectionError`s. Do not send more
  notifications with tokens that return `Unregistered`, `BadDeviceToken` or
  `DeviceTokenNotForTopic`.

## Tests

`cargo test`

## Contact

oh_lawd @ IRC (Freenode, Mozilla)
