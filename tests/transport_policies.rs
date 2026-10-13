mod transport_support;
use kalshi_client::*;
use serde_json::json;
use std::{
    num::NonZeroU32,
    sync::Arc,
    time::{Duration, Instant as WallInstant},
};
use tokio::time::Instant;
use transport_support::*;

#[tokio::test]
async fn pre_cancel_expiry_and_invalid_requests_never_dispatch() {
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    let client = server.client(Default::default());
    let token = CancellationToken::new();
    token.cancel();
    assert!(matches!(
        client
            .request_with_options(
                "GetExchangeStatus",
                public_request(),
                RequestOptions {
                    cancellation: Some(token),
                    ..Default::default()
                }
            )
            .await,
        Err(KalshiError::Cancelled)
    ));
    assert!(matches!(
        client
            .request_with_options(
                "GetExchangeStatus",
                public_request(),
                RequestOptions {
                    timeout: Some(Duration::ZERO),
                    ..Default::default()
                }
            )
            .await,
        Err(KalshiError::DeadlineExceeded)
    ));
    assert!(matches!(
        client
            .request_with_options(
                "GetExchangeStatus",
                public_request(),
                RequestOptions {
                    deadline: Some(Instant::now()),
                    ..Default::default()
                }
            )
            .await,
        Err(KalshiError::DeadlineExceeded)
    ));
    assert!(client.request("GetMarket", public_request()).await.is_err());
    let mut request = public_request();
    request
        .headers
        .insert("invalid\nheader".into(), "FAKE_SECRET".into());
    assert!(client.request("GetExchangeStatus", request).await.is_err());
    assert_eq!(server.count(), 0);
    server.close().await;
}

#[tokio::test]
async fn total_deadline_includes_headers_body_and_trickle_without_replay() {
    for reply in [
        Reply::StallHeaders,
        Reply::StallBody(b"{\"partial\":".to_vec()),
        Reply::Trickle,
    ] {
        let server = Server::new(vec![reply]).await;
        let client = server.client(TransportOptions {
            total_timeout: Some(Duration::from_millis(35)),
            ..fast_retry()
        });
        assert!(matches!(
            get(&client).await,
            Err(KalshiError::DeadlineExceeded)
        ));
        assert_eq!(server.count(), 1);
        server.close().await;
    }
}

#[tokio::test]
async fn cancellation_and_future_abort_stop_active_io_and_subsequent_attempts() {
    for reply in [Reply::StallHeaders, Reply::StallBody(b"partial".to_vec())] {
        let server = Server::new(vec![reply]).await;
        let token = CancellationToken::new();
        let client = server
            .client(fast_retry())
            .with_request_options(RequestOptions {
                cancellation: Some(token.clone()),
                ..Default::default()
            });
        let task = tokio::spawn(async move { get(&client).await });
        server.wait_for(1).await;
        token.cancel();
        assert!(matches!(task.await.unwrap(), Err(KalshiError::Cancelled)));
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(server.count(), 1);
        server.close().await;
    }
    let server = Server::new(vec![Reply::StallHeaders]).await;
    let client = server.client(fast_retry());
    let task = tokio::spawn(async move { get(&client).await });
    server.wait_for(1).await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(server.count(), 1);
    server.close().await;
}

#[tokio::test]
async fn get_recovers_and_exhaustion_retains_final_raw_error() {
    let server = Server::new(vec![
        Reply::json(503, json!({"error":"first"})),
        Reply::json(200, json!({"recovered":true})),
    ])
    .await;
    assert_eq!(
        get(&server.client(fast_retry())).await.unwrap(),
        json!({"recovered":true})
    );
    assert_eq!(server.count(), 2);
    server.close().await;
    let bytes = b"\xff\0FAKE_SECRET_RAW".to_vec();
    let server = Server::new(vec![
        Reply::bytes(503, bytes.clone()).header("X-Secret", "FAKE_SECRET_HEADER"),
    ])
    .await;
    let error = get(&server.client(fast_retry())).await.unwrap_err();
    assert_eq!(server.count(), 3);
    assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
    if let KalshiError::Api(api) = error {
        assert_eq!(api.raw_body, bytes);
        assert_eq!(api.headers["x-secret"], "FAKE_SECRET_HEADER");
    } else {
        panic!("lost API error");
    }
    server.close().await;
}

