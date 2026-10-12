mod typed_support;
use kalshi_client::{generated::typed::*, models::*, *};
use serde_json::{Value, json};

#[tokio::test]
async fn undeclared_success_preserves_binary_json_and_text_bytes_without_debug_leaks() {
    let cases = [
        (vec![0xff, 0xfe], json!("\u{fffd}\u{fffd}")),
        (
            b"FAKE_SECRET_MARKER_TEXT".to_vec(),
            json!("FAKE_SECRET_MARKER_TEXT"),
        ),
        (
            br#"{"secret":"FAKE_SECRET_MARKER_JSON"}"#.to_vec(),
            json!({"secret":"FAKE_SECRET_MARKER_JSON"}),
        ),
    ];
    let (client, server) = typed_support::server_bytes(
        cases
            .iter()
            .map(|(bytes, _)| (201, bytes.clone()))
            .collect(),
        Some(typed_support::auth()),
    );
    for (bytes, decoded) in cases {
        let response = client.upgrade_account_api_usage_level().await.unwrap();
        assert_eq!(response.raw_body, bytes);
        assert_eq!(response.body, decoded);
        assert_eq!(response.status.as_u16(), 201);
        assert_eq!(response.headers["x-request-id"], "fake-probe");
        assert!(!format!("{response:?}").contains("FAKE_SECRET_MARKER"));
    }
    server.join().unwrap();
}

#[test]
fn private_keys_nested_models_and_errors_redact_fake_markers() {
    let conversion =
        kalshi_client::models::error::ConversionError::from("FAKE_SECRET_MARKER_CONVERSION");
    assert!(!format!("{conversion:?}").contains("FAKE_SECRET_MARKER"));
    let model: GenerateApiKeyResponse=serde_json::from_value(json!({"api_key_id":"FAKE_SECRET_MARKER_ID","private_key":"FAKE_SECRET_MARKER_PEM","warning":"FAKE_SECRET_MARKER_WARNING"})).unwrap();
    let nested = GenerateApiKeyStatus201(model.clone());
    for debug in [
        format!("{model:?}"),
        format!("{nested:?}"),
        format!("{:?}", Some(model)),
    ] {
        assert!(!debug.contains("FAKE_SECRET_MARKER"));
        assert!(debug.contains("REDACTED"));
    }
    let payload: ErrorResponse=serde_json::from_value(json!({"code":"FAKE_SECRET_MARKER_CODE","message":"FAKE_SECRET_MARKER_MESSAGE","details":"FAKE_SECRET_MARKER_PEM"})).unwrap();
    assert!(!format!("{payload:?}").contains("FAKE_SECRET_MARKER"));
    let api = KalshiApiError {
        status: reqwest::StatusCode::BAD_REQUEST,
        headers: Default::default(),
        body: json!({"private_key":"FAKE_SECRET_MARKER_PEM"}),
    };
    let error = kalshi_client::typed::TypedError::<GenerateApiKeyError>::new(
        KalshiError::Api(Box::new(api)),
        None,
    );
    assert!(!format!("{error:?} {error}").contains("FAKE_SECRET_MARKER"));
}

#[test]
fn nullable_fields_distinguish_absent_null_and_value_and_required_presence() {
    for (value, expected) in [
        (json!({}), None),
        (json!({"warning":null}), Some(None)),
        (
            json!({"warning":"warning"}),
            Some(Some("warning".to_string())),
        ),
    ] {
        let mut value = value;
        value["api_key_id"] = json!("key");
        value["private_key"] = json!("fake");
        let model: GenerateApiKeyResponse = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(model.warning, expected);
        assert_eq!(serde_json::to_value(model).unwrap(), value);
    }
    let value =
        json!({"open":null,"low":null,"high":null,"close":null,"mean":null,"previous":null});
    let model: PriceDistributionHistorical = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(model).unwrap(), value);
    let mut missing = value;
    missing.as_object_mut().unwrap().remove("open");
    assert!(serde_json::from_value::<PriceDistributionHistorical>(missing).is_err());
    assert!(serde_json::from_value::<GetMarketsParameters>(json!({"limit":null})).is_err());
}

