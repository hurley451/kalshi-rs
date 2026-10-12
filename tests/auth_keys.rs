mod typed_support;

use base64::Engine;
use ed25519_dalek::{SigningKey as EdKey, Verifier};
use kalshi_client::{KalshiAuth, KalshiError, sign_kalshi_request};
use rsa::{RsaPrivateKey, pkcs1::EncodeRsaPrivateKey, pkcs8::EncodePrivateKey};
use sha2::{Digest, Sha256};

fn auth(pem: String) -> KalshiAuth {
    KalshiAuth {
        api_key_id: "fixture-key".into(),
        private_key_pem: pem,
    }
}

fn verify_rsa(key: &RsaPrivateKey, message: &[u8], encoded: &str) -> bool {
    use rsa::signature::Verifier;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap();
    rsa::pss::VerifyingKey::<Sha256>::new(key.to_public_key())
        .verify(
            message,
            &rsa::pss::Signature::try_from(bytes.as_slice()).unwrap(),
        )
        .is_ok()
}

#[test]
fn rsa_formats_preserve_canonicalization_and_salt() {
    let key = RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    for pem in [
        key.to_pkcs1_pem(Default::default()).unwrap().to_string(),
        key.to_pkcs8_pem(Default::default()).unwrap().to_string(),
    ] {
        let auth = auth(pem);
        let signed = sign_kalshi_request(
            &auth,
            "get",
            "/trade-api/v2/portfolio/balance?subaccount=1",
            "1790000000000",
        )
        .unwrap();
        let message = b"1790000000000GET/trade-api/v2/portfolio/balance";
        assert!(verify_rsa(&key, message, &signed.signature));
        for wrong in [
            b"1790000000001GET/trade-api/v2/portfolio/balance".as_slice(),
            b"1790000000000POST/trade-api/v2/portfolio/balance",
            b"1790000000000GET/trade-api/v2/portfolio/fills",
            b"1790000000000GET/trade-api/v2/portfolio/balance?subaccount=1",
        ] {
            assert!(!verify_rsa(&key, wrong, &signed.signature));
        }
        let ws = sign_kalshi_request(&auth, "GET", "/trade-api/ws/v2?ignored=1", "1790000000000")
            .unwrap();
        assert!(verify_rsa(
            &key,
            b"1790000000000GET/trade-api/ws/v2",
            &ws.signature
        ));
        assert_eq!(signed.headers.len(), 3);
    }
}

#[test]
fn ed25519_signs_without_prehashing() {
    let key = EdKey::from_bytes(&[42; 32]);
    let auth = auth(key.to_pkcs8_pem(Default::default()).unwrap().to_string());
    let signed = sign_kalshi_request(
        &auth,
        "get",
        "/trade-api/v2/portfolio/balance?subaccount=1",
        "1790000000000",
    )
    .unwrap();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&signed.signature)
        .unwrap();
    assert_eq!(bytes.len(), 64);
    let signature = ed25519_dalek::Signature::from_slice(&bytes).unwrap();
    let message = b"1790000000000GET/trade-api/v2/portfolio/balance";
    assert!(key.verifying_key().verify(message, &signature).is_ok());
    for wrong in [
        Sha256::digest(message).as_slice(),
        b"1790000000001GET/trade-api/v2/portfolio/balance",
        b"1790000000000POST/trade-api/v2/portfolio/balance",
        b"1790000000000GET/trade-api/v2/portfolio/fills",
        b"1790000000000GET/trade-api/v2/portfolio/balance?subaccount=1",
    ] {
        assert!(key.verifying_key().verify(wrong, &signature).is_err());
    }
    assert!(
        EdKey::from_bytes(&[43; 32])
            .verifying_key()
            .verify(message, &signature)
            .is_err()
    );
    assert_eq!(
        signed.signature,
        sign_kalshi_request(
            &auth,
            "GET",
            "/trade-api/v2/portfolio/balance?subaccount=2",
            "1790000000000"
        )
        .unwrap()
        .signature
    );
    let ws = sign_kalshi_request(&auth, "GET", "/trade-api/ws/v2", "1790000000000").unwrap();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(ws.signature)
        .unwrap();
    assert!(
        key.verifying_key()
            .verify(
                b"1790000000000GET/trade-api/ws/v2",
                &ed25519_dalek::Signature::from_slice(&bytes).unwrap()
            )
            .is_ok()
    );
}

