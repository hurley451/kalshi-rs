mod transport_support;
use base64::Engine;
use kalshi_client::*;
use rsa::{
    RsaPrivateKey, pkcs1::EncodeRsaPrivateKey, pkcs8::EncodePrivateKey, signature::Verifier,
};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinSet,
};
use transport_support::*;

enum Key {
    Rsa(RsaPrivateKey),
    Ed(ed25519_dalek::SigningKey),
}
fn keys() -> Vec<(KalshiAuth, Key)> {
    let rsa = RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    let ed = ed25519_dalek::SigningKey::from_bytes(&[9; 32]);
    vec![
        (
            KalshiAuth {
                api_key_id: "FAKE_RSA1_KEY".into(),
                private_key_pem: rsa.to_pkcs1_pem(Default::default()).unwrap().to_string(),
            },
            Key::Rsa(rsa.clone()),
        ),
        (
            KalshiAuth {
                api_key_id: "FAKE_RSA8_KEY".into(),
                private_key_pem: rsa.to_pkcs8_pem(Default::default()).unwrap().to_string(),
            },
            Key::Rsa(rsa),
        ),
        (
            KalshiAuth {
                api_key_id: "FAKE_ED_KEY".into(),
                private_key_pem: ed.to_pkcs8_pem(Default::default()).unwrap().to_string(),
            },
            Key::Ed(ed),
        ),
    ]
}
fn verify(log: &str, key: &Key, auth: &KalshiAuth) -> String {
    let headers: std::collections::BTreeMap<_, _> = log
        .lines()
        .skip(1)
        .filter_map(|l| l.split_once(": "))
        .map(|(k, v)| (k.to_ascii_lowercase(), v))
        .collect();
    assert_eq!(headers["kalshi-access-key"], auth.api_key_id);
    assert_ne!(headers["kalshi-access-signature"], "FORGED");
    let first: Vec<_> = log.lines().next().unwrap().split(' ').collect();
    let preimage = build_signing_preimage(headers["kalshi-access-timestamp"], first[0], first[1]);
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(headers["kalshi-access-signature"])
        .unwrap();
    match key {
        Key::Rsa(key) => rsa::pss::VerifyingKey::<sha2::Sha256>::new(key.to_public_key())
            .verify(
                preimage.as_bytes(),
                &rsa::pss::Signature::try_from(bytes.as_slice()).unwrap(),
            )
            .unwrap(),
        Key::Ed(key) => key
            .verifying_key()
            .verify_strict(
                preimage.as_bytes(),
                &ed25519_dalek::Signature::from_slice(&bytes).unwrap(),
            )
            .unwrap(),
    }
    headers["kalshi-access-timestamp"].to_string()
}

#[tokio::test]
async fn all_key_formats_sign_each_retry_fresh_and_override_forged_headers() {
    for (auth, key) in keys() {
        let server = Server::new(vec![
            Reply::json(503, json!({})).header("Retry-After", "1"),
            Reply::json(200, json!({})),
        ])
        .await;
        let client = KalshiClient::builder(server.environment)
            .auth(auth.clone())
            .http_builder(|http| http.no_proxy())
            .transport_options(fast_retry())
            .build()
            .unwrap();
        let mut request = KalshiRequest::default();
        request
            .headers
            .insert("kalshi-access-signature".into(), "FORGED".into());
        request
            .headers
            .insert("kalshi-access-key".into(), "FORGED".into());
        request
            .query
            .push(("q".into(), QueryValue::from("untrusted query")));
        client.request("GetExchangeStatus", request).await.unwrap();
        let logs = server.logs.lock().unwrap().clone();
        assert_eq!(logs.len(), 2);
        let a = verify(&logs[0], &key, &auth);
        let b = verify(&logs[1], &key, &auth);
        assert_ne!(a, b);
        server.close().await;
    }
}

