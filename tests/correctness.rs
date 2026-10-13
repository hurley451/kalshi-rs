use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use kalshi_client::*;
use serde_json::{Value, json};

fn fill_page() -> Value {
    json!({"fills":[{
        "fill_id":"fill-1","exchange_index":0,"trade_id":"trade-1",
        "order_id":"order-1","ticker":"MARKET","market_ticker":"MARKET",
        "outcome_side":"yes","book_side":"bid","count_fp":"1.00",
        "yes_price_dollars":"0.120000","no_price_dollars":"0.880000",
        "is_taker":true,"fee_cost":"0.000010"
    }],"cursor":""})
}

#[test]
fn fills_preserve_required_fields_and_accept_omitted_optional_fields() {
    let page = parse_fills_page(fill_page()).unwrap();
    assert_eq!(page.fills[0].count_fp, "1.00");
    assert_eq!(page.fills[0].fee_cost, "0.000010");
    assert!(page.fills[0].side.is_none());
    assert!(page.fills[0].created_time.is_none());
    for field in [
        "fill_id",
        "exchange_index",
        "trade_id",
        "order_id",
        "ticker",
        "market_ticker",
        "outcome_side",
        "book_side",
        "count_fp",
        "yes_price_dollars",
        "no_price_dollars",
        "is_taker",
        "fee_cost",
    ] {
        let mut value = fill_page();
        value["fills"][0].as_object_mut().unwrap().remove(field);
        assert!(parse_fills_page(value).is_err(), "accepted missing {field}");
    }
    for value in [json!({"fills":[]}), json!({"fills":[],"cursor":null})] {
        assert!(parse_fills_page(value).is_err());
    }
}

#[test]
fn fills_reject_invalid_optional_fields_without_rejecting_nullable_subaccounts() {
    for (name, value) in [
        ("side", json!("bid")),
        ("action", json!("cancel")),
        ("outcome_side", json!("invalid")),
        ("book_side", json!("yes")),
        ("created_time", json!("invalid")),
        ("side", Value::Null),
        ("action", Value::Null),
        ("created_time", Value::Null),
        ("ts", Value::Null),
    ] {
        let mut body = fill_page();
        body["fills"][0][name] = value;
        assert!(parse_fills_page(body).is_err(), "accepted invalid {name}");
    }
    let mut body = fill_page();
    body["fills"][0]["subaccount_number"] = Value::Null;
    body["fills"][0]["created_time"] = json!("2026-10-12T00:00:00Z");
    assert!(parse_fills_page(body).is_ok());
}

#[test]
fn fills_validate_known_filter_bounds() {
    for limit in [0, 1001] {
        assert!(
            portfolio_fills_query(&FillQuery {
                limit: Some(limit),
                ..Default::default()
            })
            .is_err()
        );
    }
    assert!(
        portfolio_fills_query(&FillQuery {
            subaccount: Some(64),
            ..Default::default()
        })
        .is_err()
    );
    assert!(
        portfolio_fills_query(&FillQuery {
            limit: Some(1000),
            subaccount: Some(63),
            ..Default::default()
        })
        .is_ok()
    );
}

#[test]
fn fills_round_trip_without_inventing_null_nonnullable_fields() {
    let page = parse_fills_page(fill_page()).unwrap();
    let serialized = serde_json::to_value(&page).unwrap();
    for name in ["side", "action", "created_time", "ts"] {
        assert!(
            serialized["fills"][0].get(name).is_none(),
            "invented optional {name}"
        );
    }
    assert_eq!(serialized["fills"][0]["subaccount_number"], Value::Null);
    assert_eq!(parse_fills_page(serialized).unwrap(), page);
    let mut value = fill_page();
    for (name, field) in [
        ("side", json!("yes")),
        ("action", json!("buy")),
        ("created_time", json!("2026-10-12T00:00:00Z")),
        ("ts", json!(1791763200)),
    ] {
        value["fills"][0][name] = field;
    }
    let page = parse_fills_page(value).unwrap();
    assert_eq!(
        parse_fills_page(serde_json::to_value(&page).unwrap()).unwrap(),
        page
    );
}

#[test]
fn queries_obey_operation_specific_array_serialization() {
    for (id, name) in [
        ("GetMarketOrderbooks", "tickers"),
        ("GetEventForecastPercentilesHistory", "percentiles"),
        ("GetLiveDatas", "milestone_ids"),
        ("GetStructuredTargets", "ids"),
    ] {
        let operation = get_operation(id).unwrap();
        let query = vec![(
            name.into(),
            QueryValue::Array(vec!["A B".into(), "C".into()]),
        )];
        assert_eq!(
            serialize_operation_query(operation, &query),
            format!("{name}=A%20B&{name}=C")
        );
        assert_eq!(
            serialize_operation_query(operation, &vec![(name.into(), QueryValue::Array(vec![]))]),
            ""
        );
    }
    assert_eq!(
        serialize_operation_query(
            get_operation("GetFills").unwrap(),
            &vec![(
                "ticker".into(),
                QueryValue::Array(vec!["A".into(), "B".into()])
            )]
        ),
        "ticker=A%2CB"
    );
}

