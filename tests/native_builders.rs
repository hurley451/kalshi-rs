mod typed_support;

use kalshi_client::models::*;
use serde_json::{Value, json};
use std::num::NonZeroU64;

fn order(client_order_id: Option<String>) -> CreateOrderV2Request {
    CreateOrderV2Request::builder()
        .ticker("TEST-TICKER")
        .side(BookSide::Bid)
        .count("1.00".parse().unwrap())
        .price("0.0100".parse().unwrap())
        .self_trade_prevention_type(SelfTradePreventionType::Maker)
        .time_in_force(CreateOrderV2RequestTimeInForce::GoodTillCanceled)
        .opt_client_order_id(client_order_id)
        .build()
}

#[test]
fn native_construction_preserves_presence_and_extension_fields() {
    let plain = order(None);
    let value = serde_json::to_value(&plain).unwrap();
    assert_eq!(
        value,
        json!({"ticker":"TEST-TICKER", "side":"bid", "count":"1.00",
        "price":"0.0100", "self_trade_prevention_type":"maker", "time_in_force":"good_till_canceled"})
    );
    kalshi_client::typed::validate("CreateOrderV2Request", &value).unwrap();
    assert!(!format!("{:?}", order(Some("SENSITIVE-CLIENT-ID".into()))).contains("SENSITIVE"));

    for nullable in [None, Some(None), Some(Some("10.00".parse().unwrap()))] {
        let group = CreateOrderGroupRequest::builder()
            .contracts_limit(NonZeroU64::new(10).unwrap())
            .opt_contracts_limit_fp(nullable.clone())
            .build();
        let value = serde_json::to_value(group).unwrap();
        assert_eq!(
            value.get("contracts_limit_fp"),
            nullable
                .as_ref()
                .map(|inner| match inner {
                    None => Value::Null,
                    Some(value) => Value::String(value.as_str().to_owned()),
                })
                .as_ref()
        );
        assert!(!value.as_object().unwrap().contains_key("exchange_index"));
        kalshi_client::typed::validate("CreateOrderGroupRequest", &value).unwrap();
    }
    let explicit_null = CreateOrderGroupRequest::builder()
        .contracts_limit_fp(None)
        .build();
    assert_eq!(
        serde_json::to_value(explicit_null).unwrap()["contracts_limit_fp"],
        Value::Null
    );

    let mut extra = serde_json::Map::new();
    extra.insert("future_extension".into(), json!([]));
    let batch = BatchCreateOrdersV2Request::builder()
        .orders(vec![order(None)])
        .extra(extra)
        .build();
    let value = serde_json::to_value(batch).unwrap();
    assert_eq!(value["orders"][0], serde_json::to_value(plain).unwrap());
    assert_eq!(value["future_extension"], json!([]));
    let empty = BatchCreateOrdersV2Request::builder().orders(vec![]).build();
    assert_eq!(serde_json::to_value(empty).unwrap(), json!({"orders":[]}));

    let query = GetMarketsParameters::builder()
        .opt_limit(Some(100))
        .opt_cursor(None)
        .build();
    assert_eq!(serde_json::to_value(query).unwrap(), json!({"limit":100}));
    let legacy = CreateOrderRequest::builder()
        .ticker("TEST-TICKER")
        .action(CreateOrderRequestAction::Buy)
        .side(CreateOrderRequestSide::Yes)
        .count(NonZeroU64::new(1).unwrap())
        .yes_price(NonZeroU64::new(1).unwrap())
        .build();
    kalshi_client::typed::validate("CreateOrderRequest", &serde_json::to_value(legacy).unwrap())
        .unwrap();
}

#[tokio::test]
async fn native_mutation_models_dispatch_through_named_facades() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
    let ids = [
        "CreateOrderV2",
        "CreateOrderGroup",
        "CreateQuote",
        "ApplySubaccountTransfer",
    ];
    let auth = typed_support::auth();
    let (client, server) = typed_support::server(
        ids.iter()
            .map(|id| {
                let status = if *id == "ApplySubaccountTransfer" {
                    200
                } else {
                    201
                };
                (status, fixtures[id]["response"].clone())
            })
            .collect(),
        Some(auth.clone()),
    );
    let native_order = order(None);
    client
        .portfolio()
        .create_order_v2(&CreateOrderV2Body(native_order))
        .await
        .unwrap();
    let group = CreateOrderGroupRequest::builder()
        .contracts_limit(NonZeroU64::new(10).unwrap())
        .build();
    client
        .portfolio()
        .create_order_group(&CreateOrderGroupBody(group))
        .await
        .unwrap();
    let quote = CreateQuoteRequest::builder()
        .rfq_id("00000000-0000-0000-0000-000000000001")
        .yes_bid("0.0100".parse().unwrap())
        .no_bid("0.0200".parse().unwrap())
        .rest_remainder(false)
        .build();
    client
        .communications()
        .create_quote(&CreateQuoteBody(quote))
        .await
        .unwrap();
    let transfer = ApplySubaccountTransferRequest::builder()
        .amount_cents(100)
        .client_transfer_id(uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap())
        .from_subaccount(0)
        .to_subaccount(1)
        .build();
    client
        .portfolio()
        .apply_subaccount_transfer(&ApplySubaccountTransferBody(transfer))
        .await
        .unwrap();
    let requests = server.join().unwrap();
    let expected = [
        json!({"ticker":"TEST-TICKER", "side":"bid", "count":"1.00", "price":"0.0100",
            "self_trade_prevention_type":"maker", "time_in_force":"good_till_canceled"}),
        json!({"contracts_limit":10}),
        json!({"rfq_id":"00000000-0000-0000-0000-000000000001", "yes_bid":"0.0100", "no_bid":"0.0200", "rest_remainder":false}),
        json!({"amount_cents":100, "client_transfer_id":"00000000-0000-0000-0000-000000000002", "from_subaccount":0, "to_subaccount":1}),
    ];
    for ((id, request), body) in ids.iter().zip(&requests).zip(&expected) {
        typed_support::assert_request(id, request, None, Some(body), true, &auth);
    }
}
