mod transport_support;
use async_compression::tokio::bufread::{BrotliEncoder, GzipEncoder, ZlibEncoder, ZstdEncoder};
use kalshi_client::*;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use transport_support::*;

async fn encode(name: &str, bytes: &[u8]) -> Vec<u8> {
    let reader = BufReader::new(bytes);
    let mut encoder: Box<dyn AsyncRead + Unpin> = match name {
        "gzip" => Box::new(GzipEncoder::new(reader)),
        "deflate" => Box::new(ZlibEncoder::new(reader)),
        "br" => Box::new(BrotliEncoder::new(reader)),
        "zstd" => Box::new(ZstdEncoder::new(reader)),
        _ => unreachable!(),
    };
    let mut bytes = Vec::new();
    encoder.read_to_end(&mut bytes).await.unwrap();
    bytes
}
fn fixture() -> serde_json::Value {
    serde_json::from_str::<serde_json::Value>(include_str!("typed-fixtures.json")).unwrap()["GetExchangeStatus"].clone()
}

#[tokio::test]
async fn all_four_codecs_preserve_encoded_bytes_headers_and_typed_payloads() {
    for algorithm in ["gzip", "deflate", "br", "zstd"] {
        let fixture = fixture();
        let success = encode(
            algorithm,
            &serde_json::to_vec(&fixture["response"]).unwrap(),
        )
        .await;
        let failure = encode(
            algorithm,
            &serde_json::to_vec(&fixture["errors"]["500"]).unwrap(),
        )
        .await;
        let server = Server::new(vec![
            Reply::bytes(200, success.clone()).header("Content-Encoding", algorithm),
            Reply::bytes(500, failure.clone()).header("Content-Encoding", algorithm),
        ])
        .await;
        let client = server.client(fast_retry());
        let decoded = client.get_exchange_status().await.unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), fixture["response"]);
        let error = client.get_exchange_status().await.unwrap_err();
        assert!(
            error.payload.is_some(),
            "{algorithm}: lost documented payload"
        );
        if let KalshiError::Api(api) = error.source {
            assert_eq!(api.raw_body, failure);
            assert_eq!(api.body, fixture["errors"]["500"]);
            assert_eq!(api.headers["content-encoding"], algorithm);
            assert_eq!(api.headers["content-length"], failure.len().to_string());
        } else {
            panic!("{algorithm}: lost structured API error")
        }
        assert_eq!(server.count(), 2);
        server.close().await;
    }
}

#[tokio::test]
async fn compressed_unspecified_bodies_retain_raw_entity_bytes() {
    use ed25519_dalek::{SigningKey, pkcs8::EncodePrivateKey};
    let auth = KalshiAuth {
        api_key_id: "FAKE_KEY".into(),
        private_key_pem: SigningKey::from_bytes(&[7; 32])
            .to_pkcs8_pem(Default::default())
            .unwrap()
            .to_string(),
    };
    for bytes in [
        b"FAKE_SECRET_TEXT".to_vec(),
        vec![0xff, 0, 0xfe],
        br#"{"secret":"FAKE_SECRET_JSON"}"#.to_vec(),
    ] {
        let raw = encode("gzip", &bytes).await;
        let server = Server::new(vec![
            Reply::bytes(201, raw.clone()).header("Content-Encoding", "gzip"),
        ])
        .await;
        let client = KalshiClient::builder(server.environment)
            .auth(auth.clone())
            .http_builder(|http| http.no_proxy())
            .build()
            .unwrap();
        let response = client.upgrade_account_api_usage_level().await.unwrap();
        assert_eq!(response.raw_body, raw);
        let expected = serde_json::from_slice::<serde_json::Value>(&bytes)
            .unwrap_or_else(|_| json!(String::from_utf8_lossy(&bytes)));
        assert_eq!(response.body, expected);
        assert_eq!(response.headers["content-encoding"], "gzip");
        assert!(!format!("{response:?}").contains("FAKE_SECRET"));
        server.close().await;
    }
}

