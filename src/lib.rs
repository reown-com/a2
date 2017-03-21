//! A library for sending push notifications to iOS devices using Apple's APNS
//! API. Supports certificate based authentication through
//! `apns2::client::CertificateClient` and JWT token based authentication
//! through `apns2::client::TokenClient`.
//!
//! If using JWT tokens for authentication, `apns2::apns_token::ApnsToken` can
//! be used for generating and holding tokens, allowing re-use and renewal.
//!
//! The `apns::client::ProviderResponse` does not block until using the
//! `recv_timeout`. The request is handled in another thread and the response is
//! sent through a channel to the thread calling the method.

extern crate solicit;
extern crate rustc_serialize;
extern crate time;
extern crate openssl;
extern crate btls;

pub mod client;
pub mod notification;
pub mod payload;
pub mod apns_token;
