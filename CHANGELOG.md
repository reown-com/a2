# Changelog

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
