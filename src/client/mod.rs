//! The APNs connection handling modules. `TokenClient` for connections using
//! JWT authentication, `CertificateClient` when using a sertificate and a
//! private key to authenticate. `ProviderResponse` handles responses and maps
//! the results to `APNSStatus` and `APNSError`.

mod certificate;
mod response;
mod headers;
mod error;
mod token;

pub use self::token::TokenClient;
pub use self::certificate::CertificateClient;
pub use self::response::{ProviderResponse, APNSStatus, APNSError};

static DEVELOPMENT: &'static str = "api.development.push.apple.com";
static PRODUCTION: &'static str = "api.push.apple.com";
