use std::io::Read;
use serde_json;
use base64::encode;
use crate::error::Error;
use std::{sync::RwLock, time::{Duration, SystemTime, UNIX_EPOCH}};

use openssl::{
    ec::EcKey,
    pkey::{PKey, Private},
    hash::MessageDigest,
    sign::Signer as SslSigner,
};

#[derive(Debug)]
struct Signature {
    key: String,
    issued_at: i64,
}

/// For signing requests when using token-based authentication. Re-uses the same
/// signature for a certain amount of time.
pub struct Signer {
    signature: RwLock<Signature>,
    key_id: String,
    team_id: String,
    secret: PKey<Private>,
    expire_after_s: Duration,
}

#[derive(Serialize, Deserialize)]
enum JwtAlg {
    ES256,
}

#[derive(Serialize, Deserialize)]
struct JwtHeader<'a> {
    alg: JwtAlg,
    kid: &'a str,
}

#[derive(Serialize, Deserialize)]
struct JwtPayload<'a> {
    iss: &'a str,
    iat: i64,
}

impl Signer {
    /// Creates a signer with a pkcs8 private key, APNs key id and team id.
    /// Can fail if the key is not valid or there is a problem with system OpenSSL.
    pub fn new<S, T, R>(
        mut pk_pem: R,
        key_id: S,
        team_id: T,
        signature_ttl: Duration,
    ) -> Result<Signer, Error>
    where
        S: Into<String>,
        T: Into<String>,
        R: Read,
    {
        let key_id: String = key_id.into();
        let team_id: String = team_id.into();

        let mut pem_key: Vec<u8> = Vec::new();
        pk_pem.read_to_end(&mut pem_key)?;

        let ec_key = EcKey::private_key_from_pem(&pem_key)?;

        let issued_at = get_time();
        let secret = PKey::from_ec_key(ec_key)?;

        let signature = RwLock::new(Signature {
            key: Self::create_signature(&secret, &key_id, &team_id, issued_at)?,
            issued_at: issued_at,
        });

        let signer = Signer {
            signature: signature,
            key_id: key_id,
            team_id: team_id,
            secret: secret,
            expire_after_s: signature_ttl,
        };

        Ok(signer)
    }

    /// Take a signature out for usage. Automatically renews the signature
    /// if it's older than the expiration time.
    pub fn with_signature<F, T>(&self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&str) -> T,
    {
        if self.is_expired() {
            self.renew()?;
        }

        let signature = self.signature.read().unwrap();

        trace!(
            "Signer::with_signature found signature for {}/{} valid for {}s",
            self.key_id,
            self.team_id,
            self.expire_after_s.as_secs(),
        );

        Ok(f(&signature.key))
    }

    fn create_signature(
        secret: &PKey<Private>,
        key_id: &str,
        team_id: &str,
        issued_at: i64,
    ) -> Result<String, Error> {
        let headers = JwtHeader {
            alg: JwtAlg::ES256,
            kid: key_id,
        };

        let payload = JwtPayload {
            iss: team_id,
            iat: issued_at,
        };

        let encoded_header = encode(&serde_json::to_string(&headers)?);
        let encoded_payload = encode(&serde_json::to_string(&payload)?);
        let signing_input = format!("{}.{}", encoded_header, encoded_payload);

        let mut signer = SslSigner::new(MessageDigest::sha256(), secret)?;
        signer.update(signing_input.as_bytes())?;

        let signature_payload = signer.sign_to_vec()?;

        Ok(format!("{}.{}", signing_input, encode(&signature_payload)))
    }

    fn renew(&self) -> Result<(), Error> {
        let issued_at = get_time();

        trace!(
            "Signer::renew for k_id {} t_id {} issued {} valid for {}s",
            self.key_id,
            self.team_id,
            issued_at,
            self.expire_after_s.as_secs(),
        );

        let mut signature = self.signature.write().unwrap();

        *signature = Signature {
            key: Self::create_signature(&self.secret, &self.key_id, &self.team_id, issued_at)?,
            issued_at: issued_at,
        };

        Ok(())
    }

    fn is_expired(&self) -> bool {
        let sig = self.signature.read().unwrap();
        let expiry = get_time() - sig.issued_at;
        expiry >= self.expire_after_s.as_secs() as i64
    }
}

fn get_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIVATE_KEY: &'static str = indoc!(
        "-----BEGIN PRIVATE KEY-----
        MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg8g/n6j9roKvnUkwu
        lCEIvbDqlUhA5FOzcakkG90E8L+hRANCAATKS2ZExEybUvchRDuKBftotMwVEus3
        jDwmlD1Gg0yJt1e38djFwsxsfr5q2hv0Rj9fTEqAPr8H7mGm0wKxZ7iQ
        -----END PRIVATE KEY-----"
    );

    #[test]
    fn test_signature_caching() {
        let signer = Signer::new(PRIVATE_KEY.as_bytes(), "89AFRD1X22", "ASDFQWERTY", Duration::from_secs(100)).unwrap();

        let mut sig1 = String::new();
        signer.with_signature(|sig| sig1.push_str(sig)).unwrap();

        let mut sig2 = String::new();
        signer.with_signature(|sig| sig2.push_str(sig)).unwrap();

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_signature_without_caching() {
        let signer = Signer::new(PRIVATE_KEY.as_bytes(), "89AFRD1X22", "ASDFQWERTY", Duration::from_secs(0)).unwrap();

        let mut sig1 = String::new();
        signer.with_signature(|sig| sig1.push_str(sig)).unwrap();

        let mut sig2 = String::new();
        signer.with_signature(|sig| sig2.push_str(sig)).unwrap();

        assert_ne!(sig1, sig2);
    }
}
