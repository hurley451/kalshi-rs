use std::collections::BTreeMap;

use base64::Engine;
use rsa::RsaPrivateKey;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pss::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding, Signer};
use sha2::Sha256;

use crate::error::KalshiError;

#[derive(Clone)]
pub struct KalshiAuth {
    pub api_key_id: String,
    pub private_key_pem: String,
}

impl std::fmt::Debug for KalshiAuth {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KalshiAuth")
            .field("api_key_id", &"[REDACTED]")
            .field("private_key_pem", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SignedKalshiRequest {
    pub timestamp_ms: String,
    pub signature: String,
    pub headers: BTreeMap<String, String>,
}

impl std::fmt::Debug for SignedKalshiRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SignedKalshiRequest")
            .field("timestamp_ms", &self.timestamp_ms)
            .field("signature", &"[REDACTED]")
            .field("headers", &"[REDACTED]")
            .finish()
    }
}

pub fn build_signing_preimage(timestamp_ms: &str, method: &str, path_and_query: &str) -> String {
    let canonical_path = path_and_query.split('?').next().unwrap_or(path_and_query);
    format!(
        "{}{}{}",
        timestamp_ms,
        method.to_uppercase(),
        canonical_path
    )
}

pub fn sign_kalshi_request(
    auth: &KalshiAuth,
    method: &str,
    path_and_query: &str,
    timestamp_ms: &str,
) -> Result<SignedKalshiRequest, KalshiError> {
    if auth.api_key_id.is_empty()
        || !auth
            .api_key_id
            .bytes()
            .all(|byte| (0x21..=0x7e).contains(&byte))
        || timestamp_ms.is_empty()
        || !timestamp_ms.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(KalshiError::InvalidSigningCredentials);
    }
    let message = build_signing_preimage(timestamp_ms, method, path_and_query);
    let signature = if let Ok(key) = RsaPrivateKey::from_pkcs8_pem(&auth.private_key_pem)
        .or_else(|_| RsaPrivateKey::from_pkcs1_pem(&auth.private_key_pem))
    {
        SigningKey::<Sha256>::new(key)
            .try_sign_with_rng(&mut rand_core::OsRng, message.as_bytes())
            .map_err(|_| KalshiError::SigningFailed)?
            .to_bytes()
            .to_vec()
    } else if let Ok(key) = ed25519_dalek::SigningKey::from_pkcs8_pem(&auth.private_key_pem) {
        key.try_sign(message.as_bytes())
            .map_err(|_| KalshiError::SigningFailed)?
            .to_bytes()
            .to_vec()
    } else {
        return Err(KalshiError::InvalidSigningCredentials);
    };
    let signature = base64::engine::general_purpose::STANDARD.encode(signature);

    let mut headers = BTreeMap::new();
    headers.insert("KALSHI-ACCESS-KEY".to_string(), auth.api_key_id.clone());
    headers.insert(
        "KALSHI-ACCESS-TIMESTAMP".to_string(),
        timestamp_ms.to_string(),
    );
    headers.insert("KALSHI-ACCESS-SIGNATURE".to_string(), signature.clone());

    Ok(SignedKalshiRequest {
        timestamp_ms: timestamp_ms.to_string(),
        signature,
        headers,
    })
}