#[tokio::test]
async fn authenticated_redirects_never_forward_rsa_or_ed_credentials() {
    let target = Server::new(vec![Reply::json(200, json!({}))]).await;
    for (auth, key) in keys() {
        let location = format!("{}/stolen", target.environment.rest_base_url);
        let server = Server::new(vec![
            Reply::bytes(307, b"redirect".to_vec()).header("Location", &location),
        ])
        .await;
        let client = KalshiClient::builder(server.environment)
            .auth(auth.clone())
            .http_builder(|http| {
                http.no_proxy()
                    .redirect(reqwest::redirect::Policy::limited(10))
            })
            .build()
            .unwrap();
        assert!(matches!(
            client
                .request("GetExchangeStatus", KalshiRequest::default())
                .await,
            Err(KalshiError::Api(_))
        ));
        verify(&server.logs.lock().unwrap()[0], &key, &auth);
        assert_eq!(server.count(), 1);
        assert_eq!(target.count(), 0);
        server.close().await;
    }
    target.close().await;
}

async fn tls_server(
    location: String,
) -> (
    KalshiEnvironment,
    CancellationToken,
    tokio::task::JoinHandle<()>,
    Arc<AtomicUsize>,
) {
    use tokio_rustls::rustls::{ServerConfig, pki_types::PrivatePkcs8KeyDer};
    let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let key = PrivatePkcs8KeyDer::from(certificate.signing_key.serialize_der());
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate.cert.der().clone()], key.into())
        .unwrap();
    let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let environment = KalshiEnvironment {
        name: "tls-loopback",
        rest_base_url: Box::leak(
            format!(
                "https://localhost:{}/trade-api/v2",
                listener.local_addr().unwrap().port()
            )
            .into_boxed_str(),
        ),
        websocket_url: "wss://unused",
    };
    let count = Arc::new(AtomicUsize::new(0));
    let stop = CancellationToken::new();
    let (seen, shutdown) = (count.clone(), stop.clone());
    let task = tokio::spawn(async move {
        let mut tasks = JoinSet::new();
        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                accepted = listener.accept() => {
                    let (socket,_) = accepted.unwrap(); seen.fetch_add(1,Ordering::SeqCst); let acceptor = acceptor.clone(); let location = location.clone();
                    tasks.spawn(async move {
                        if let Ok(mut socket) = acceptor.accept(socket).await {
                            let mut buffer = [0;8192]; if socket.read(&mut buffer).await.is_ok() {
                                let _ = socket.write_all(format!("HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").as_bytes()).await;
                            }
                        }
                    });
                }
            }
        }
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
    });
    (environment, stop, task, count)
}

#[tokio::test]
async fn real_https_downgrade_is_refused_and_bad_certificate_is_not_retried() {
    let target = Server::new(vec![Reply::json(200, json!({}))]).await;
    let (environment, stop, task, count) =
        tls_server(format!("{}/stolen", target.environment.rest_base_url)).await;
    let client = KalshiClient::builder(environment)
        .http_builder(|http| {
            http.no_proxy()
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::limited(10))
        })
        .transport_options(fast_retry())
        .build()
        .unwrap();
    assert!(matches!(get(&client).await, Err(KalshiError::Api(_))));
    assert_eq!(target.count(), 0);
    assert_eq!(count.load(Ordering::SeqCst), 1);
    stop.cancel();
    task.await.unwrap();
    let (environment, stop, task, count) = tls_server("http://example.invalid/".into()).await;
    let client = KalshiClient::builder(environment)
        .http_builder(|http| http.no_proxy())
        .transport_options(fast_retry())
        .build()
        .unwrap();
    assert!(matches!(get(&client).await, Err(KalshiError::Http(_))));
    assert_eq!(count.load(Ordering::SeqCst), 1);
    stop.cancel();
    task.await.unwrap();
    target.close().await;
}

#[tokio::test]
async fn invalid_signing_credentials_never_dispatch_and_config_debug_is_redacted() {
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    let client = KalshiClient::builder(server.environment)
        .auth(KalshiAuth {
            api_key_id: "FAKE_SECRET_ID".into(),
            private_key_pem: "FAKE_SECRET_BAD_PEM".into(),
        })
        .build()
        .unwrap();
    let error = client
        .request("GetExchangeStatus", KalshiRequest::default())
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error} {client:?}").contains("FAKE_SECRET"));
    assert_eq!(server.count(), 0);
    let token = CancellationToken::new();
    token.cancel();
    assert!(matches!(
        client
            .request_with_options(
                "GetExchangeStatus",
                KalshiRequest::default(),
                RequestOptions {
                    cancellation: Some(token),
                    ..Default::default()
                }
            )
            .await,
        Err(KalshiError::Cancelled)
    ));
    assert_eq!(server.count(), 0);
    server.close().await;
}
