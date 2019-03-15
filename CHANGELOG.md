# Changelog

## v0.4.0

Introduces two changes that are a bit more drastic and hence increasing the
major version. The 2018 syntax requires a Rust compiler version 1.31 or newer
and the locking primitive hasn't been measured with high traffic yet in a2.

- Upgrade to Rust 2018 syntax [#29](https://github.com/pimeys/a2/pull/29)
- Switch from deprecated crossbeam ArcCell to parking_lot RwLock
  [#32](https://github.com/pimeys/a2/pull/32)

## v0.3.5

- Implement `fmt::Display` for `ErrorReason` [#28](https://github.com/pimeys/a2/pull/28)

## v0.3.4

- Changing the author email due to company breakdown to the private one.

## v0.3.3

- Taking the alpn connector out to its own crate, using tokio-dns for resolving

## v0.3.2

- OK responses don't have a body, so we don't need to handle it and gain a bit
  more performance

## v0.3.1

- Bunch of examples to the builder documentation

## v0.3.0

- Convert the API to not clone the input data, using references until
  converting to JSON, remove tokio-service dependency
  [#25](https://github.com/pimeys/a2/pull/25)