#[test]
fn debug_output_redacts_credentials_and_account_payloads() {
    let auth = KalshiAuth {
        api_key_id: "SECRET_KEY_ID".into(),
        private_key_pem: "SECRET_PRIVATE_KEY".into(),
    };
    let client = KalshiClient::new(DEMO, Some(auth.clone()));
    for debug in [format!("{auth:?}"), format!("{client:?}")] {
        assert!(!debug.contains("SECRET"));
    }
    let signed = SignedKalshiRequest {
        timestamp_ms: "123".into(),
        signature: "SECRET_SIGNATURE".into(),
        headers: BTreeMap::from([("Authorization".into(), "SECRET_TOKEN".into())]),
    };
    assert!(!format!("{signed:?}").contains("SECRET"));
    let prepared = PreparedKalshiRequest {
        method: "GET".into(),
        url: "https://example.test".into(),
        signing_path: "/trade-api/v2/test".into(),
        headers: signed.headers,
        body: Some(json!({"account":"SECRET_ACCOUNT"})),
    };
    assert!(!format!("{prepared:?}").contains("SECRET"));
}

#[test]
fn headers_are_case_insensitive_and_signed_auth_replaces_stale_values() {
    use rsa::pkcs8::EncodePrivateKey;
    let key = rsa::RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    let auth = KalshiAuth {
        api_key_id: "fresh-id".into(),
        private_key_pem: key.to_pkcs8_pem(Default::default()).unwrap().to_string(),
    };
    let client = KalshiClient::new(DEMO, Some(auth));
    let request = KalshiRequest {
        headers: BTreeMap::from([
            ("accept".into(), "application/custom".into()),
            ("kalshi-access-key".into(), "stale".into()),
            ("kalshi-access-signature".into(), "stale".into()),
        ]),
        ..Default::default()
    };
    let prepared = client
        .prepare_request("GetFills", request, "1790000000000")
        .unwrap();
    assert_eq!(prepared.headers["Accept"], "application/custom");
    assert_eq!(prepared.headers["KALSHI-ACCESS-KEY"], "fresh-id");
    assert_ne!(prepared.headers["KALSHI-ACCESS-SIGNATURE"], "stale");
    assert_eq!(
        prepared
            .headers
            .keys()
            .filter(|name| name.eq_ignore_ascii_case("kalshi-access-key"))
            .count(),
        1
    );
    for headers in [
        BTreeMap::from([("bad\nname".into(), "value".into())]),
        BTreeMap::from([("accept".into(), "bad\nvalue".into())]),
    ] {
        assert!(matches!(
            client.prepare_request(
                "GetFills",
                KalshiRequest {
                    headers,
                    ..Default::default()
                },
                "1"
            ),
            Err(KalshiError::InvalidHeader(_))
        ));
    }
}

#[test]
fn signatures_verify_independently_against_the_exact_wire_preimage() {
    use base64::Engine;
    use rsa::pkcs8::EncodePrivateKey;
    use rsa::signature::Verifier;
    let key = rsa::RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    let auth = KalshiAuth {
        api_key_id: "test-id".into(),
        private_key_pem: key.to_pkcs8_pem(Default::default()).unwrap().to_string(),
    };
    let signed = sign_kalshi_request(
        &auth,
        "get",
        "/trade-api/v2/portfolio/fills?limit=2",
        "1790000000000",
    )
    .unwrap();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&signed.signature)
        .unwrap();
    let signature = rsa::pss::Signature::try_from(bytes.as_slice()).unwrap();
    let verifier = rsa::pss::VerifyingKey::<sha2::Sha256>::new(key.to_public_key());
    verifier
        .verify(b"1790000000000GET/trade-api/v2/portfolio/fills", &signature)
        .unwrap();
    assert!(
        verifier
            .verify(
                b"1790000000000POST/trade-api/v2/portfolio/fills",
                &signature
            )
            .is_err()
    );
    assert!(
        verifier
            .verify(
                b"1790000000000GET/trade-api/v2/portfolio/fills?limit=2",
                &signature
            )
            .is_err()
    );
    assert_eq!(signed.headers["KALSHI-ACCESS-SIGNATURE"], signed.signature);
}