#[tokio::test]
async fn stacked_encodings_decode_in_reverse_order_and_gzip_accepts_multiple_members() {
    let body = br#"{"exact":"FAKE_SECRET_JSON"}"#;
    let inner = encode("gzip", body).await;
    let outer = encode("br", &inner).await;
    let server = Server::new(vec![
        Reply::bytes(503, outer.clone()).header("Content-Encoding", "gzip, br"),
    ])
    .await;
    let error = get(&server.client(Default::default())).await.unwrap_err();
    if let KalshiError::Api(api) = error {
        assert_eq!(api.body, json!({"exact":"FAKE_SECRET_JSON"}));
        assert_eq!(api.raw_body, outer);
    } else {
        panic!("stacked decoding failed")
    }
    server.close().await;
    let mut members = encode("gzip", b"{\"members\":").await;
    members.extend(encode("gzip", b"true}").await);
    let server = Server::new(vec![
        Reply::bytes(200, members).header("Content-Encoding", "gzip"),
    ])
    .await;
    assert_eq!(
        get(&server.client(Default::default())).await.unwrap(),
        json!({"members":true})
    );
    server.close().await;
    let mut four = body.to_vec();
    for encoding in ["gzip", "deflate", "br", "zstd"] {
        four = encode(encoding, &four).await;
    }
    let server = Server::new(vec![
        Reply::bytes(200, four).header("Content-Encoding", "gzip, deflate, br, zstd"),
    ])
    .await;
    assert_eq!(
        get(&server.client(Default::default())).await.unwrap(),
        json!({"exact":"FAKE_SECRET_JSON"})
    );
    server.close().await;
}

