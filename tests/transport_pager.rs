mod transport_support;
use kalshi_client::*;
use serde_json::{Value, json};
use std::time::Duration;
use tokio::time::Instant;
use transport_support::*;

fn auth() -> KalshiAuth {
    use ed25519_dalek::{SigningKey, pkcs8::EncodePrivateKey};
    KalshiAuth {
        api_key_id: "FAKE_KEY".into(),
        private_key_pem: SigningKey::from_bytes(&[7; 32])
            .to_pkcs8_pem(Default::default())
            .unwrap()
            .to_string(),
    }
}
fn fixtures() -> Value {
    serde_json::from_str(include_str!("typed-fixtures.json")).unwrap()
}
fn client(server: &Server) -> KalshiClient {
    KalshiClient::builder(server.environment)
        .auth(auth())
        .http_builder(|http| http.no_proxy())
        .build()
        .unwrap()
}

#[tokio::test]
async fn all_28_pagers_honor_options_and_preserve_state_after_pre_cancel() {
    let fixtures = fixtures();
    let mut executed = 0;
    macro_rules! run_pager {
        ($id:literal, $method:ident) => {{
            let server =
                Server::new(vec![Reply::json(200, fixtures[$id]["response"].clone())]).await;
            let client = client(&server);
            let mut pager = client
                .$method(serde_json::from_value(fixtures[$id]["parameters"].clone()).unwrap());
            let token = CancellationToken::new();
            token.cancel();
            assert!(
                matches!(
                    pager
                        .next_page_with_options(RequestOptions {
                            cancellation: Some(token),
                            ..Default::default()
                        })
                        .await
                        .unwrap_err()
                        .source,
                    KalshiError::Cancelled
                ),
                $id
            );
            assert_eq!(server.count(), 0);
            assert!(
                pager
                    .next_page_with_options(RequestOptions {
                        timeout: Some(Duration::from_secs(3)),
                        ..Default::default()
                    })
                    .await
                    .unwrap()
                    .is_some(),
                $id
            );
            assert_eq!(server.count(), 1);
            server.close().await;
            executed += 1;
        }};
    }
    run_pager!("GetTrades", get_trades_pages);
    run_pager!("GetMarkets", get_markets_pages);
    run_pager!("GetEvents", get_events_pages);
    run_pager!("GetMultivariateEvents", get_multivariate_events_pages);
    run_pager!("GetEventFeeChanges", get_event_fee_changes_pages);
    run_pager!("GetOrders", get_orders_pages);
    run_pager!(
        "GetIntraExchangeInstanceTransfers",
        get_intra_exchange_instance_transfers_pages
    );
    run_pager!("GetSubaccountTransfers", get_subaccount_transfers_pages);
    run_pager!("GetPositions", get_positions_pages);
    run_pager!("GetSettlements", get_settlements_pages);
    run_pager!("GetDeposits", get_deposits_pages);
    run_pager!("GetWithdrawals", get_withdrawals_pages);
    run_pager!("GetFills", get_fills_pages);
    run_pager!("GetBlockTradeProposals", get_block_trade_proposals_pages);
    run_pager!("GetRFQs", get_rf_qs_pages);
    run_pager!("GetQuotes", get_quotes_pages);
    run_pager!("GetStructuredTargets", get_structured_targets_pages);
    run_pager!("GetMilestones", get_milestones_pages);
    run_pager!(
        "GetMultivariateEventCollections",
        get_multivariate_event_collections_pages
    );
    run_pager!("GetIncentivePrograms", get_incentive_programs_pages);
    run_pager!("GetFCMFills", get_fcm_fills_pages);
    run_pager!("GetFCMOrders", get_fcm_orders_pages);
    run_pager!("GetFCMPositions", get_fcm_positions_pages);
    run_pager!("GetFillsHistorical", get_fills_historical_pages);
    run_pager!("GetHistoricalOrders", get_historical_orders_pages);
    run_pager!("GetHistoricalPositions", get_historical_positions_pages);
    run_pager!("GetTradesHistorical", get_trades_historical_pages);
    run_pager!("GetHistoricalMarkets", get_historical_markets_pages);
    assert_eq!(executed, 28);
    println!(
        "28 typed pagers: cancelled page sends zero; fresh options resume without cursor changes"
    );
}

#[tokio::test]
async fn named_methods_and_domain_facades_inherit_scoped_options_without_mutating_parent() {
    let server = Server::new(vec![Reply::json(200, json!({"markets":[],"cursor":""}))]).await;
    let client = client(&server);
    let token = CancellationToken::new();
    token.cancel();
    let scoped = client.with_request_options(RequestOptions {
        cancellation: Some(token),
        ..Default::default()
    });
    assert!(matches!(
        scoped
            .get_markets(&Default::default())
            .await
            .unwrap_err()
            .source,
        KalshiError::Cancelled
    ));
    assert!(matches!(
        scoped
            .markets()
            .get_markets(&Default::default())
            .await
            .unwrap_err()
            .source,
        KalshiError::Cancelled
    ));
    assert_eq!(server.count(), 0);
    client.get_markets(&Default::default()).await.unwrap();
    assert_eq!(server.count(), 1);
    server.close().await;
}

