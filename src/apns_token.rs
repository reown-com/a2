use btls::server_keys::LocalKeyPair;
use btls::jose_jws::{sign_jws, JsonNode};
use std::convert::From;
use btls::error::KeyReadError;
use time::get_time;
use std::collections::BTreeMap;
use std::io::Read;

const SIG_ECDSA_SHA256: u16 = 0x0403;

pub struct ApnsToken {
    signature: Option<String>,
    issued_at: Option<i64>,
    key_id: String,
    team_id: String,
    secret: LocalKeyPair,
}

#[derive(Debug)]
pub enum ApnsTokenError {
    SignError,
    KeyParseError(String),
    KeyOpenError(String),
    KeyReadError(String),
    KeyGenerationError,
    KeyError,
}


impl From<KeyReadError> for ApnsTokenError {
    fn from(e: KeyReadError) -> ApnsTokenError {
        match e {
            KeyReadError::ParseError(e, _) => ApnsTokenError::KeyParseError(e),
            KeyReadError::OpenError(e, _) => ApnsTokenError::KeyOpenError(e),
            KeyReadError::ReadError(e, _) => ApnsTokenError::KeyReadError(e),
            KeyReadError::KeyGenerationFailed => ApnsTokenError::KeyGenerationError,
            _ => ApnsTokenError::KeyError,
        }
    }
}

impl ApnsToken {
    pub fn new<S: Into<String>, R: Read>(pk_der: &mut R, key_id: S, team_id: S) -> Result<ApnsToken, ApnsTokenError> {
        let mut token = ApnsToken {
            signature: None,
            issued_at: None,
            key_id: key_id.into(),
            team_id: team_id.into(),
            secret: LocalKeyPair::new(pk_der, "apns_private_key")?,
        };

        match token.renew() {
            Err(e) => Err(e),
            _ => Ok(token),
        }
    }

    pub fn signature(&self) -> String {
        match self.signature {
            Some(ref sig) => sig.to_string(),
            None => "".to_string()
        }
    }

    pub fn renew(&mut self) -> Result<(), ApnsTokenError> {
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
            _ => Err(ApnsTokenError::SignError)
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(issued_at) = self.issued_at {
            (get_time().sec - issued_at) > 3600
        } else {
            true
        }
    }
}