#[tokio::test]
async fn encoding_failures_are_terminal_keep_complete_raw_and_never_claim_typed_payloads() {
    for algorithm in ["gzip", "deflate", "br", "zstd"] {
        for mut raw in [
            encode(algorithm, br#"{"valid":true}"#).await,
            vec![0xff, 0xfe, 0, 1],
        ] {
            raw.truncate(raw.len().saturating_sub(1));
            let server = Server::new(vec![
                Reply::bytes(503, raw.clone()).header("Content-Encoding", algorithm),
            ])
            .await;
            let error = server
                .client(fast_retry())
                .get_exchange_status()
                .await
                .unwrap_err();
            assert!(error.payload.is_none());
            if let KalshiError::Response(response) = error.source {
                assert_eq!(response.reason, ResponseFailureReason::MalformedEncoding);
                assert!(response.raw_body_complete);
                assert_eq!(response.raw_body, raw);
                assert_eq!(response.headers["content-encoding"], algorithm);
            } else {
                panic!("{algorithm}: expected decoding failure")
            }
            assert_eq!(server.count(), 1);
            server.close().await;
        }
    }
    for (encoding, reason) in [
        ("secret-codec", ResponseFailureReason::UnsupportedEncoding),
        ("gzip,,br", ResponseFailureReason::MalformedEncoding),
        (
            "identity, identity, identity, identity, identity",
            ResponseFailureReason::TooManyEncodingLayers,
        ),
    ] {
        let raw = b"FAKE_SECRET_ENCODED".to_vec();
        let server = Server::new(vec![
            Reply::bytes(503, raw.clone()).header("Content-Encoding", encoding),
        ])
        .await;
        let error = get(&server.client(fast_retry())).await.unwrap_err();
        assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
        if let KalshiError::Response(response) = error {
            assert_eq!(response.reason, reason);
            assert!(response.raw_body_complete);
            assert_eq!(response.raw_body, raw);
        } else {
            panic!("expected content-decoding failure")
        }
        assert_eq!(server.count(), 1);
        server.close().await;
    }
}

#[tokio::test]
async fn encoded_and_each_decoded_layer_are_bounded_and_prefixes_marked_partial() {
    let server = Server::new(vec![Reply::bytes(503, vec![b'x'; 1000])]).await;
    let error = get(&server.client(TransportOptions {
        max_response_bytes: 64,
        ..fast_retry()
    }))
    .await
    .unwrap_err();
    if let KalshiError::Response(response) = error {
        assert_eq!(response.reason, ResponseFailureReason::EncodedSizeLimit);
        assert!(!response.raw_body_complete);
        assert_eq!(response.raw_body, vec![b'x'; 64]);
    } else {
        panic!("expected encoded size failure")
    }
    assert_eq!(server.count(), 1);
    server.close().await;
    // The outer layer expands a gzip header; the final JSON would be tiny.
    // Its intermediate output must still obey the same limit.
    let mut inner = encode("gzip", b"{}").await;
    inner[3] |= 8;
    let mut named = inner[..10].to_vec();
    named.extend(vec![b'n'; 1024]);
    named.push(0);
    named.extend_from_slice(&inner[10..]);
    let raw = encode("gzip", &named).await;
    let server = Server::new(vec![
        Reply::bytes(503, raw.clone()).header("Content-Encoding", "gzip, gzip"),
    ])
    .await;
    let error = get(&server.client(TransportOptions {
        max_response_bytes: 128,
        ..fast_retry()
    }))
    .await
    .unwrap_err();
    if let KalshiError::Response(response) = error {
        assert_eq!(response.reason, ResponseFailureReason::DecodedSizeLimit);
        assert!(response.raw_body_complete);
        assert_eq!(response.raw_body, raw);
    } else {
        panic!("intermediate limit not enforced")
    }
    assert_eq!(server.count(), 1);
    server.close().await;
    let raw = encode("gzip", &vec![b'x'; 10000]).await;
    let server = Server::new(vec![
        Reply::bytes(503, raw.clone()).header("Content-Encoding", "gzip"),
    ])
    .await;
    let error = get(&server.client(TransportOptions {
        max_response_bytes: 128,
        ..fast_retry()
    }))
    .await
    .unwrap_err();
    if let KalshiError::Response(response) = error {
        assert_eq!(response.reason, ResponseFailureReason::DecodedSizeLimit);
        assert!(response.raw_body_complete);
        assert_eq!(response.raw_body, raw);
    } else {
        panic!("expected decoded size failure")
    }
    assert_eq!(server.count(), 1);
    server.close().await;
    let server = Server::new(vec![Reply::Incomplete(b"FAKE_SECRET_PREFIX".to_vec())]).await;
    let error = get(&server.client(Default::default())).await.unwrap_err();
    if let KalshiError::Response(response) = error {
        assert_eq!(response.reason, ResponseFailureReason::Read);
        assert!(!response.raw_body_complete);
        assert_eq!(response.raw_body, b"FAKE_SECRET_PREFIX");
    } else {
        panic!("expected incomplete raw body")
    }
    server.close().await;
}

#[tokio::test]
async fn caller_accept_encoding_is_retained_and_identity_is_supported() {
    let server = Server::new(vec![
        Reply::bytes(200, b"{}".to_vec()).header("Content-Encoding", "identity"),
    ])
    .await;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("accept-encoding", "br, gzip".parse().unwrap());
    let client = KalshiClient::builder(server.environment)
        .http_builder(|http| http.no_proxy().default_headers(headers))
        .build()
        .unwrap();
    get(&client).await.unwrap();
    assert!(server.logs.lock().unwrap()[0].contains("accept-encoding: br, gzip"));
    server.close().await;
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    get(&server.client(Default::default())).await.unwrap();
    assert!(server.logs.lock().unwrap()[0].contains("accept-encoding: identity"));
    let mut request = public_request();
    request
        .headers
        .insert("Accept-Encoding".into(), "zstd".into());
    server
        .client(Default::default())
        .request("GetExchangeStatus", request)
        .await
        .unwrap();
    assert!(server.logs.lock().unwrap()[1].contains("accept-encoding: zstd"));
    server.close().await;
}
