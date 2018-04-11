//! # A2
//!
//! A2 is an asynchronous client to Apple push notification service. It
//! provides a typesafe way to generate correct requests and maps responses into
//! corresponding types. It supports both, the certificate and token based
//! authentication.
//!
//! To create a connection it is required to have either a PKCS12 database file
//! including a valid certificate and private key, and a password, or a private
//! key in PKCS8 PEM format with the corresponding team and key ids. All of
//! these should be available from Xcode or your Apple developer account.
//!
//! The library is meant to be high performace and typesafe. It is also meant to
//! be used together with [Tokio framework](https://tokio.rs) in an asynchronous
//! event loop.
//!
//! ## Payload
//!
//! Building the notification payload should be done with the corresponding builders:
//!
//! * [PlainNotificationBuilder](request/notification/struct.PlainNotificationBuilder.html) for text only messages.
//! * [SilentNotificationBuilder](request/notification/struct.SilentNotificationBuilder.html) for silent notifications with custom data.
//! * [LocalizedNotificationBuilder](request/notification/struct.LocalizedNotificationBuilder.html) for localized rich notifications.
//!
//! ## The client
//!
//! The [asynchronous client](client/struct.Client.html), works either with
//! [certificate](client/struct.Client.html#method.certificate) or
//! [token](client/struct.Client.html#method.token) authentication.
//!
//! ## Example sending a plain notification using token authentication:
//!
//! ```no_run
//! extern crate tokio_core;
//! extern crate a2;
//!
//! use a2::request::notification::{PlainNotificationBuilder, NotificationBuilder};
//! use a2::client::{Client, Endpoint};
//! use std::fs::File;
//!
//! fn main() {
//!     let mut core = tokio_core::reactor::Core::new().unwrap();
//!     let handle = core.handle();
//!
//!     let mut builder = PlainNotificationBuilder::new("Hi there");
//!     builder.set_badge(420);
//!     builder.set_category("cat1");
//!     builder.set_sound("ping.flac");
//!     let payload = builder.build("device-token-from-the-user", Default::default());
//!
//!     let mut file = File::open("/path/to/private_key.p8").unwrap();
//!
//!     let client = Client::token(&mut file, "KEY_ID", "TEAM_ID", &handle, Endpoint::Production).unwrap();
//!     let work = client.send(payload);
//!
//!     match core.run(work) {
//!         Ok(response) => println!("Success: {:?}", response),
//!         Err(error) => println!("Error: {:?}", error),
//!     };
//! }
//! ```
//!
//! ## Example sending a silent notification with custom data using certificate authentication:
//!
//! ```no_run
//! #[macro_use] extern crate serde_derive;
//! extern crate serde;
//! extern crate tokio_core;
//! extern crate a2;
//!
//! use a2::request::notification::{SilentNotificationBuilder, NotificationBuilder};
//! use a2::client::{Client, Endpoint};
//! use std::fs::File;
//!
//! #[derive(Serialize, Debug)]
//! struct CorporateData {
//!     tracking_code: &'static str,
//!     is_paying_user: bool,
//! }
//!
//! fn main() {
//!     let mut core = tokio_core::reactor::Core::new().unwrap();
//!     let handle = core.handle();
//!
//!     let tracking_data = CorporateData {
//!         tracking_code: "999-212-UF-NSA",
//!         is_paying_user: false,
//!     };
//!
//!     let mut payload = SilentNotificationBuilder::new()
//!         .build("device-token-from-the-user", Default::default());
//!
//!     payload.add_custom_data("apns_gmbh", &tracking_data).unwrap();
//!
//!     let mut file = File::open("/path/to/cert_db.p12").unwrap();
//!
//!     let client = Client::certificate(
//!         &mut file,
//!         "Correct Horse Battery Stable",
//!         &handle,
//!         Endpoint::Production).unwrap();
//!
//!     let work = client.send(payload);
//!
//!     match core.run(work) {
//!         Ok(response) => println!("Success: {:?}", response),
//!         Err(error) => println!("Error: {:?}", error),
//!     };
//! }
//! ```

extern crate base64;
extern crate chrono;
extern crate crossbeam;
extern crate erased_serde;
extern crate futures;
extern crate hyper;
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_rustls;
extern crate tokio_service;
extern crate tokio_timer;
extern crate webpki;
extern crate webpki_roots;

pub mod request;
pub mod error;
pub mod response;
pub mod client;
mod signer;
mod stream;
mod alpn;
