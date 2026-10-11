use std::collections::BTreeMap;

use kalshi_client::{
    DEMO, KalshiAuth, KalshiClient, KalshiRequest, OFFICIAL_OPERATION_SOURCE, OPERATIONS,
    QueryValue, build_operation_path, build_portfolio_fills_path, build_signing_preimage,
    build_trade_api_path, get_operation,
};
use rand_core::OsRng;
use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePrivateKey;

#[test]
fn generated_registry_covers_official_inventory() {
    assert_eq!(OFFICIAL_OPERATION_SOURCE.operation_count, 118);
    assert_eq!(OPERATIONS.len(), OFFICIAL_OPERATION_SOURCE.operation_count);
    assert_eq!(get_operation("GetFills").unwrap().path, "/portfolio/fills");
}

#[test]
fn builds_operation_paths_and_trade_api_signing_paths() {
    let operation = get_operation("GetMarket").unwrap();
    let mut path_parameters = BTreeMap::new();
    path_parameters.insert("ticker".to_string(), "KXTEST-26".to_string());
    let query = vec![
        (
            "tags".to_string(),
            QueryValue::Array(vec![QueryValue::from("macro"), QueryValue::from("rates")]),
        ),
        ("limit".to_string(), QueryValue::from(10_u32)),
    ];

    let path = build_operation_path(operation, &path_parameters, &query).unwrap();
    assert_eq!(path, "/markets/KXTEST-26?tags=macro%2Crates&limit=10");
    assert_eq!(
        build_trade_api_path("/portfolio/fills?limit=2"),
        "/trade-api/v2/portfolio/fills?limit=2"
    );
    assert_eq!(
        build_signing_preimage(
            "1790000000000",
            "get",
            "/trade-api/v2/portfolio/fills?limit=2"
        ),
        "1790000000000GET/trade-api/v2/portfolio/fills"
    );
}

#[test]
fn prepares_authenticated_requests() {
    let auth = test_auth();
    let client = KalshiClient::new(DEMO, Some(auth));
    let mut request = KalshiRequest::default();
    request
        .path_parameters
        .insert("ticker".to_string(), "KXTEST-26".to_string());
    request
        .query
        .push(("nested".to_string(), QueryValue::from(true)));

    let prepared = client
        .prepare_request("GetMarket", request, "1790000000000")
        .unwrap();

    assert_eq!(
        prepared.url,
        "https://demo-api.kalshi.co/trade-api/v2/markets/KXTEST-26?nested=true"
    );
    assert_eq!(prepared.method, "GET");
    assert_eq!(
        prepared.signing_path,
        "/trade-api/v2/markets/KXTEST-26?nested=true"
    );
    assert_eq!(
        prepared.headers.get("KALSHI-ACCESS-KEY").unwrap(),
        "test-key-id"
    );
    assert!(
        prepared
            .headers
            .get("KALSHI-ACCESS-SIGNATURE")
            .unwrap()
            .len()
            > 20
    );
}

#[test]
fn keeps_fills_helper_compatible() {
    let query = kalshi_client::FillQuery {
        tickers: vec!["KXAAA".to_string(), "KXBBB".to_string()],
        limit: Some(100),
        cursor: Some("abc".to_string()),
        ..Default::default()
    };
    assert_eq!(
        build_portfolio_fills_path(&query).unwrap(),
        "/trade-api/v2/portfolio/fills?limit=100&cursor=abc&ticker=KXAAA%2CKXBBB"
    );
}

fn test_auth() -> KalshiAuth {
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).unwrap();
    KalshiAuth {
        api_key_id: "test-key-id".to_string(),
        private_key_pem: private_key
            .to_pkcs8_pem(Default::default())
            .unwrap()
            .to_string(),
    }
}