#[tokio::test]
async fn page_timeout_resumes_same_cursor_and_keeps_filters() {
    let server = Server::new(vec![
        Reply::json(200, json!({"markets":[],"cursor":"next cursor"})),
        Reply::StallBody(b"partial".to_vec()),
        Reply::json(200, json!({"markets":[],"cursor":""})),
    ])
    .await;
    let client = client(&server);
    let parameters = serde_json::from_value(json!({"series_ticker":"SERIES","limit":2})).unwrap();
    let mut pager = client.markets().get_markets_pages(parameters);
    pager.next_page().await.unwrap();
    let (timed_page, ()) = tokio::join!(
        pager.next_page_with_options(RequestOptions {
            timeout: Some(Duration::from_secs(1)),
            ..Default::default()
        }),
        server.wait_for(2)
    );
    assert_eq!(
        server.count(),
        2,
        "timed page must reach the scripted stalled response"
    );
    assert!(matches!(
        timed_page.unwrap_err().source,
        KalshiError::DeadlineExceeded
    ));
    pager.next_page().await.unwrap();
    assert!(pager.next_page().await.unwrap().is_none());
    let logs = server.logs.lock().unwrap().clone();
    assert_eq!(logs[1].lines().next(), logs[2].lines().next());
    assert!(logs[1].contains("cursor=next%20cursor"));
    assert!(logs.iter().all(|log| log.contains("series_ticker=SERIES")));
    server.close().await;
}

#[tokio::test]
async fn page_cancellation_during_io_resumes_same_cursor() {
    let server = Server::new(vec![
        Reply::StallHeaders,
        Reply::json(200, json!({"markets":[],"cursor":""})),
    ])
    .await;
    let client = client(&server);
    let mut pager = client.get_markets_pages(Default::default());
    let token = CancellationToken::new();
    let cancel = async {
        server.wait_for(1).await;
        token.cancel();
    };
    let page = pager.next_page_with_options(RequestOptions {
        cancellation: Some(token.clone()),
        ..Default::default()
    });
    let (result, ()) = tokio::join!(page, cancel);
    assert!(matches!(result.unwrap_err().source, KalshiError::Cancelled));
    pager.next_page().await.unwrap();
    assert!(pager.next_page().await.unwrap().is_none());
    let logs = server.logs.lock().unwrap().clone();
    assert_eq!(logs[0].lines().next(), logs[1].lines().next());
    server.close().await;
}

#[tokio::test]
async fn traversal_guards_cannot_be_erased_by_page_overrides_and_span_pages() {
    let server = Server::new(vec![
        Reply::json(200, json!({"markets":[],"cursor":"next"})),
        Reply::StallHeaders,
    ])
    .await;
    let client = client(&server);
    let deadline = Instant::now() + Duration::from_secs(1);
    let mut pager = client
        .get_markets_pages(Default::default())
        .with_deadline(deadline);
    pager.next_page().await.unwrap();
    assert!(matches!(
        pager
            .next_page_with_options(RequestOptions {
                timeout: Some(Duration::from_secs(5)),
                deadline: Some(Instant::now() + Duration::from_secs(5)),
                ..Default::default()
            })
            .await
            .unwrap_err()
            .source,
        KalshiError::DeadlineExceeded
    ));
    assert!(Instant::now() >= deadline);
    let first = CancellationToken::new();
    first.cancel();
    let second = CancellationToken::new();
    let mut pager = client
        .get_markets_pages(Default::default())
        .with_cancellation(first)
        .with_cancellation(second);
    assert!(matches!(
        pager
            .next_page_with_options(RequestOptions {
                cancellation: Some(CancellationToken::new()),
                ..Default::default()
            })
            .await
            .unwrap_err()
            .source,
        KalshiError::Cancelled
    ));
    assert_eq!(server.count(), 2);
    server.close().await;
}

#[tokio::test]
async fn typed_pager_error_payloads_remain_present_and_collect_options_work() {
    let fixture = fixtures();
    let server = Server::new(vec![
        Reply::json(500, fixture["GetMarkets"]["errors"]["500"].clone()),
        Reply::json(200, json!({"markets":[],"cursor":""})),
    ])
    .await;
    let client = client(&server);
    let mut pager = client.get_markets_pages(Default::default());
    let error = pager
        .next_page_with_options(Default::default())
        .await
        .unwrap_err();
    assert!(error.payload.is_some());
    assert!(matches!(error.source, KalshiError::Api(_)));
    let pages = pager
        .collect_pages_with_options(RequestOptions {
            timeout: Some(Duration::from_secs(1)),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(server.count(), 2);
    server.close().await;
}