#[tokio::test]
async fn attempt_timeout_and_incomplete_body_are_safe_get_retry_candidates() {
    for reply in [Reply::StallHeaders, Reply::Incomplete(b"partial".to_vec())] {
        let server = Server::new(vec![reply, Reply::json(200, json!({"recovered":true}))]).await;
        let client = server.client(TransportOptions {
            attempt_timeout: Some(Duration::from_millis(25)),
            ..fast_retry()
        });
        assert_eq!(get(&client).await.unwrap(), json!({"recovered":true}));
        assert_eq!(server.count(), 2);
        server.close().await;
    }
}

#[tokio::test]
async fn retry_after_is_a_minimum_and_does_not_overrun_total_budget() {
    let date = httpdate::fmt_http_date(std::time::SystemTime::now() + Duration::from_secs(60));
    for value in ["60", &date, "18446744073709551616", "18446744073709551615"] {
        let server = Server::new(vec![
            Reply::json(429, json!({"retry":"later"})).header("Retry-After", value),
        ])
        .await;
        let result = get(&server.client(TransportOptions {
            total_timeout: Some(Duration::from_millis(40)),
            ..fast_retry()
        }))
        .await;
        assert!(matches!(result, Err(KalshiError::Api(_))), "{value}");
        assert_eq!(server.count(), 1);
        server.close().await;
    }
    for value in ["invalid", "-1", "0", "Sun, 06 Nov 1994 08:49:37 GMT"] {
        let server = Server::new(vec![
            Reply::json(429, json!({})).header("Retry-After", value),
            Reply::json(200, json!({})),
        ])
        .await;
        get(&server.client(fast_retry())).await.unwrap();
        assert_eq!(server.count(), 2);
        server.close().await;
    }
    let server = Server::new(vec![
        Reply::json(429, json!({})).header("Retry-After", "1"),
        Reply::json(200, json!({})),
    ])
    .await;
    let start = WallInstant::now();
    get(&server.client(fast_retry())).await.unwrap();
    assert!(start.elapsed() >= Duration::from_secs(1));
    assert_eq!(server.count(), 2);
    server.close().await;
}

#[tokio::test]
async fn cancellation_during_retry_after_stops_dispatch() {
    let server = Server::new(vec![Reply::json(503, json!({})).header("Retry-After", "1")]).await;
    let token = CancellationToken::new();
    let client = server
        .client(fast_retry())
        .with_request_options(RequestOptions {
            cancellation: Some(token.clone()),
            ..Default::default()
        });
    let task = tokio::spawn(async move { get(&client).await });
    server.wait_for(1).await;
    token.cancel();
    assert!(matches!(task.await.unwrap(), Err(KalshiError::Cancelled)));
    assert_eq!(server.count(), 1);
    server.close().await;
}

#[tokio::test]
async fn json_and_terminal_statuses_never_retry() {
    for reply in [
        Reply::bytes(200, b"{broken FAKE_SECRET".to_vec()),
        Reply::json(401, json!({})),
        Reply::json(409, json!({})),
        Reply::json(500, json!({})),
    ] {
        let server = Server::new(vec![reply]).await;
        let error = get(&server.client(fast_retry())).await.unwrap_err();
        assert_eq!(server.count(), 1);
        assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
        server.close().await;
    }
}

#[tokio::test]
async fn every_current_mutation_is_sent_once_under_all_failure_modes() {
    let mutations: Vec<_> = OPERATIONS.iter().filter(|op| op.method != "GET").collect();
    assert!(
        mutations
            .iter()
            .any(|op| op.operation_id == "CreateOrderV2")
    );
    assert!(
        mutations
            .iter()
            .any(|op| op.operation_id == "CancelOrderV2")
    );
    // This pinned inventory has POST/PUT/DELETE; it has no PATCH operation.
    for reply in [
        Reply::json(429, json!({})),
        Reply::json(503, json!({})),
        Reply::Drop,
        Reply::Incomplete(b"partial".to_vec()),
        Reply::StallBody(b"partial".to_vec()),
    ] {
        let server = Server::new(vec![reply]).await;
        let client = server.client(TransportOptions {
            attempt_timeout: Some(Duration::from_millis(20)),
            ..fast_retry()
        });
        for (index, operation) in mutations.iter().enumerate() {
            assert!(
                client
                    .request(
                        operation.operation_id,
                        mutation_request(operation.operation_id)
                    )
                    .await
                    .is_err()
            );
            assert_eq!(
                server.count(),
                index + 1,
                "{} replayed",
                operation.operation_id
            );
        }
        println!(
            "No replay: {} mutations, one physical HTTP/1 send each",
            mutations.len()
        );
        server.close().await;
    }
}