#[test]
fn invalid_credentials_are_sanitized_and_small_rsa_does_not_panic() {
    let small = RsaPrivateKey::new(&mut rand_core::OsRng, 512).unwrap();
    let small_auth = auth(small.to_pkcs8_pem(Default::default()).unwrap().to_string());
    assert!(matches!(
        sign_kalshi_request(&small_auth, "GET", "/trade-api/v2/exchange/status", "1"),
        Err(KalshiError::SigningFailed)
    ));
    for pem in [
        "SECRET_PRIVATE_KEY",
        "-----BEGIN ENCRYPTED PRIVATE KEY-----\nSECRET_PRIVATE_KEY\n-----END ENCRYPTED PRIVATE KEY-----",
    ] {
        let error = sign_kalshi_request(
            &auth(pem.into()),
            "GET",
            "/trade-api/v2/exchange/status",
            "1",
        )
        .unwrap_err();
        assert!(!format!("{error:?} {error}").contains("SECRET_PRIVATE_KEY"));
        assert!(std::error::Error::source(&error).is_none());
    }
    let key = EdKey::from_bytes(&[42; 32]);
    let mut auth = auth(key.to_pkcs8_pem(Default::default()).unwrap().to_string());
    for id in ["", "key\nSECRET", "white space", "\u{80}"] {
        auth.api_key_id = id.into();
        assert!(matches!(
            sign_kalshi_request(&auth, "GET", "/", "1"),
            Err(KalshiError::InvalidSigningCredentials)
        ));
    }
    auth.api_key_id = "fixture-key".into();
    for timestamp in ["", "-1", "1.5", "secret\n"] {
        assert!(matches!(
            sign_kalshi_request(&auth, "GET", "/", timestamp),
            Err(KalshiError::InvalidSigningCredentials)
        ));
    }
}

#[tokio::test]
async fn all_key_formats_sign_real_named_http_calls() {
    let rsa = RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    let ed = EdKey::from_bytes(&[42; 32]);
    for (pem, is_ed) in [
        (
            rsa.to_pkcs1_pem(Default::default()).unwrap().to_string(),
            false,
        ),
        (
            rsa.to_pkcs8_pem(Default::default()).unwrap().to_string(),
            false,
        ),
        (
            ed.to_pkcs8_pem(Default::default()).unwrap().to_string(),
            true,
        ),
    ] {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
        let (client, server) = typed_support::server(
            vec![(200, fixture["GetBalance"]["response"].clone())],
            Some(auth(pem)),
        );
        client
            .portfolio()
            .get_balance(&kalshi_client::models::GetBalanceParameters::default())
            .await
            .unwrap();
        let requests = server.join().unwrap();
        let headers: std::collections::BTreeMap<_, _> = requests[0]
            .lines()
            .filter_map(|line| {
                line.split_once(": ")
                    .map(|(name, value)| (name.to_ascii_lowercase(), value))
            })
            .collect();
        assert_eq!(headers["kalshi-access-key"], "fixture-key");
        let message = format!(
            "{}GET/trade-api/v2/portfolio/balance",
            headers["kalshi-access-timestamp"]
        );
        if is_ed {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(headers["kalshi-access-signature"])
                .unwrap();
            ed.verifying_key()
                .verify(
                    message.as_bytes(),
                    &ed25519_dalek::Signature::from_slice(&bytes).unwrap(),
                )
                .unwrap();
        } else {
            assert!(verify_rsa(
                &rsa,
                message.as_bytes(),
                headers["kalshi-access-signature"]
            ));
        }
    }
}
