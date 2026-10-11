use std::collections::BTreeMap;

use base64::Engine;
use rsa::RsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pss::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use sha2::Sha256;

use crate::error::KalshiError;

#[derive(Debug, Clone)]
pub struct KalshiAuth {
    pub api_key_id: String,
    pub private_key_pem: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedKalshiRequest {
    pub timestamp_ms: String,
    pub signature: String,
    pub headers: BTreeMap<String, String>,
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
    let key = RsaPrivateKey::from_pkcs8_pem(&auth.private_key_pem)
        .map_err(|error| KalshiError::RsaKey(error.to_string()))?;
    let signing_key = SigningKey::<Sha256>::new(key);
    let mut rng = rand_core::OsRng;
    let signature = signing_key.sign_with_rng(
        &mut rng,
        build_signing_preimage(timestamp_ms, method, path_and_query).as_bytes(),
    );
    let signature = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());

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