#[tokio::test]
async fn redirect_refusal_overrides_caller_policy_and_never_contacts_target() {
    let target = Server::new(vec![Reply::json(200, json!({}))]).await;
    for status in [301, 302, 303, 307, 308] {
        for location in [
            format!("{}/stolen", target.environment.rest_base_url),
            "/relative".into(),
            "https://example.invalid/downgrade".into(),
        ] {
            let server = Server::new(vec![
                Reply::bytes(status, b"redirect".to_vec()).header("Location", &location),
            ])
            .await;
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("x-private", "FAKE_SECRET_DEFAULT".parse().unwrap());
            let client = KalshiClient::builder(server.environment)
                .http_builder(|http| {
                    http.no_proxy()
                        .default_headers(headers)
                        .redirect(reqwest::redirect::Policy::limited(10))
                        .retry(reqwest::retry::for_host("127.0.0.1").max_retries_per_request(10))
                })
                .transport_options(fast_retry())
                .build()
                .unwrap();
            let mut request = public_request();
            request.headers.insert(
                "KALSHI-ACCESS-SIGNATURE".into(),
                "FAKE_SECRET_SIGNATURE".into(),
            );
            let error = client
                .request("GetExchangeStatus", request)
                .await
                .unwrap_err();
            assert!(matches!(error, KalshiError::Api(_)));
            assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
            assert_eq!(server.count(), 1);
            assert_eq!(target.count(), 0);
            server.close().await;
        }
    }
    target.close().await;
}

#[tokio::test]
async fn shared_rate_costs_buckets_cancellation_and_deadlines_cover_queueing() {
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    let quota = RateQuota::with_period(Duration::from_millis(80))
        .unwrap()
        .allow_burst(NonZeroU32::new(2).unwrap());
    let limiter = LocalRateLimiter::new(RateLimitOptions {
        read: Some(quota),
        write: Some(quota),
        costs: [(
            "GetExchangeStatus".into(),
            OperationCost {
                bucket: RateBucket::Read,
                units: NonZeroU32::new(2).unwrap(),
            },
        )]
        .into(),
    })
    .unwrap();
    let client = KalshiClient::builder(server.environment)
        .http_builder(|http| http.no_proxy())
        .rate_limiter(limiter.clone())
        .build()
        .unwrap();
    let first = WallInstant::now();
    get(&client).await.unwrap();
    let token = CancellationToken::new();
    let blocked = client.with_request_options(RequestOptions {
        cancellation: Some(token.clone()),
        ..Default::default()
    });
    let task = tokio::spawn(async move { get(&blocked).await });
    tokio::task::yield_now().await;
    token.cancel();
    assert!(matches!(task.await.unwrap(), Err(KalshiError::Cancelled)));
    let other = KalshiClient::builder(server.environment)
        .http_builder(|http| http.no_proxy())
        .rate_limiter(limiter)
        .build()
        .unwrap();
    assert!(matches!(
        get(&other.with_request_options(RequestOptions {
            timeout: Some(Duration::from_millis(10)),
            ..Default::default()
        }))
        .await,
        Err(KalshiError::DeadlineExceeded)
    ));
    assert_eq!(server.count(), 1);
    // Write bucket remains available independently of the saturated read bucket.
    client
        .request("CreateOrderV2", mutation_request("CreateOrderV2"))
        .await
        .unwrap();
    assert_eq!(server.count(), 2);
    get(&other).await.unwrap();
    assert!(first.elapsed() >= Duration::from_millis(160));
    assert_eq!(server.count(), 3);
    server.close().await;
}

#[tokio::test]
async fn each_get_retry_reacquires_shared_rate_capacity() {
    let server = Server::new(vec![
        Reply::json(503, json!({})),
        Reply::json(200, json!({})),
    ])
    .await;
    let limiter = LocalRateLimiter::new(RateLimitOptions {
        read: Some(RateQuota::with_period(Duration::from_millis(50)).unwrap()),
        ..Default::default()
    })
    .unwrap();
    let client = KalshiClient::builder(server.environment)
        .http_builder(|http| http.no_proxy())
        .transport_options(fast_retry())
        .rate_limiter(limiter)
        .build()
        .unwrap();
    let start = WallInstant::now();
    get(&client).await.unwrap();
    assert!(start.elapsed() >= Duration::from_millis(45));
    assert_eq!(server.count(), 2);
    server.close().await;
}

#[tokio::test]
async fn concurrent_clones_share_one_admission_quota() {
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    let limiter = LocalRateLimiter::new(RateLimitOptions {
        read: Some(RateQuota::with_period(Duration::from_millis(20)).unwrap()),
        ..Default::default()
    })
    .unwrap();
    let client = KalshiClient::builder(server.environment)
        .http_builder(|http| http.no_proxy())
        .rate_limiter(limiter)
        .build()
        .unwrap();
    let start = WallInstant::now();
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..6 {
        let client = client.clone();
        tasks.spawn(async move { get(&client).await.unwrap() });
    }
    while let Some(result) = tasks.join_next().await {
        result.unwrap();
    }
    assert_eq!(server.count(), 6);
    assert!(start.elapsed() >= Duration::from_millis(95));
    server.close().await;
}