#[test]
fn websocket_envelopes_preserve_command_correlation_and_timestamps() {
    let envelope = parse_websocket_message(
        json!({"id":123,"type":"error","sending_ts_ms":1669149841234_i64,
        "msg":{"code":7,"msg":"Unknown subscription ID"}}),
    )
    .unwrap();
    assert_eq!(envelope.id, Some(123));
    assert_eq!(envelope.sending_ts_ms, Some(1669149841234));
    let serialized = serde_json::to_value(envelope).unwrap();
    assert_eq!(serialized["id"], 123);
}

fn loopback_response(
    status: &str,
    headers: &str,
    body: &str,
) -> (KalshiClient, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = mpsc::channel();
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",
        body.len()
    );
    let handle = thread::spawn(move || {
        sender.send(()).unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(value) => break value,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(
                        std::time::Instant::now() < deadline,
                        "request never reached loopback server"
                    );
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("loopback accept failed: {error}"),
            }
        };
        stream.set_nonblocking(false).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = Vec::new();
        let read_deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            let mut buffer = [0; 1024];
            let count = match stream.read(&mut buffer) {
                Ok(count) => count,
                Err(error)
                    if [
                        std::io::ErrorKind::Interrupted,
                        std::io::ErrorKind::WouldBlock,
                        std::io::ErrorKind::TimedOut,
                    ]
                    .contains(&error.kind()) =>
                {
                    assert!(
                        std::time::Instant::now() < read_deadline,
                        "request header read timed out"
                    );
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(error) => panic!("request header read failed: {error}"),
            };
            assert!(count > 0, "request ended before headers");
            bytes.extend_from_slice(&buffer[..count]);
            if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        stream.write_all(response.as_bytes()).unwrap();
        String::from_utf8(bytes).unwrap()
    });
    receiver.recv_timeout(Duration::from_secs(5)).unwrap();
    let environment = KalshiEnvironment {
        name: "loopback",
        rest_base_url: Box::leak(format!("http://{address}/trade-api/v2").into_boxed_str()),
        websocket_url: "ws://unused",
    };
    let http = |builder: reqwest::ClientBuilder| builder.no_proxy().timeout(Duration::from_secs(5));
    (
        KalshiClient::with_http_client_builder(environment, None, http).unwrap(),
        handle,
    )
}

#[tokio::test]
async fn transport_accepts_empty_successes_and_decodes_json() {
    for (status, body, expected) in [
        ("204 No Content", "", Value::Null),
        ("200 OK", "", Value::Null),
        (
            "200 OK",
            "{\"exchange_active\":true}",
            json!({"exchange_active":true}),
        ),
    ] {
        let (client, server) =
            loopback_response(status, "Content-Type: application/json\r\n", body);
        assert_eq!(
            client
                .request(
                    "GetExchangeStatus",
                    KalshiRequest {
                        authenticated: false,
                        ..Default::default()
                    }
                )
                .await
                .unwrap(),
            expected
        );
        assert!(
            server
                .join()
                .unwrap()
                .starts_with("GET /trade-api/v2/exchange/status HTTP/1.1\r\n")
        );
    }
}

#[tokio::test]
async fn transport_preserves_json_and_text_api_errors_without_debug_disclosure() {
    for body in [
        "{\"error\":{\"code\":\"throttled\",\"message\":\"SECRET_ACCOUNT\"}}",
        "SECRET_ACCOUNT invalid JSON",
    ] {
        let (client, server) = loopback_response(
            "429 Too Many Requests",
            "Content-Type: application/json\r\nRetry-After: 2\r\n",
            body,
        );
        let error = client
            .request(
                "GetExchangeStatus",
                KalshiRequest {
                    authenticated: false,
                    ..Default::default()
                },
            )
            .await
            .unwrap_err();
        assert!(!format!("{error:?}").contains("SECRET_ACCOUNT"));
        assert!(!error.to_string().contains("SECRET_ACCOUNT"));
        match error {
            KalshiError::Api(error) => {
                assert_eq!(error.status, 429);
                assert_eq!(error.headers["retry-after"], "2");
                if body.starts_with('{') {
                    assert_eq!(error.body["error"]["code"], "throttled");
                } else {
                    assert_eq!(error.body, body);
                }
            }
            other => panic!("unexpected error: {other}"),
        }
        server.join().unwrap();
    }
}

#[tokio::test]
async fn transport_does_not_accept_malformed_success_json() {
    let (client, server) =
        loopback_response("200 OK", "Content-Type: application/json\r\n", "{broken");
    assert!(matches!(
        client
            .request(
                "GetExchangeStatus",
                KalshiRequest {
                    authenticated: false,
                    ..Default::default()
                }
            )
            .await,
        Err(KalshiError::Json(_))
    ));
    server.join().unwrap();
}
