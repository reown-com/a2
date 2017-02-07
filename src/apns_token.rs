//! A module for APNS JWT token management.

use btls::server_keys::LocalKeyPair;
use btls::jose_jws::{sign_jws, JsonNode};
use std::convert::From;
use btls::error::KeyReadError;
use time::get_time;
use std::collections::BTreeMap;
use std::io::Read;

const SIG_ECDSA_SHA256: u16 = 0x0403;

pub struct APNSToken {
    signature: Option<String>,
    issued_at: Option<i64>,
    key_id: String,
    team_id: String,
    secret: LocalKeyPair,
}

#[derive(Debug)]
pub enum APNSTokenError {
    SignError,
    KeyParseError(String),
    KeyOpenError(String),
    KeyReadError(String),
    KeyGenerationError,
    KeyError,
}

impl From<KeyReadError> for APNSTokenError {
    fn from(e: KeyReadError) -> APNSTokenError {
        match e {
            KeyReadError::ParseError(e, _) => APNSTokenError::KeyParseError(e),
            KeyReadError::OpenError(e, _) => APNSTokenError::KeyOpenError(e),
            KeyReadError::ReadError(e, _) => APNSTokenError::KeyReadError(e),
            KeyReadError::KeyGenerationFailed => APNSTokenError::KeyGenerationError,
            _ => APNSTokenError::KeyError,
        }
    }
}

impl APNSToken {
    /// Create a new APNSToken.
    ///
    /// A generator for JWT tokens when using the token-based authentication in APNs.
    /// The private key should be in DER binary format and can be provided in any
    /// format implementing the Read trait.
    ///
    /// # Example
    /// ```no_run
    /// # extern crate apns2;
    /// # fn main() {
    /// use apns2::apns_token::APNSToken;
    /// use std::fs::File;
    ///
    /// let der_file = File::open("/path/to/apns.der").unwrap();
    /// APNSToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
    /// # }
    /// ```
    pub fn new<S,R>(mut pk_der: R, key_id: S, team_id: S) -> Result<APNSToken, APNSTokenError>
        where S: Into<String>, R: Read {

        let mut token = APNSToken {
            signature: None,
            issued_at: None,
            key_id: key_id.into(),
            team_id: team_id.into(),
            secret: LocalKeyPair::new(&mut pk_der, "apns_private_key")?,
        };

        match token.renew() {
            Err(e) => Err(e),
            _ => Ok(token),
        }
    }

    /// Generates an authentication signature.
    ///
    /// # Example
    /// ```no_run
    /// # extern crate apns2;
    /// # fn main() {
    /// use apns2::apns_token::APNSToken;
    /// use std::fs::File;
    ///
    /// let der_file = File::open("/path/to/apns.der").unwrap();
    /// let apns_token = APNSToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
    /// let signature = apns_token.signature();
    /// # }
    /// ```
    pub fn signature(&self) -> &str {
        match self.signature {
            Some(ref sig) => sig,
            None => ""
        }
    }

    /// Sets a new timestamp for the token. APNs tokens are valid for 60 minutes until
    /// they need to be renewed.
    ///
    /// # Example
    /// ```no_run
    /// # extern crate apns2;
    /// # fn main() {
    /// use apns2::apns_token::APNSToken;
    /// use std::fs::File;
    ///
    /// let der_file = File::open("/path/to/apns.der").unwrap();
    /// let mut apns_token = APNSToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
    /// apns_token.renew().unwrap();
    /// # }
    /// ```
    pub fn renew(&mut self) -> Result<(), APNSTokenError> {
        let issued_at = get_time().sec;

        let mut headers: BTreeMap<String, JsonNode> = BTreeMap::new();
        headers.insert("alg".to_string(), JsonNode::String("ES256".to_string()));
        headers.insert("kid".to_string(), JsonNode::String(self.key_id.to_string()));

        let mut payload: BTreeMap<String, JsonNode> = BTreeMap::new();
        payload.insert("iss".to_string(), JsonNode::String(self.team_id.to_string()));
        payload.insert("iat".to_string(), JsonNode::Number(issued_at));

        let jwt_headers = JsonNode::Dictionary(headers);
        let jwt_payload = JsonNode::Dictionary(payload).serialize();

        match sign_jws(&jwt_headers, jwt_payload.as_bytes(), &self.secret, SIG_ECDSA_SHA256).read() {
            Ok(Ok(token)) => {
                self.signature = Some(token.to_compact());
                self.issued_at = Some(issued_at);
                Ok(())
            }
            _ => Err(APNSTokenError::SignError)
        }
    }

    /// Info about the token expiration. If older than one hour, returns true.
    ///
    /// # Example
    /// ```no_run
    /// # extern crate apns2;
    /// # fn main() {
    /// use apns2::apns_token::APNSToken;
    /// use std::fs::File;
    ///
    /// let der_file = File::open("/path/to/apns.der").unwrap();
    /// let mut apns_token = APNSToken::new(der_file, "TEAMID1234", "KEYID12345").unwrap();
    /// if apns_token.is_expired() {
    ///     apns_token.renew();
    /// }
    /// # }
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(issued_at) = self.issued_at {
            (get_time().sec - issued_at) > 3600
        } else {
            true
        }
    }
}