#[tokio::test]
async fn impossible_cost_and_bad_options_fail_before_wire() {
    for quota in [
        RateQuota::per_second(NonZeroU32::MAX),
        RateQuota::with_period(Duration::MAX).unwrap(),
        RateQuota::with_period(Duration::from_nanos(u64::MAX)).unwrap(),
    ] {
        assert!(
            LocalRateLimiter::new(RateLimitOptions {
                read: Some(quota),
                ..Default::default()
            })
            .is_err()
        );
    }
    assert!(
        LocalRateLimiter::new(RateLimitOptions {
            read: Some(RateQuota::per_second(NonZeroU32::MIN)),
            costs: [(
                "GetExchangeStatus".into(),
                OperationCost {
                    bucket: RateBucket::Read,
                    units: NonZeroU32::new(2).unwrap()
                }
            )]
            .into(),
            ..Default::default()
        })
        .is_err()
    );
    assert!(
        LocalRateLimiter::new(RateLimitOptions {
            costs: [(
                "UnknownOperation".into(),
                OperationCost {
                    bucket: RateBucket::Read,
                    units: NonZeroU32::MIN
                }
            )]
            .into(),
            ..Default::default()
        })
        .is_err()
    );
    let server = Server::new(vec![Reply::json(200, json!({}))]).await;
    for options in [
        TransportOptions {
            max_response_bytes: 0,
            ..Default::default()
        },
        TransportOptions {
            retry: RetryPolicy::safe_get(0),
            ..Default::default()
        },
        TransportOptions {
            total_timeout: Some(Duration::MAX),
            ..Default::default()
        },
    ] {
        assert!(
            KalshiClient::builder(server.environment)
                .transport_options(options)
                .build()
                .is_err()
        );
    }
    assert_eq!(server.count(), 0);
    server.close().await;
}

#[tokio::test]
async fn http_error_source_chain_has_no_query_secret() {
    let server = Server::new(vec![Reply::Drop]).await;
    let mut request = public_request();
    request
        .query
        .push(("secret".into(), QueryValue::from("FAKE_SECRET_QUERY")));
    let error = server
        .client(Default::default())
        .request("GetExchangeStatus", request)
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
    let mut source = std::error::Error::source(&error);
    while let Some(error) = source {
        assert!(!format!("{error:?} {error}").contains("FAKE_SECRET"));
        source = error.source();
    }
    server.close().await;
}

#[tokio::test]
async fn all_mutations_ignore_http2_refused_stream_nacks_even_with_caller_retry_policy() {
    use tokio::{net::TcpListener, task::JoinSet};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let environment = environment(listener.local_addr().unwrap());
    let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let stop = CancellationToken::new();
    let (observed, shutdown) = (count.clone(), stop.clone());
    let server = tokio::spawn(async move {
        let mut tasks = JoinSet::new();
        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                accepted = listener.accept() => {
                    let (socket,_) = accepted.unwrap(); let observed = observed.clone();
                    tasks.spawn(async move {
                        let mut connection = h2::server::handshake(socket).await.unwrap();
                        while let Some(Ok((_,mut response))) = connection.accept().await {
                            observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            response.send_reset(h2::Reason::REFUSED_STREAM);
                        }
                    });
                }
            }
        }
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
    });
    let client = KalshiClient::builder(environment)
        .http_builder(|http| {
            http.no_proxy()
                .http2_prior_knowledge()
                .retry(reqwest::retry::for_host("127.0.0.1").max_retries_per_request(10))
        })
        .transport_options(fast_retry())
        .build()
        .unwrap();
    for (index, operation) in OPERATIONS
        .iter()
        .filter(|op| op.method != "GET")
        .enumerate()
    {
        assert!(
            client
                .request(
                    operation.operation_id,
                    mutation_request(operation.operation_id)
                )
                .await
                .is_err()
        );
        assert_eq!(
            count.load(std::sync::atomic::Ordering::SeqCst),
            index + 1,
            "HTTP/2 replayed {}",
            operation.operation_id
        );
    }
    println!(
        "No hidden HTTP/2 REFUSED_STREAM replay: {} mutation streams",
        count.load(std::sync::atomic::Ordering::SeqCst)
    );
    stop.cancel();
    server.await.unwrap();
}
