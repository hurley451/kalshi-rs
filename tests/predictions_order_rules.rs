mod typed_support;

use kalshi_client::{models::*, *};
use serde_json::Value;
use std::{net::TcpListener, time::Duration};

fn order(
    tif: CreateOrderV2RequestTimeInForce,
    reduce: Option<bool>,
    expiry: Option<i64>,
) -> CreateOrderV2Request {
    CreateOrderV2Request::builder()
        .ticker("FAKE_SECRET_TICKER")
        .side(BookSide::Bid)
        .count("1.00".parse().unwrap())
        .price("0.0100".parse().unwrap())
        .self_trade_prevention_type(SelfTradePreventionType::Maker)
        .time_in_force(tif)
        .opt_reduce_only(reduce)
        .opt_expiration_time(expiry)
        .build()
}

fn cases() -> Vec<(CreateOrderV2Request, bool)> {
    let mut cases = Vec::new();
    for tif in [
        CreateOrderV2RequestTimeInForce::FillOrKill,
        CreateOrderV2RequestTimeInForce::GoodTillCanceled,
        CreateOrderV2RequestTimeInForce::ImmediateOrCancel,
    ] {
        for reduce in [None, Some(false), Some(true)] {
            for expiry in [None, Some(0)] {
                let ioc = tif == CreateOrderV2RequestTimeInForce::ImmediateOrCancel;
                let valid = !(ioc && expiry.is_some() || reduce == Some(true) && !ioc);
                cases.push((order(tif, reduce, expiry), valid));
            }
        }
    }
    cases
}

#[tokio::test]
async fn single_and_mixed_batch_negatives_reject_without_http() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let environment = KalshiEnvironment {
        name: "order-rules-negative",
        rest_base_url: Box::leak(
            format!("http://{}/trade-api/v2", listener.local_addr().unwrap()).into_boxed_str(),
        ),
        websocket_url: "ws://unused",
    };
    let client =
        KalshiClient::with_http_client_builder(environment, Some(typed_support::auth()), |http| {
            http.no_proxy()
        })
        .unwrap()
        .with_request_options(RequestOptions {
            timeout: Some(Duration::from_secs(2)),
            ..Default::default()
        });
    let valid = order(
        CreateOrderV2RequestTimeInForce::GoodTillCanceled,
        None,
        None,
    );
    let mut rejections = 0;
    for (candidate, accepted) in cases() {
        if accepted {
            continue;
        }
        let body = CreateOrderV2Body(candidate.clone());
        let error = client.portfolio().create_order_v2(&body).await.unwrap_err();
        assert!(matches!(error.source, KalshiError::Contract(_)));
        assert!(!error.to_string().contains("FAKE_SECRET"));
        rejections += 1;
        for orders in [
            vec![candidate.clone(), valid.clone()],
            vec![valid.clone(), candidate.clone()],
        ] {
            let body = BatchCreateOrdersV2Body(
                BatchCreateOrdersV2Request::builder().orders(orders).build(),
            );
            let error = client
                .portfolio()
                .batch_create_orders_v2(&body)
                .await
                .unwrap_err();
            assert!(matches!(error.source, KalshiError::Contract(_)));
            assert!(!error.to_string().contains("FAKE_SECRET"));
            rejections += 1;
        }
    }
    assert_eq!(rejections, 21);
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[tokio::test]
async fn native_positive_combinations_use_signed_single_and_batch_methods() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
    let auth = typed_support::auth();
    let mut controls = 0;
    for (candidate, accepted) in cases() {
        if !accepted {
            continue;
        }
        let body = CreateOrderV2Body(candidate.clone());
        let (client, server) = typed_support::server(
            vec![(201, fixtures["CreateOrderV2"]["response"].clone())],
            Some(auth.clone()),
        );
        client.portfolio().create_order_v2(&body).await.unwrap();
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateOrderV2",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
        let batch = BatchCreateOrdersV2Body(
            BatchCreateOrdersV2Request::builder()
                .orders(vec![candidate])
                .build(),
        );
        let (client, server) = typed_support::server(
            vec![(201, fixtures["BatchCreateOrdersV2"]["response"].clone())],
            Some(auth.clone()),
        );
        client
            .portfolio()
            .batch_create_orders_v2(&batch)
            .await
            .unwrap();
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "BatchCreateOrdersV2",
            &requests[0],
            None,
            Some(&serde_json::to_value(&batch).unwrap()),
            true,
            &auth,
        );
        controls += 2;
    }
    assert_eq!(controls, 22);
}

#[tokio::test]
async fn canceled_or_expired_entry_preserves_control_priority() {
    let client = KalshiClient::new(DEMO, None);
    let body = CreateOrderV2Body(order(
        CreateOrderV2RequestTimeInForce::ImmediateOrCancel,
        Some(true),
        Some(0),
    ));
    let token = CancellationToken::new();
    token.cancel();
    let canceled = client.with_request_options(RequestOptions {
        cancellation: Some(token),
        ..Default::default()
    });
    assert!(matches!(
        canceled.create_order_v2(&body).await.unwrap_err().source,
        KalshiError::Cancelled
    ));
    let expired = client.with_request_options(RequestOptions {
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    assert!(matches!(
        expired.create_order_v2(&body).await.unwrap_err().source,
        KalshiError::DeadlineExceeded
    ));
}
