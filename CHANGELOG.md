	# Changelog

	## v0.6.2

	- Add support for Safari web push

	## v0.6.0

	- Update to Tokio 1.0

	## v0.5.2

	- Fix `TooManyProviderTokenUpdates` issue [#44](https://github.com/pimeys/a2/pull/44)

	## v0.5.1

	- Enforcing static lifetimes for client send [#43](https://github.com/pimeys/a2/pull/43)

	## v0.5.0

	- Stable Hyper 0.13 and Tokio 0.2 support

	## v0.5.0-alpha.6

	- Fix a bug in ALPN resolving.

	## v0.5.0-alpha.5

	- And down to async-std's new `ToSocketAddrs` resolver

	## v0.5.0-alpha.4

	- Switch to Hyper's GaiResolver to go around of a bug in the latest nightly.

	## v0.5.0-alpha.1

	- Update to `std::future` and async/await, requiring a nightly compiler for now.

	## v0.4.1

	- Fix token_client example not building due to unresolvable `as_ref()`. [#35](https://github.com/pimeys/a2/pull/35)
	- Move indoc to dev-dependencies so that crates depending on us don't need it. [#36](https://github.com/pimeys/a2/pull/36)
	- Move pretty_env_logger to dev-dependencies, it's not useful to crates depending on us. [#37](https://github.com/pimeys/a2/pull/37)
	- Remove unused tokio-io dependency. [#38](https://github.com/pimeys/a2/pull/38)

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