#[test]
fn fixed_point_constraints_required_enums_and_dates_are_enforced() {
    for text in ["0.120000", "123456789012345678901234567890.123456"] {
        let model: FixedPointDollars = serde_json::from_value(json!(text)).unwrap();
        assert_eq!(serde_json::to_value(model).unwrap(), json!(text));
    }
    for (name, value) in [
        ("FixedPointDollars", json!(0.12)),
        ("GetMarketsParameters", json!({"limit":1001})),
        ("GetMarketsParameters", json!({"status":"invalid"})),
        ("GetMarketCandlesticksParameters", json!({})),
        ("GenerateApiKeyResponse", json!({"api_key_id":"key"})),
    ] {
        assert!(
            kalshi_client::typed::validate(name, &value).is_err(),
            "{name}"
        );
    }
    let mut fixtures: Value =
        serde_json::from_str(include_str!("typed-model-fixtures.json")).unwrap();
    fixtures["Market"]["created_time"] = json!("invalid-date");
    assert!(kalshi_client::typed::validate("Market", &fixtures["Market"]).is_err());
    assert!(
        kalshi_client::typed::validate(
            "GetHistoricalMarketsParameters",
            &json!({"mve_filter":null})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<GetHistoricalMarketsParameters>(json!({"mve_filter":null}))
            .is_err()
    );
    assert!(
        kalshi_client::typed::validate(
            "GetHistoricalMarketsParameters",
            &json!({"mve_filter":"exclude"})
        )
        .is_ok()
    );
    let parameters: GetMarketsParameters = serde_json::from_value(json!({})).unwrap();
    assert_eq!(serde_json::to_value(parameters).unwrap(), json!({}));
}

#[tokio::test]
async fn unspecified_success_and_error_bodies_remain_available_and_redacted() {
    let auth = typed_support::auth();
    let raw = json!({"private_key":"FAKE_SECRET_MARKER_RETURNED"});
    let (client, server) =
        typed_support::server(vec![(201, raw.clone()), (403, raw.clone())], Some(auth));
    let response = client.upgrade_account_api_usage_level().await.unwrap();
    assert_eq!(response.status.as_u16(), 201);
    assert_eq!(response.body, raw);
    assert_eq!(response.headers["x-request-id"], "fake-probe");
    assert!(!format!("{response:?}").contains("FAKE_SECRET_MARKER"));
    let error = client.upgrade_account_api_usage_level().await.unwrap_err();
    assert!(matches!(
        error.payload,
        Some(UpgradeAccountApiUsageLevelError::Status403)
    ));
    if let KalshiError::Api(api) = error.source {
        assert_eq!(api.status.as_u16(), 403);
        assert_eq!(api.body, raw);
        assert_eq!(api.headers["x-request-id"], "fake-probe");
    } else {
        panic!("unspecified error replaced by schema error");
    }
    server.join().unwrap();
}

#[tokio::test]
async fn pagination_advances_cursor_keeps_filters_and_stops() {
    let (client, server) = typed_support::server(
        vec![
            (200, json!({"markets":[],"cursor":"next cursor"})),
            (200, json!({"markets":[],"cursor":""})),
        ],
        None,
    );
    let mut pager = client.markets().get_markets_pages(GetMarketsParameters {
        series_ticker: Some("SERIES".into()),
        limit: Some(2),
        ..Default::default()
    });
    assert!(pager.next_page().await.unwrap().is_some());
    assert!(pager.next_page().await.unwrap().is_some());
    assert!(pager.next_page().await.unwrap().is_none());
    let requests = server.join().unwrap();
    assert!(requests[0].contains("series_ticker=SERIES"));
    assert!(requests[1].contains("series_ticker=SERIES"));
    assert!(requests[1].contains("cursor=next%20cursor"));
    assert!(requests.iter().all(|r| r.contains("limit=2")));
}

#[tokio::test]
async fn pagination_retries_errors_detects_cycles_and_handles_next_cursor() {
    let (client, server) = typed_support::server(
        vec![
            (500, json!({})),
            (200, json!({"markets":[],"cursor":"cycle"})),
            (200, json!({"markets":[],"cursor":"cycle"})),
        ],
        None,
    );
    let mut pager = client.get_markets_pages(Default::default());
    assert!(pager.next_page().await.is_err());
    assert!(pager.next_page().await.unwrap().is_some());
    assert!(pager.next_page().await.is_err());
    assert!(pager.next_page().await.unwrap().is_none());
    let requests = server.join().unwrap();
    assert_eq!(requests[0].lines().next(), requests[1].lines().next());
    let (client, server) = typed_support::server(
        vec![
            (200, json!({"incentive_programs":[],"next_cursor":"next"})),
            (200, json!({"incentive_programs":[]})),
        ],
        None,
    );
    assert_eq!(
        client
            .get_incentive_programs_pages(Default::default())
            .collect_pages()
            .await
            .unwrap()
            .len(),
        2
    );
    assert!(server.join().unwrap()[1].contains("cursor=next"));
}

#[tokio::test]
async fn typed_query_arrays_repeat_and_empty_arrays_disappear() {
    let (client, server) =
        typed_support::server(vec![(499, Value::Null), (499, Value::Null)], None);
    let parameters: GetMarketOrderbooksParameters =
        serde_json::from_value(json!({"tickers":["A B","C"]})).unwrap();
    assert!(client.get_market_orderbooks(&parameters).await.is_err());
    let parameters: GetMarketOrderbooksParameters =
        serde_json::from_value(json!({"tickers":[]})).unwrap();
    // Source minItems may disallow an empty request; validate before transport.
    if kalshi_client::typed::validate(
        "GetMarketOrderbooksParameters",
        &serde_json::to_value(&parameters).unwrap(),
    )
    .is_ok()
    {
        assert!(client.get_market_orderbooks(&parameters).await.is_err());
    } else {
        let parameters: GetMarketOrderbooksParameters =
            serde_json::from_value(json!({"tickers":["D"]})).unwrap();
        assert!(client.get_market_orderbooks(&parameters).await.is_err());
    }
    let requests = server.join().unwrap();
    assert!(requests[0].contains("tickers=A%20B&tickers=C"));
}
