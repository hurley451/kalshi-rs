// Generated documented-status error probes.
mod typed_support;
use kalshi_client::models::*;
use serde_json::Value;
#[tokio::test]
async fn all_documented_errors_keep_structured_source_and_typed_payload() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
    let auth = typed_support::auth();
    {
        let fixture = &fixtures["GetExchangeStatus"];
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_exchange_status()
                .await
                .expect_err("GetExchangeStatus 500");
            assert!(
                error.payload.is_some(),
                "GetExchangeStatus 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(503, fixture["errors"]["503"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_exchange_status()
                .await
                .expect_err("GetExchangeStatus 503");
            assert!(
                error.payload.is_some(),
                "GetExchangeStatus 503 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 503);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["503"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(504, fixture["errors"]["504"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_exchange_status()
                .await
                .expect_err("GetExchangeStatus 504");
            assert!(
                error.payload.is_some(),
                "GetExchangeStatus 504 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 504);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["504"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSeriesFeeChanges"];
        let parameters: GetSeriesFeeChangesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series_fee_changes(&parameters)
                .await
                .expect_err("GetSeriesFeeChanges 400");
            assert!(
                error.payload.is_some(),
                "GetSeriesFeeChanges 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series_fee_changes(&parameters)
                .await
                .expect_err("GetSeriesFeeChanges 500");
            assert!(
                error.payload.is_some(),
                "GetSeriesFeeChanges 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetExchangeSchedule"];
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_exchange_schedule()
                .await
                .expect_err("GetExchangeSchedule 500");
            assert!(
                error.payload.is_some(),
                "GetExchangeSchedule 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetUserDataTimestamp"];
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_user_data_timestamp()
                .await
                .expect_err("GetUserDataTimestamp 500");
            assert!(
                error.payload.is_some(),
                "GetUserDataTimestamp 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarketCandlesticks"];
        let parameters: GetMarketCandlesticksParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks(&parameters)
                .await
                .expect_err("GetMarketCandlesticks 400");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticks 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks(&parameters)
                .await
                .expect_err("GetMarketCandlesticks 404");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticks 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks(&parameters)
                .await
                .expect_err("GetMarketCandlesticks 500");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticks 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetTrades"];
        let parameters: GetTradesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_trades(&parameters)
                .await
                .expect_err("GetTrades 400");
            assert!(error.payload.is_some(), "GetTrades 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_trades(&parameters)
                .await
                .expect_err("GetTrades 500");
            assert!(error.payload.is_some(), "GetTrades 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarketOrderbook"];
        let parameters: GetMarketOrderbookParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbook(&parameters)
                .await
                .expect_err("GetMarketOrderbook 401");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbook 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbook(&parameters)
                .await
                .expect_err("GetMarketOrderbook 404");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbook 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbook(&parameters)
                .await
                .expect_err("GetMarketOrderbook 500");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbook 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarketOrderbooks"];
        let parameters: GetMarketOrderbooksParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbooks(&parameters)
                .await
                .expect_err("GetMarketOrderbooks 400");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbooks 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbooks(&parameters)
                .await
                .expect_err("GetMarketOrderbooks 401");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbooks 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_orderbooks(&parameters)
                .await
                .expect_err("GetMarketOrderbooks 500");
            assert!(
                error.payload.is_some(),
                "GetMarketOrderbooks 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSeries"];
        let parameters: GetSeriesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series(&parameters)
                .await
                .expect_err("GetSeries 400");
            assert!(error.payload.is_some(), "GetSeries 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series(&parameters)
                .await
                .expect_err("GetSeries 500");
            assert!(error.payload.is_some(), "GetSeries 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSeriesList"];
        let parameters: GetSeriesListParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series_list(&parameters)
                .await
                .expect_err("GetSeriesList 400");
            assert!(
                error.payload.is_some(),
                "GetSeriesList 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_series_list(&parameters)
                .await
                .expect_err("GetSeriesList 500");
            assert!(
                error.payload.is_some(),
                "GetSeriesList 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarkets"];
        let parameters: GetMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_markets(&parameters)
                .await
                .expect_err("GetMarkets 400");
            assert!(
                error.payload.is_some(),
                "GetMarkets 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_markets(&parameters)
                .await
                .expect_err("GetMarkets 401");
            assert!(
                error.payload.is_some(),
                "GetMarkets 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_markets(&parameters)
                .await
                .expect_err("GetMarkets 500");
            assert!(
                error.payload.is_some(),
                "GetMarkets 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarket"];
        let parameters: GetMarketParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market(&parameters)
                .await
                .expect_err("GetMarket 401");
            assert!(error.payload.is_some(), "GetMarket 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market(&parameters)
                .await
                .expect_err("GetMarket 404");
            assert!(error.payload.is_some(), "GetMarket 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market(&parameters)
                .await
                .expect_err("GetMarket 500");
            assert!(error.payload.is_some(), "GetMarket 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["BatchGetMarketCandlesticks"];
        let parameters: BatchGetMarketCandlesticksParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_get_market_candlesticks(&parameters)
                .await
                .expect_err("BatchGetMarketCandlesticks 400");
            assert!(
                error.payload.is_some(),
                "BatchGetMarketCandlesticks 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_get_market_candlesticks(&parameters)
                .await
                .expect_err("BatchGetMarketCandlesticks 401");
            assert!(
                error.payload.is_some(),
                "BatchGetMarketCandlesticks 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_get_market_candlesticks(&parameters)
                .await
                .expect_err("BatchGetMarketCandlesticks 500");
            assert!(
                error.payload.is_some(),
                "BatchGetMarketCandlesticks 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarketCandlesticksByEvent"];
        let parameters: GetMarketCandlesticksByEventParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_by_event(&parameters)
                .await
                .expect_err("GetMarketCandlesticksByEvent 400");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksByEvent 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_by_event(&parameters)
                .await
                .expect_err("GetMarketCandlesticksByEvent 401");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksByEvent 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_by_event(&parameters)
                .await
                .expect_err("GetMarketCandlesticksByEvent 500");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksByEvent 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEvents"];
        let parameters: GetEventsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_events(&parameters)
                .await
                .expect_err("GetEvents 400");
            assert!(error.payload.is_some(), "GetEvents 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_events(&parameters)
                .await
                .expect_err("GetEvents 401");
            assert!(error.payload.is_some(), "GetEvents 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_events(&parameters)
                .await
                .expect_err("GetEvents 500");
            assert!(error.payload.is_some(), "GetEvents 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMultivariateEvents"];
        let parameters: GetMultivariateEventsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_events(&parameters)
                .await
                .expect_err("GetMultivariateEvents 400");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEvents 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_events(&parameters)
                .await
                .expect_err("GetMultivariateEvents 401");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEvents 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_events(&parameters)
                .await
                .expect_err("GetMultivariateEvents 500");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEvents 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEventFeeChanges"];
        let parameters: GetEventFeeChangesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_fee_changes(&parameters)
                .await
                .expect_err("GetEventFeeChanges 400");
            assert!(
                error.payload.is_some(),
                "GetEventFeeChanges 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_fee_changes(&parameters)
                .await
                .expect_err("GetEventFeeChanges 500");
            assert!(
                error.payload.is_some(),
                "GetEventFeeChanges 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEvent"];
        let parameters: GetEventParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event(&parameters)
                .await
                .expect_err("GetEvent 400");
            assert!(error.payload.is_some(), "GetEvent 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event(&parameters)
                .await
                .expect_err("GetEvent 401");
            assert!(error.payload.is_some(), "GetEvent 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event(&parameters)
                .await
                .expect_err("GetEvent 404");
            assert!(error.payload.is_some(), "GetEvent 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event(&parameters)
                .await
                .expect_err("GetEvent 500");
            assert!(error.payload.is_some(), "GetEvent 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEventMetadata"];
        let parameters: GetEventMetadataParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_metadata(&parameters)
                .await
                .expect_err("GetEventMetadata 400");
            assert!(
                error.payload.is_some(),
                "GetEventMetadata 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_metadata(&parameters)
                .await
                .expect_err("GetEventMetadata 401");
            assert!(
                error.payload.is_some(),
                "GetEventMetadata 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_metadata(&parameters)
                .await
                .expect_err("GetEventMetadata 404");
            assert!(
                error.payload.is_some(),
                "GetEventMetadata 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_metadata(&parameters)
                .await
                .expect_err("GetEventMetadata 500");
            assert!(
                error.payload.is_some(),
                "GetEventMetadata 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEventForecastPercentilesHistory"];
        let parameters: GetEventForecastPercentilesHistoryParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_forecast_percentiles_history(&parameters)
                .await
                .expect_err("GetEventForecastPercentilesHistory 400");
            assert!(
                error.payload.is_some(),
                "GetEventForecastPercentilesHistory 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_forecast_percentiles_history(&parameters)
                .await
                .expect_err("GetEventForecastPercentilesHistory 401");
            assert!(
                error.payload.is_some(),
                "GetEventForecastPercentilesHistory 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_forecast_percentiles_history(&parameters)
                .await
                .expect_err("GetEventForecastPercentilesHistory 500");
            assert!(
                error.payload.is_some(),
                "GetEventForecastPercentilesHistory 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrders"];
        let parameters: GetOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_orders(&parameters)
                .await
                .expect_err("GetOrders 400");
            assert!(error.payload.is_some(), "GetOrders 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_orders(&parameters)
                .await
                .expect_err("GetOrders 401");
            assert!(error.payload.is_some(), "GetOrders 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_orders(&parameters)
                .await
                .expect_err("GetOrders 500");
            assert!(error.payload.is_some(), "GetOrders 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrder"];
        let parameters: GetOrderParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order(&parameters)
                .await
                .expect_err("GetOrder 401");
            assert!(error.payload.is_some(), "GetOrder 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order(&parameters)
                .await
                .expect_err("GetOrder 404");
            assert!(error.payload.is_some(), "GetOrder 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order(&parameters)
                .await
                .expect_err("GetOrder 500");
            assert!(error.payload.is_some(), "GetOrder 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrderQueuePositions"];
        let parameters: GetOrderQueuePositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_positions(&parameters)
                .await
                .expect_err("GetOrderQueuePositions 400");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePositions 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_positions(&parameters)
                .await
                .expect_err("GetOrderQueuePositions 401");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePositions 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_positions(&parameters)
                .await
                .expect_err("GetOrderQueuePositions 500");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePositions 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrderQueuePosition"];
        let parameters: GetOrderQueuePositionParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_position(&parameters)
                .await
                .expect_err("GetOrderQueuePosition 401");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePosition 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_position(&parameters)
                .await
                .expect_err("GetOrderQueuePosition 404");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePosition 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_queue_position(&parameters)
                .await
                .expect_err("GetOrderQueuePosition 500");
            assert!(
                error.payload.is_some(),
                "GetOrderQueuePosition 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateOrderV2"];
        let body: CreateOrderV2Body = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 400");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 401");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 403");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 409");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 409 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(429, fixture["errors"]["429"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 429");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 429 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 429);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["429"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 500");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(503, fixture["errors"]["503"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_v2(&body)
                .await
                .expect_err("CreateOrderV2 503");
            assert!(
                error.payload.is_some(),
                "CreateOrderV2 503 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 503);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["503"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CancelAllOrders"];
        let parameters: CancelAllOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_all_orders(&parameters)
                .await
                .expect_err("CancelAllOrders 401");
            assert!(
                error.payload.is_some(),
                "CancelAllOrders 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(429, fixture["errors"]["429"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_all_orders(&parameters)
                .await
                .expect_err("CancelAllOrders 429");
            assert!(
                error.payload.is_some(),
                "CancelAllOrders 429 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 429);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["429"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_all_orders(&parameters)
                .await
                .expect_err("CancelAllOrders 500");
            assert!(
                error.payload.is_some(),
                "CancelAllOrders 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["BatchCreateOrdersV2"];
        let body: BatchCreateOrdersV2Body =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_create_orders_v2(&body)
                .await
                .expect_err("BatchCreateOrdersV2 400");
            assert!(
                error.payload.is_some(),
                "BatchCreateOrdersV2 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_create_orders_v2(&body)
                .await
                .expect_err("BatchCreateOrdersV2 401");
            assert!(
                error.payload.is_some(),
                "BatchCreateOrdersV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_create_orders_v2(&body)
                .await
                .expect_err("BatchCreateOrdersV2 403");
            assert!(
                error.payload.is_some(),
                "BatchCreateOrdersV2 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_create_orders_v2(&body)
                .await
                .expect_err("BatchCreateOrdersV2 500");
            assert!(
                error.payload.is_some(),
                "BatchCreateOrdersV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["BatchCancelOrdersV2"];
        let body: BatchCancelOrdersV2Body =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_cancel_orders_v2(&body)
                .await
                .expect_err("BatchCancelOrdersV2 400");
            assert!(
                error.payload.is_some(),
                "BatchCancelOrdersV2 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_cancel_orders_v2(&body)
                .await
                .expect_err("BatchCancelOrdersV2 401");
            assert!(
                error.payload.is_some(),
                "BatchCancelOrdersV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_cancel_orders_v2(&body)
                .await
                .expect_err("BatchCancelOrdersV2 403");
            assert!(
                error.payload.is_some(),
                "BatchCancelOrdersV2 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .batch_cancel_orders_v2(&body)
                .await
                .expect_err("BatchCancelOrdersV2 500");
            assert!(
                error.payload.is_some(),
                "BatchCancelOrdersV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CancelOrderV2"];
        let parameters: CancelOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_order_v2(&parameters)
                .await
                .expect_err("CancelOrderV2 401");
            assert!(
                error.payload.is_some(),
                "CancelOrderV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_order_v2(&parameters)
                .await
                .expect_err("CancelOrderV2 404");
            assert!(
                error.payload.is_some(),
                "CancelOrderV2 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .cancel_order_v2(&parameters)
                .await
                .expect_err("CancelOrderV2 500");
            assert!(
                error.payload.is_some(),
                "CancelOrderV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["AmendOrderV2"];
        let parameters: AmendOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: AmendOrderV2Body = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 400");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 401");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 403");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 404");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 500");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(503, fixture["errors"]["503"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .amend_order_v2(&parameters, &body)
                .await
                .expect_err("AmendOrderV2 503");
            assert!(
                error.payload.is_some(),
                "AmendOrderV2 503 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 503);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["503"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DecreaseOrderV2"];
        let parameters: DecreaseOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: DecreaseOrderV2Body = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .decrease_order_v2(&parameters, &body)
                .await
                .expect_err("DecreaseOrderV2 400");
            assert!(
                error.payload.is_some(),
                "DecreaseOrderV2 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .decrease_order_v2(&parameters, &body)
                .await
                .expect_err("DecreaseOrderV2 401");
            assert!(
                error.payload.is_some(),
                "DecreaseOrderV2 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .decrease_order_v2(&parameters, &body)
                .await
                .expect_err("DecreaseOrderV2 404");
            assert!(
                error.payload.is_some(),
                "DecreaseOrderV2 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .decrease_order_v2(&parameters, &body)
                .await
                .expect_err("DecreaseOrderV2 500");
            assert!(
                error.payload.is_some(),
                "DecreaseOrderV2 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrderGroups"];
        let parameters: GetOrderGroupsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_groups(&parameters)
                .await
                .expect_err("GetOrderGroups 400");
            assert!(
                error.payload.is_some(),
                "GetOrderGroups 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_groups(&parameters)
                .await
                .expect_err("GetOrderGroups 401");
            assert!(
                error.payload.is_some(),
                "GetOrderGroups 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_groups(&parameters)
                .await
                .expect_err("GetOrderGroups 500");
            assert!(
                error.payload.is_some(),
                "GetOrderGroups 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateOrderGroup"];
        let body: CreateOrderGroupBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_group(&body)
                .await
                .expect_err("CreateOrderGroup 400");
            assert!(
                error.payload.is_some(),
                "CreateOrderGroup 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_group(&body)
                .await
                .expect_err("CreateOrderGroup 401");
            assert!(
                error.payload.is_some(),
                "CreateOrderGroup 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_order_group(&body)
                .await
                .expect_err("CreateOrderGroup 500");
            assert!(
                error.payload.is_some(),
                "CreateOrderGroup 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetOrderGroup"];
        let parameters: GetOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_group(&parameters)
                .await
                .expect_err("GetOrderGroup 401");
            assert!(
                error.payload.is_some(),
                "GetOrderGroup 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_group(&parameters)
                .await
                .expect_err("GetOrderGroup 404");
            assert!(
                error.payload.is_some(),
                "GetOrderGroup 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_order_group(&parameters)
                .await
                .expect_err("GetOrderGroup 500");
            assert!(
                error.payload.is_some(),
                "GetOrderGroup 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteOrderGroup"];
        let parameters: DeleteOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_order_group(&parameters)
                .await
                .expect_err("DeleteOrderGroup 401");
            assert!(
                error.payload.is_some(),
                "DeleteOrderGroup 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_order_group(&parameters)
                .await
                .expect_err("DeleteOrderGroup 404");
            assert!(
                error.payload.is_some(),
                "DeleteOrderGroup 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_order_group(&parameters)
                .await
                .expect_err("DeleteOrderGroup 500");
            assert!(
                error.payload.is_some(),
                "DeleteOrderGroup 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ResetOrderGroup"];
        let parameters: ResetOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: ResetOrderGroupBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .reset_order_group(&parameters, Some(&body))
                .await
                .expect_err("ResetOrderGroup 401");
            assert!(
                error.payload.is_some(),
                "ResetOrderGroup 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .reset_order_group(&parameters, Some(&body))
                .await
                .expect_err("ResetOrderGroup 404");
            assert!(
                error.payload.is_some(),
                "ResetOrderGroup 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .reset_order_group(&parameters, Some(&body))
                .await
                .expect_err("ResetOrderGroup 500");
            assert!(
                error.payload.is_some(),
                "ResetOrderGroup 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["TriggerOrderGroup"];
        let parameters: TriggerOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: TriggerOrderGroupBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .trigger_order_group(&parameters, Some(&body))
                .await
                .expect_err("TriggerOrderGroup 401");
            assert!(
                error.payload.is_some(),
                "TriggerOrderGroup 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .trigger_order_group(&parameters, Some(&body))
                .await
                .expect_err("TriggerOrderGroup 404");
            assert!(
                error.payload.is_some(),
                "TriggerOrderGroup 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .trigger_order_group(&parameters, Some(&body))
                .await
                .expect_err("TriggerOrderGroup 500");
            assert!(
                error.payload.is_some(),
                "TriggerOrderGroup 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["UpdateOrderGroupLimit"];
        let parameters: UpdateOrderGroupLimitParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: UpdateOrderGroupLimitBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_order_group_limit(&parameters, &body)
                .await
                .expect_err("UpdateOrderGroupLimit 400");
            assert!(
                error.payload.is_some(),
                "UpdateOrderGroupLimit 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_order_group_limit(&parameters, &body)
                .await
                .expect_err("UpdateOrderGroupLimit 401");
            assert!(
                error.payload.is_some(),
                "UpdateOrderGroupLimit 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_order_group_limit(&parameters, &body)
                .await
                .expect_err("UpdateOrderGroupLimit 404");
            assert!(
                error.payload.is_some(),
                "UpdateOrderGroupLimit 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_order_group_limit(&parameters, &body)
                .await
                .expect_err("UpdateOrderGroupLimit 500");
            assert!(
                error.payload.is_some(),
                "UpdateOrderGroupLimit 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetBalance"];
        let parameters: GetBalanceParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_balance(&parameters)
                .await
                .expect_err("GetBalance 401");
            assert!(
                error.payload.is_some(),
                "GetBalance 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_balance(&parameters)
                .await
                .expect_err("GetBalance 500");
            assert!(
                error.payload.is_some(),
                "GetBalance 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["IntraExchangeInstanceTransfer"];
        let body: IntraExchangeInstanceTransferBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .intra_exchange_instance_transfer(&body)
                .await
                .expect_err("IntraExchangeInstanceTransfer 400");
            assert!(
                error.payload.is_some(),
                "IntraExchangeInstanceTransfer 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .intra_exchange_instance_transfer(&body)
                .await
                .expect_err("IntraExchangeInstanceTransfer 401");
            assert!(
                error.payload.is_some(),
                "IntraExchangeInstanceTransfer 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .intra_exchange_instance_transfer(&body)
                .await
                .expect_err("IntraExchangeInstanceTransfer 403");
            assert!(
                error.payload.is_some(),
                "IntraExchangeInstanceTransfer 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .intra_exchange_instance_transfer(&body)
                .await
                .expect_err("IntraExchangeInstanceTransfer 500");
            assert!(
                error.payload.is_some(),
                "IntraExchangeInstanceTransfer 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetIntraExchangeInstanceTransfers"];
        let parameters: GetIntraExchangeInstanceTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfers(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfers 400");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfers 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfers(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfers 401");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfers 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfers(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfers 500");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfers 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetIntraExchangeInstanceTransfer"];
        let parameters: GetIntraExchangeInstanceTransferParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfer(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfer 401");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfer 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfer(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfer 404");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfer 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_intra_exchange_instance_transfer(&parameters)
                .await
                .expect_err("GetIntraExchangeInstanceTransfer 500");
            assert!(
                error.payload.is_some(),
                "GetIntraExchangeInstanceTransfer 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateSubaccount"];
        let body: CreateSubaccountBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_subaccount(Some(&body))
                .await
                .expect_err("CreateSubaccount 400");
            assert!(
                error.payload.is_some(),
                "CreateSubaccount 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_subaccount(Some(&body))
                .await
                .expect_err("CreateSubaccount 401");
            assert!(
                error.payload.is_some(),
                "CreateSubaccount 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_subaccount(Some(&body))
                .await
                .expect_err("CreateSubaccount 500");
            assert!(
                error.payload.is_some(),
                "CreateSubaccount 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ApplySubaccountTransfer"];
        let body: ApplySubaccountTransferBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .apply_subaccount_transfer(&body)
                .await
                .expect_err("ApplySubaccountTransfer 400");
            assert!(
                error.payload.is_some(),
                "ApplySubaccountTransfer 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .apply_subaccount_transfer(&body)
                .await
                .expect_err("ApplySubaccountTransfer 401");
            assert!(
                error.payload.is_some(),
                "ApplySubaccountTransfer 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .apply_subaccount_transfer(&body)
                .await
                .expect_err("ApplySubaccountTransfer 500");
            assert!(
                error.payload.is_some(),
                "ApplySubaccountTransfer 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSubaccountBalances"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_balances()
                .await
                .expect_err("GetSubaccountBalances 401");
            assert!(
                error.payload.is_some(),
                "GetSubaccountBalances 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_balances()
                .await
                .expect_err("GetSubaccountBalances 500");
            assert!(
                error.payload.is_some(),
                "GetSubaccountBalances 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSubaccountTransfers"];
        let parameters: GetSubaccountTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_transfers(&parameters)
                .await
                .expect_err("GetSubaccountTransfers 401");
            assert!(
                error.payload.is_some(),
                "GetSubaccountTransfers 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_transfers(&parameters)
                .await
                .expect_err("GetSubaccountTransfers 500");
            assert!(
                error.payload.is_some(),
                "GetSubaccountTransfers 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSubaccountNetting"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_netting()
                .await
                .expect_err("GetSubaccountNetting 401");
            assert!(
                error.payload.is_some(),
                "GetSubaccountNetting 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_subaccount_netting()
                .await
                .expect_err("GetSubaccountNetting 500");
            assert!(
                error.payload.is_some(),
                "GetSubaccountNetting 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["UpdateSubaccountNetting"];
        let body: UpdateSubaccountNettingBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_subaccount_netting(&body)
                .await
                .expect_err("UpdateSubaccountNetting 400");
            assert!(
                error.payload.is_some(),
                "UpdateSubaccountNetting 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_subaccount_netting(&body)
                .await
                .expect_err("UpdateSubaccountNetting 401");
            assert!(
                error.payload.is_some(),
                "UpdateSubaccountNetting 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_subaccount_netting(&body)
                .await
                .expect_err("UpdateSubaccountNetting 500");
            assert!(
                error.payload.is_some(),
                "UpdateSubaccountNetting 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetPositions"];
        let parameters: GetPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_positions(&parameters)
                .await
                .expect_err("GetPositions 400");
            assert!(
                error.payload.is_some(),
                "GetPositions 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_positions(&parameters)
                .await
                .expect_err("GetPositions 401");
            assert!(
                error.payload.is_some(),
                "GetPositions 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_positions(&parameters)
                .await
                .expect_err("GetPositions 500");
            assert!(
                error.payload.is_some(),
                "GetPositions 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetSettlements"];
        let parameters: GetSettlementsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_settlements(&parameters)
                .await
                .expect_err("GetSettlements 400");
            assert!(
                error.payload.is_some(),
                "GetSettlements 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_settlements(&parameters)
                .await
                .expect_err("GetSettlements 401");
            assert!(
                error.payload.is_some(),
                "GetSettlements 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_settlements(&parameters)
                .await
                .expect_err("GetSettlements 500");
            assert!(
                error.payload.is_some(),
                "GetSettlements 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetDeposits"];
        let parameters: GetDepositsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_deposits(&parameters)
                .await
                .expect_err("GetDeposits 400");
            assert!(
                error.payload.is_some(),
                "GetDeposits 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_deposits(&parameters)
                .await
                .expect_err("GetDeposits 401");
            assert!(
                error.payload.is_some(),
                "GetDeposits 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_deposits(&parameters)
                .await
                .expect_err("GetDeposits 500");
            assert!(
                error.payload.is_some(),
                "GetDeposits 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetWithdrawals"];
        let parameters: GetWithdrawalsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_withdrawals(&parameters)
                .await
                .expect_err("GetWithdrawals 400");
            assert!(
                error.payload.is_some(),
                "GetWithdrawals 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_withdrawals(&parameters)
                .await
                .expect_err("GetWithdrawals 401");
            assert!(
                error.payload.is_some(),
                "GetWithdrawals 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_withdrawals(&parameters)
                .await
                .expect_err("GetWithdrawals 500");
            assert!(
                error.payload.is_some(),
                "GetWithdrawals 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetPortfolioRestingOrderTotalValue"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_portfolio_resting_order_total_value()
                .await
                .expect_err("GetPortfolioRestingOrderTotalValue 401");
            assert!(
                error.payload.is_some(),
                "GetPortfolioRestingOrderTotalValue 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_portfolio_resting_order_total_value()
                .await
                .expect_err("GetPortfolioRestingOrderTotalValue 500");
            assert!(
                error.payload.is_some(),
                "GetPortfolioRestingOrderTotalValue 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFills"];
        let parameters: GetFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills(&parameters)
                .await
                .expect_err("GetFills 400");
            assert!(error.payload.is_some(), "GetFills 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills(&parameters)
                .await
                .expect_err("GetFills 401");
            assert!(error.payload.is_some(), "GetFills 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills(&parameters)
                .await
                .expect_err("GetFills 500");
            assert!(error.payload.is_some(), "GetFills 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetCommunicationsID"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_communications_id()
                .await
                .expect_err("GetCommunicationsID 401");
            assert!(
                error.payload.is_some(),
                "GetCommunicationsID 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_communications_id()
                .await
                .expect_err("GetCommunicationsID 500");
            assert!(
                error.payload.is_some(),
                "GetCommunicationsID 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetBlockTradeProposals"];
        let parameters: GetBlockTradeProposalsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_block_trade_proposals(&parameters)
                .await
                .expect_err("GetBlockTradeProposals 400");
            assert!(
                error.payload.is_some(),
                "GetBlockTradeProposals 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_block_trade_proposals(&parameters)
                .await
                .expect_err("GetBlockTradeProposals 401");
            assert!(
                error.payload.is_some(),
                "GetBlockTradeProposals 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_block_trade_proposals(&parameters)
                .await
                .expect_err("GetBlockTradeProposals 500");
            assert!(
                error.payload.is_some(),
                "GetBlockTradeProposals 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ProposeBlockTrade"];
        let body: ProposeBlockTradeBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .propose_block_trade(&body)
                .await
                .expect_err("ProposeBlockTrade 400");
            assert!(
                error.payload.is_some(),
                "ProposeBlockTrade 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .propose_block_trade(&body)
                .await
                .expect_err("ProposeBlockTrade 401");
            assert!(
                error.payload.is_some(),
                "ProposeBlockTrade 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .propose_block_trade(&body)
                .await
                .expect_err("ProposeBlockTrade 403");
            assert!(
                error.payload.is_some(),
                "ProposeBlockTrade 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .propose_block_trade(&body)
                .await
                .expect_err("ProposeBlockTrade 500");
            assert!(
                error.payload.is_some(),
                "ProposeBlockTrade 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["AcceptBlockTradeProposal"];
        let parameters: AcceptBlockTradeProposalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: AcceptBlockTradeProposalBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_block_trade_proposal(&parameters, Some(&body))
                .await
                .expect_err("AcceptBlockTradeProposal 400");
            assert!(
                error.payload.is_some(),
                "AcceptBlockTradeProposal 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_block_trade_proposal(&parameters, Some(&body))
                .await
                .expect_err("AcceptBlockTradeProposal 401");
            assert!(
                error.payload.is_some(),
                "AcceptBlockTradeProposal 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_block_trade_proposal(&parameters, Some(&body))
                .await
                .expect_err("AcceptBlockTradeProposal 404");
            assert!(
                error.payload.is_some(),
                "AcceptBlockTradeProposal 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_block_trade_proposal(&parameters, Some(&body))
                .await
                .expect_err("AcceptBlockTradeProposal 500");
            assert!(
                error.payload.is_some(),
                "AcceptBlockTradeProposal 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetRFQs"];
        let parameters: GetRfQsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rf_qs(&parameters)
                .await
                .expect_err("GetRFQs 400");
            assert!(error.payload.is_some(), "GetRFQs 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rf_qs(&parameters)
                .await
                .expect_err("GetRFQs 401");
            assert!(error.payload.is_some(), "GetRFQs 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rf_qs(&parameters)
                .await
                .expect_err("GetRFQs 500");
            assert!(error.payload.is_some(), "GetRFQs 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateRFQ"];
        let body: CreateRfqBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client.create_rfq(&body).await.expect_err("CreateRFQ 400");
            assert!(error.payload.is_some(), "CreateRFQ 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client.create_rfq(&body).await.expect_err("CreateRFQ 401");
            assert!(error.payload.is_some(), "CreateRFQ 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client.create_rfq(&body).await.expect_err("CreateRFQ 409");
            assert!(error.payload.is_some(), "CreateRFQ 409 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client.create_rfq(&body).await.expect_err("CreateRFQ 500");
            assert!(error.payload.is_some(), "CreateRFQ 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetRFQ"];
        let parameters: GetRfqParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client.get_rfq(&parameters).await.expect_err("GetRFQ 400");
            assert!(error.payload.is_some(), "GetRFQ 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client.get_rfq(&parameters).await.expect_err("GetRFQ 401");
            assert!(error.payload.is_some(), "GetRFQ 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client.get_rfq(&parameters).await.expect_err("GetRFQ 404");
            assert!(error.payload.is_some(), "GetRFQ 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client.get_rfq(&parameters).await.expect_err("GetRFQ 500");
            assert!(error.payload.is_some(), "GetRFQ 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteRFQ"];
        let parameters: DeleteRfqParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq(&parameters)
                .await
                .expect_err("DeleteRFQ 400");
            assert!(error.payload.is_some(), "DeleteRFQ 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq(&parameters)
                .await
                .expect_err("DeleteRFQ 401");
            assert!(error.payload.is_some(), "DeleteRFQ 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq(&parameters)
                .await
                .expect_err("DeleteRFQ 404");
            assert!(error.payload.is_some(), "DeleteRFQ 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq(&parameters)
                .await
                .expect_err("DeleteRFQ 500");
            assert!(error.payload.is_some(), "DeleteRFQ 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetRFQQuote"];
        let parameters: GetRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rfq_quote(&parameters)
                .await
                .expect_err("GetRFQQuote 400");
            assert!(
                error.payload.is_some(),
                "GetRFQQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rfq_quote(&parameters)
                .await
                .expect_err("GetRFQQuote 401");
            assert!(
                error.payload.is_some(),
                "GetRFQQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rfq_quote(&parameters)
                .await
                .expect_err("GetRFQQuote 404");
            assert!(
                error.payload.is_some(),
                "GetRFQQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_rfq_quote(&parameters)
                .await
                .expect_err("GetRFQQuote 500");
            assert!(
                error.payload.is_some(),
                "GetRFQQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteRFQQuote"];
        let parameters: DeleteRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq_quote(&parameters)
                .await
                .expect_err("DeleteRFQQuote 400");
            assert!(
                error.payload.is_some(),
                "DeleteRFQQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq_quote(&parameters)
                .await
                .expect_err("DeleteRFQQuote 401");
            assert!(
                error.payload.is_some(),
                "DeleteRFQQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq_quote(&parameters)
                .await
                .expect_err("DeleteRFQQuote 404");
            assert!(
                error.payload.is_some(),
                "DeleteRFQQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_rfq_quote(&parameters)
                .await
                .expect_err("DeleteRFQQuote 500");
            assert!(
                error.payload.is_some(),
                "DeleteRFQQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["AcceptRFQQuote"];
        let parameters: AcceptRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: AcceptRfqQuoteBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_rfq_quote(&parameters, &body)
                .await
                .expect_err("AcceptRFQQuote 400");
            assert!(
                error.payload.is_some(),
                "AcceptRFQQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_rfq_quote(&parameters, &body)
                .await
                .expect_err("AcceptRFQQuote 401");
            assert!(
                error.payload.is_some(),
                "AcceptRFQQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_rfq_quote(&parameters, &body)
                .await
                .expect_err("AcceptRFQQuote 404");
            assert!(
                error.payload.is_some(),
                "AcceptRFQQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_rfq_quote(&parameters, &body)
                .await
                .expect_err("AcceptRFQQuote 500");
            assert!(
                error.payload.is_some(),
                "AcceptRFQQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ConfirmRFQQuote"];
        let parameters: ConfirmRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: ConfirmRfqQuoteBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_rfq_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmRFQQuote 400");
            assert!(
                error.payload.is_some(),
                "ConfirmRFQQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_rfq_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmRFQQuote 401");
            assert!(
                error.payload.is_some(),
                "ConfirmRFQQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_rfq_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmRFQQuote 404");
            assert!(
                error.payload.is_some(),
                "ConfirmRFQQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_rfq_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmRFQQuote 500");
            assert!(
                error.payload.is_some(),
                "ConfirmRFQQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetQuotes"];
        let parameters: GetQuotesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quotes(&parameters)
                .await
                .expect_err("GetQuotes 400");
            assert!(error.payload.is_some(), "GetQuotes 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quotes(&parameters)
                .await
                .expect_err("GetQuotes 401");
            assert!(error.payload.is_some(), "GetQuotes 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quotes(&parameters)
                .await
                .expect_err("GetQuotes 500");
            assert!(error.payload.is_some(), "GetQuotes 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateQuote"];
        let body: CreateQuoteBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_quote(&body)
                .await
                .expect_err("CreateQuote 400");
            assert!(
                error.payload.is_some(),
                "CreateQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_quote(&body)
                .await
                .expect_err("CreateQuote 401");
            assert!(
                error.payload.is_some(),
                "CreateQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_quote(&body)
                .await
                .expect_err("CreateQuote 500");
            assert!(
                error.payload.is_some(),
                "CreateQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetQuote"];
        let parameters: GetQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quote(&parameters)
                .await
                .expect_err("GetQuote 400");
            assert!(error.payload.is_some(), "GetQuote 400 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quote(&parameters)
                .await
                .expect_err("GetQuote 401");
            assert!(error.payload.is_some(), "GetQuote 401 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quote(&parameters)
                .await
                .expect_err("GetQuote 404");
            assert!(error.payload.is_some(), "GetQuote 404 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_quote(&parameters)
                .await
                .expect_err("GetQuote 500");
            assert!(error.payload.is_some(), "GetQuote 500 missing typed error");
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteQuote"];
        let parameters: DeleteQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_quote(&parameters)
                .await
                .expect_err("DeleteQuote 400");
            assert!(
                error.payload.is_some(),
                "DeleteQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_quote(&parameters)
                .await
                .expect_err("DeleteQuote 401");
            assert!(
                error.payload.is_some(),
                "DeleteQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_quote(&parameters)
                .await
                .expect_err("DeleteQuote 404");
            assert!(
                error.payload.is_some(),
                "DeleteQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_quote(&parameters)
                .await
                .expect_err("DeleteQuote 500");
            assert!(
                error.payload.is_some(),
                "DeleteQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["AcceptQuote"];
        let parameters: AcceptQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: AcceptQuoteBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_quote(&parameters, &body)
                .await
                .expect_err("AcceptQuote 400");
            assert!(
                error.payload.is_some(),
                "AcceptQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_quote(&parameters, &body)
                .await
                .expect_err("AcceptQuote 401");
            assert!(
                error.payload.is_some(),
                "AcceptQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_quote(&parameters, &body)
                .await
                .expect_err("AcceptQuote 404");
            assert!(
                error.payload.is_some(),
                "AcceptQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .accept_quote(&parameters, &body)
                .await
                .expect_err("AcceptQuote 500");
            assert!(
                error.payload.is_some(),
                "AcceptQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ConfirmQuote"];
        let parameters: ConfirmQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: ConfirmQuoteBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmQuote 400");
            assert!(
                error.payload.is_some(),
                "ConfirmQuote 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmQuote 401");
            assert!(
                error.payload.is_some(),
                "ConfirmQuote 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmQuote 404");
            assert!(
                error.payload.is_some(),
                "ConfirmQuote 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .confirm_quote(&parameters, Some(&body))
                .await
                .expect_err("ConfirmQuote 500");
            assert!(
                error.payload.is_some(),
                "ConfirmQuote 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetApiKeys"];
        let parameters: GetApiKeysParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_api_keys(&parameters)
                .await
                .expect_err("GetApiKeys 401");
            assert!(
                error.payload.is_some(),
                "GetApiKeys 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_api_keys(&parameters)
                .await
                .expect_err("GetApiKeys 500");
            assert!(
                error.payload.is_some(),
                "GetApiKeys 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateApiKey"];
        let body: CreateApiKeyBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_api_key(&body)
                .await
                .expect_err("CreateApiKey 400");
            assert!(
                error.payload.is_some(),
                "CreateApiKey 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_api_key(&body)
                .await
                .expect_err("CreateApiKey 401");
            assert!(
                error.payload.is_some(),
                "CreateApiKey 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_api_key(&body)
                .await
                .expect_err("CreateApiKey 403");
            assert!(
                error.payload.is_some(),
                "CreateApiKey 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_api_key(&body)
                .await
                .expect_err("CreateApiKey 409");
            assert!(
                error.payload.is_some(),
                "CreateApiKey 409 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_api_key(&body)
                .await
                .expect_err("CreateApiKey 500");
            assert!(
                error.payload.is_some(),
                "CreateApiKey 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GenerateApiKey"];
        let body: GenerateApiKeyBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .generate_api_key(&body)
                .await
                .expect_err("GenerateApiKey 400");
            assert!(
                error.payload.is_some(),
                "GenerateApiKey 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .generate_api_key(&body)
                .await
                .expect_err("GenerateApiKey 401");
            assert!(
                error.payload.is_some(),
                "GenerateApiKey 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .generate_api_key(&body)
                .await
                .expect_err("GenerateApiKey 403");
            assert!(
                error.payload.is_some(),
                "GenerateApiKey 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .generate_api_key(&body)
                .await
                .expect_err("GenerateApiKey 409");
            assert!(
                error.payload.is_some(),
                "GenerateApiKey 409 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .generate_api_key(&body)
                .await
                .expect_err("GenerateApiKey 500");
            assert!(
                error.payload.is_some(),
                "GenerateApiKey 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteApiKey"];
        let parameters: DeleteApiKeyParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_api_key(&parameters)
                .await
                .expect_err("DeleteApiKey 400");
            assert!(
                error.payload.is_some(),
                "DeleteApiKey 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_api_key(&parameters)
                .await
                .expect_err("DeleteApiKey 401");
            assert!(
                error.payload.is_some(),
                "DeleteApiKey 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_api_key(&parameters)
                .await
                .expect_err("DeleteApiKey 404");
            assert!(
                error.payload.is_some(),
                "DeleteApiKey 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_api_key(&parameters)
                .await
                .expect_err("DeleteApiKey 500");
            assert!(
                error.payload.is_some(),
                "DeleteApiKey 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetAccountApiLimits"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_account_api_limits()
                .await
                .expect_err("GetAccountApiLimits 401");
            assert!(
                error.payload.is_some(),
                "GetAccountApiLimits 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_account_api_limits()
                .await
                .expect_err("GetAccountApiLimits 500");
            assert!(
                error.payload.is_some(),
                "GetAccountApiLimits 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["UpgradeAccountApiUsageLevel"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .upgrade_account_api_usage_level()
                .await
                .expect_err("UpgradeAccountApiUsageLevel 401");
            assert!(
                error.payload.is_some(),
                "UpgradeAccountApiUsageLevel 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .upgrade_account_api_usage_level()
                .await
                .expect_err("UpgradeAccountApiUsageLevel 403");
            assert!(
                error.payload.is_some(),
                "UpgradeAccountApiUsageLevel 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(429, fixture["errors"]["429"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .upgrade_account_api_usage_level()
                .await
                .expect_err("UpgradeAccountApiUsageLevel 429");
            assert!(
                error.payload.is_some(),
                "UpgradeAccountApiUsageLevel 429 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 429);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["429"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .upgrade_account_api_usage_level()
                .await
                .expect_err("UpgradeAccountApiUsageLevel 500");
            assert!(
                error.payload.is_some(),
                "UpgradeAccountApiUsageLevel 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetAccountApiUsageLevelVolumeProgress"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_account_api_usage_level_volume_progress()
                .await
                .expect_err("GetAccountApiUsageLevelVolumeProgress 401");
            assert!(
                error.payload.is_some(),
                "GetAccountApiUsageLevelVolumeProgress 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_account_api_usage_level_volume_progress()
                .await
                .expect_err("GetAccountApiUsageLevelVolumeProgress 500");
            assert!(
                error.payload.is_some(),
                "GetAccountApiUsageLevelVolumeProgress 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetAccountEndpointCosts"];
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_account_endpoint_costs()
                .await
                .expect_err("GetAccountEndpointCosts 500");
            assert!(
                error.payload.is_some(),
                "GetAccountEndpointCosts 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetTagsForSeriesCategories"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_tags_for_series_categories()
                .await
                .expect_err("GetTagsForSeriesCategories 401");
            assert!(
                error.payload.is_some(),
                "GetTagsForSeriesCategories 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_tags_for_series_categories()
                .await
                .expect_err("GetTagsForSeriesCategories 500");
            assert!(
                error.payload.is_some(),
                "GetTagsForSeriesCategories 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFiltersForSports"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_filters_for_sports()
                .await
                .expect_err("GetFiltersForSports 401");
            assert!(
                error.payload.is_some(),
                "GetFiltersForSports 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_filters_for_sports()
                .await
                .expect_err("GetFiltersForSports 500");
            assert!(
                error.payload.is_some(),
                "GetFiltersForSports 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetLiveDataByMilestone"];
        let parameters: GetLiveDataByMilestoneParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_live_data_by_milestone(&parameters)
                .await
                .expect_err("GetLiveDataByMilestone 404");
            assert!(
                error.payload.is_some(),
                "GetLiveDataByMilestone 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_live_data_by_milestone(&parameters)
                .await
                .expect_err("GetLiveDataByMilestone 500");
            assert!(
                error.payload.is_some(),
                "GetLiveDataByMilestone 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetLiveData"];
        let parameters: GetLiveDataParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_live_data(&parameters)
                .await
                .expect_err("GetLiveData 404");
            assert!(
                error.payload.is_some(),
                "GetLiveData 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_live_data(&parameters)
                .await
                .expect_err("GetLiveData 500");
            assert!(
                error.payload.is_some(),
                "GetLiveData 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetLiveDatas"];
        let parameters: GetLiveDatasParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_live_datas(&parameters)
                .await
                .expect_err("GetLiveDatas 500");
            assert!(
                error.payload.is_some(),
                "GetLiveDatas 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetGameStats"];
        let parameters: GetGameStatsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_game_stats(&parameters)
                .await
                .expect_err("GetGameStats 404");
            assert!(
                error.payload.is_some(),
                "GetGameStats 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_game_stats(&parameters)
                .await
                .expect_err("GetGameStats 500");
            assert!(
                error.payload.is_some(),
                "GetGameStats 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetEventLiveData"];
        let parameters: GetEventLiveDataParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_live_data(&parameters)
                .await
                .expect_err("GetEventLiveData 404");
            assert!(
                error.payload.is_some(),
                "GetEventLiveData 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_event_live_data(&parameters)
                .await
                .expect_err("GetEventLiveData 500");
            assert!(
                error.payload.is_some(),
                "GetEventLiveData 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetWeatherIndex"];
        let parameters: GetWeatherIndexParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_weather_index(&parameters)
                .await
                .expect_err("GetWeatherIndex 400");
            assert!(
                error.payload.is_some(),
                "GetWeatherIndex 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_weather_index(&parameters)
                .await
                .expect_err("GetWeatherIndex 500");
            assert!(
                error.payload.is_some(),
                "GetWeatherIndex 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetWeatherIndexCalibrations"];
        let parameters: GetWeatherIndexCalibrationsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_weather_index_calibrations(&parameters)
                .await
                .expect_err("GetWeatherIndexCalibrations 400");
            assert!(
                error.payload.is_some(),
                "GetWeatherIndexCalibrations 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_weather_index_calibrations(&parameters)
                .await
                .expect_err("GetWeatherIndexCalibrations 500");
            assert!(
                error.payload.is_some(),
                "GetWeatherIndexCalibrations 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetStructuredTargets"];
        let parameters: GetStructuredTargetsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_structured_targets(&parameters)
                .await
                .expect_err("GetStructuredTargets 401");
            assert!(
                error.payload.is_some(),
                "GetStructuredTargets 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_structured_targets(&parameters)
                .await
                .expect_err("GetStructuredTargets 500");
            assert!(
                error.payload.is_some(),
                "GetStructuredTargets 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetStructuredTarget"];
        let parameters: GetStructuredTargetParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_structured_target(&parameters)
                .await
                .expect_err("GetStructuredTarget 401");
            assert!(
                error.payload.is_some(),
                "GetStructuredTarget 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_structured_target(&parameters)
                .await
                .expect_err("GetStructuredTarget 404");
            assert!(
                error.payload.is_some(),
                "GetStructuredTarget 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_structured_target(&parameters)
                .await
                .expect_err("GetStructuredTarget 500");
            assert!(
                error.payload.is_some(),
                "GetStructuredTarget 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMilestone"];
        let parameters: GetMilestoneParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestone(&parameters)
                .await
                .expect_err("GetMilestone 400");
            assert!(
                error.payload.is_some(),
                "GetMilestone 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestone(&parameters)
                .await
                .expect_err("GetMilestone 401");
            assert!(
                error.payload.is_some(),
                "GetMilestone 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestone(&parameters)
                .await
                .expect_err("GetMilestone 404");
            assert!(
                error.payload.is_some(),
                "GetMilestone 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestone(&parameters)
                .await
                .expect_err("GetMilestone 500");
            assert!(
                error.payload.is_some(),
                "GetMilestone 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMilestones"];
        let parameters: GetMilestonesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestones(&parameters)
                .await
                .expect_err("GetMilestones 400");
            assert!(
                error.payload.is_some(),
                "GetMilestones 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestones(&parameters)
                .await
                .expect_err("GetMilestones 401");
            assert!(
                error.payload.is_some(),
                "GetMilestones 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_milestones(&parameters)
                .await
                .expect_err("GetMilestones 500");
            assert!(
                error.payload.is_some(),
                "GetMilestones 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMultivariateEventCollection"];
        let parameters: GetMultivariateEventCollectionParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_event_collection(&parameters)
                .await
                .expect_err("GetMultivariateEventCollection 400");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEventCollection 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_event_collection(&parameters)
                .await
                .expect_err("GetMultivariateEventCollection 404");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEventCollection 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_event_collection(&parameters)
                .await
                .expect_err("GetMultivariateEventCollection 500");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEventCollection 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateMarketInMultivariateEventCollection"];
        let parameters: CreateMarketInMultivariateEventCollectionParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let body: CreateMarketInMultivariateEventCollectionBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_market_in_multivariate_event_collection(&parameters, &body)
                .await
                .expect_err("CreateMarketInMultivariateEventCollection 400");
            assert!(
                error.payload.is_some(),
                "CreateMarketInMultivariateEventCollection 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_market_in_multivariate_event_collection(&parameters, &body)
                .await
                .expect_err("CreateMarketInMultivariateEventCollection 401");
            assert!(
                error.payload.is_some(),
                "CreateMarketInMultivariateEventCollection 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(429, fixture["errors"]["429"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_market_in_multivariate_event_collection(&parameters, &body)
                .await
                .expect_err("CreateMarketInMultivariateEventCollection 429");
            assert!(
                error.payload.is_some(),
                "CreateMarketInMultivariateEventCollection 429 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 429);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["429"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_market_in_multivariate_event_collection(&parameters, &body)
                .await
                .expect_err("CreateMarketInMultivariateEventCollection 500");
            assert!(
                error.payload.is_some(),
                "CreateMarketInMultivariateEventCollection 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMultivariateEventCollections"];
        let parameters: GetMultivariateEventCollectionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_event_collections(&parameters)
                .await
                .expect_err("GetMultivariateEventCollections 400");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEventCollections 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_multivariate_event_collections(&parameters)
                .await
                .expect_err("GetMultivariateEventCollections 500");
            assert!(
                error.payload.is_some(),
                "GetMultivariateEventCollections 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetIncentivePrograms"];
        let parameters: GetIncentiveProgramsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_incentive_programs(&parameters)
                .await
                .expect_err("GetIncentivePrograms 400");
            assert!(
                error.payload.is_some(),
                "GetIncentivePrograms 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_incentive_programs(&parameters)
                .await
                .expect_err("GetIncentivePrograms 500");
            assert!(
                error.payload.is_some(),
                "GetIncentivePrograms 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFCMFills"];
        let parameters: GetFcmFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_fills(&parameters)
                .await
                .expect_err("GetFCMFills 400");
            assert!(
                error.payload.is_some(),
                "GetFCMFills 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_fills(&parameters)
                .await
                .expect_err("GetFCMFills 401");
            assert!(
                error.payload.is_some(),
                "GetFCMFills 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_fills(&parameters)
                .await
                .expect_err("GetFCMFills 403");
            assert!(
                error.payload.is_some(),
                "GetFCMFills 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_fills(&parameters)
                .await
                .expect_err("GetFCMFills 500");
            assert!(
                error.payload.is_some(),
                "GetFCMFills 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFCMOrders"];
        let parameters: GetFcmOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_orders(&parameters)
                .await
                .expect_err("GetFCMOrders 400");
            assert!(
                error.payload.is_some(),
                "GetFCMOrders 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_orders(&parameters)
                .await
                .expect_err("GetFCMOrders 401");
            assert!(
                error.payload.is_some(),
                "GetFCMOrders 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_orders(&parameters)
                .await
                .expect_err("GetFCMOrders 403");
            assert!(
                error.payload.is_some(),
                "GetFCMOrders 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_orders(&parameters)
                .await
                .expect_err("GetFCMOrders 404");
            assert!(
                error.payload.is_some(),
                "GetFCMOrders 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_orders(&parameters)
                .await
                .expect_err("GetFCMOrders 500");
            assert!(
                error.payload.is_some(),
                "GetFCMOrders 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["ListFCMSubtraders"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .list_fcm_subtraders()
                .await
                .expect_err("ListFCMSubtraders 401");
            assert!(
                error.payload.is_some(),
                "ListFCMSubtraders 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .list_fcm_subtraders()
                .await
                .expect_err("ListFCMSubtraders 403");
            assert!(
                error.payload.is_some(),
                "ListFCMSubtraders 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .list_fcm_subtraders()
                .await
                .expect_err("ListFCMSubtraders 500");
            assert!(
                error.payload.is_some(),
                "ListFCMSubtraders 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateFCMSubtrader"];
        let body: CreateFcmSubtraderBody = serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_subtrader(&body)
                .await
                .expect_err("CreateFCMSubtrader 400");
            assert!(
                error.payload.is_some(),
                "CreateFCMSubtrader 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_subtrader(&body)
                .await
                .expect_err("CreateFCMSubtrader 401");
            assert!(
                error.payload.is_some(),
                "CreateFCMSubtrader 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_subtrader(&body)
                .await
                .expect_err("CreateFCMSubtrader 403");
            assert!(
                error.payload.is_some(),
                "CreateFCMSubtrader 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_subtrader(&body)
                .await
                .expect_err("CreateFCMSubtrader 409");
            assert!(
                error.payload.is_some(),
                "CreateFCMSubtrader 409 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_subtrader(&body)
                .await
                .expect_err("CreateFCMSubtrader 500");
            assert!(
                error.payload.is_some(),
                "CreateFCMSubtrader 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFCMEventContractDailyCap"];
        let parameters: GetFcmEventContractDailyCapParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("GetFCMEventContractDailyCap 400");
            assert!(
                error.payload.is_some(),
                "GetFCMEventContractDailyCap 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("GetFCMEventContractDailyCap 401");
            assert!(
                error.payload.is_some(),
                "GetFCMEventContractDailyCap 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("GetFCMEventContractDailyCap 403");
            assert!(
                error.payload.is_some(),
                "GetFCMEventContractDailyCap 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("GetFCMEventContractDailyCap 404");
            assert!(
                error.payload.is_some(),
                "GetFCMEventContractDailyCap 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("GetFCMEventContractDailyCap 500");
            assert!(
                error.payload.is_some(),
                "GetFCMEventContractDailyCap 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["UpdateFCMEventContractDailyCap"];
        let body: UpdateFcmEventContractDailyCapBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_event_contract_daily_cap(&body)
                .await
                .expect_err("UpdateFCMEventContractDailyCap 400");
            assert!(
                error.payload.is_some(),
                "UpdateFCMEventContractDailyCap 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_event_contract_daily_cap(&body)
                .await
                .expect_err("UpdateFCMEventContractDailyCap 401");
            assert!(
                error.payload.is_some(),
                "UpdateFCMEventContractDailyCap 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_event_contract_daily_cap(&body)
                .await
                .expect_err("UpdateFCMEventContractDailyCap 403");
            assert!(
                error.payload.is_some(),
                "UpdateFCMEventContractDailyCap 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_event_contract_daily_cap(&body)
                .await
                .expect_err("UpdateFCMEventContractDailyCap 404");
            assert!(
                error.payload.is_some(),
                "UpdateFCMEventContractDailyCap 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_event_contract_daily_cap(&body)
                .await
                .expect_err("UpdateFCMEventContractDailyCap 500");
            assert!(
                error.payload.is_some(),
                "UpdateFCMEventContractDailyCap 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["DeleteFCMEventContractDailyCap"];
        let parameters: DeleteFcmEventContractDailyCapParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("DeleteFCMEventContractDailyCap 400");
            assert!(
                error.payload.is_some(),
                "DeleteFCMEventContractDailyCap 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("DeleteFCMEventContractDailyCap 401");
            assert!(
                error.payload.is_some(),
                "DeleteFCMEventContractDailyCap 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("DeleteFCMEventContractDailyCap 403");
            assert!(
                error.payload.is_some(),
                "DeleteFCMEventContractDailyCap 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("DeleteFCMEventContractDailyCap 404");
            assert!(
                error.payload.is_some(),
                "DeleteFCMEventContractDailyCap 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .delete_fcm_event_contract_daily_cap(&parameters)
                .await
                .expect_err("DeleteFCMEventContractDailyCap 500");
            assert!(
                error.payload.is_some(),
                "DeleteFCMEventContractDailyCap 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFCMSubtraderBlockedCategories"];
        let parameters: GetFcmSubtraderBlockedCategoriesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_subtrader_blocked_categories(&parameters)
                .await
                .expect_err("GetFCMSubtraderBlockedCategories 400");
            assert!(
                error.payload.is_some(),
                "GetFCMSubtraderBlockedCategories 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_subtrader_blocked_categories(&parameters)
                .await
                .expect_err("GetFCMSubtraderBlockedCategories 401");
            assert!(
                error.payload.is_some(),
                "GetFCMSubtraderBlockedCategories 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_subtrader_blocked_categories(&parameters)
                .await
                .expect_err("GetFCMSubtraderBlockedCategories 403");
            assert!(
                error.payload.is_some(),
                "GetFCMSubtraderBlockedCategories 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_subtrader_blocked_categories(&parameters)
                .await
                .expect_err("GetFCMSubtraderBlockedCategories 500");
            assert!(
                error.payload.is_some(),
                "GetFCMSubtraderBlockedCategories 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["UpdateFCMSubtraderBlockedCategories"];
        let body: UpdateFcmSubtraderBlockedCategoriesBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_subtrader_blocked_categories(&body)
                .await
                .expect_err("UpdateFCMSubtraderBlockedCategories 400");
            assert!(
                error.payload.is_some(),
                "UpdateFCMSubtraderBlockedCategories 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_subtrader_blocked_categories(&body)
                .await
                .expect_err("UpdateFCMSubtraderBlockedCategories 401");
            assert!(
                error.payload.is_some(),
                "UpdateFCMSubtraderBlockedCategories 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_subtrader_blocked_categories(&body)
                .await
                .expect_err("UpdateFCMSubtraderBlockedCategories 403");
            assert!(
                error.payload.is_some(),
                "UpdateFCMSubtraderBlockedCategories 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_subtrader_blocked_categories(&body)
                .await
                .expect_err("UpdateFCMSubtraderBlockedCategories 404");
            assert!(
                error.payload.is_some(),
                "UpdateFCMSubtraderBlockedCategories 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .update_fcm_subtrader_blocked_categories(&body)
                .await
                .expect_err("UpdateFCMSubtraderBlockedCategories 500");
            assert!(
                error.payload.is_some(),
                "UpdateFCMSubtraderBlockedCategories 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetTargetBalanceAllocation"];
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_target_balance_allocation()
                .await
                .expect_err("GetTargetBalanceAllocation 401");
            assert!(
                error.payload.is_some(),
                "GetTargetBalanceAllocation 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_target_balance_allocation()
                .await
                .expect_err("GetTargetBalanceAllocation 403");
            assert!(
                error.payload.is_some(),
                "GetTargetBalanceAllocation 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_target_balance_allocation()
                .await
                .expect_err("GetTargetBalanceAllocation 500");
            assert!(
                error.payload.is_some(),
                "GetTargetBalanceAllocation 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["SetTargetBalanceAllocation"];
        let body: SetTargetBalanceAllocationBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .set_target_balance_allocation(&body)
                .await
                .expect_err("SetTargetBalanceAllocation 400");
            assert!(
                error.payload.is_some(),
                "SetTargetBalanceAllocation 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .set_target_balance_allocation(&body)
                .await
                .expect_err("SetTargetBalanceAllocation 401");
            assert!(
                error.payload.is_some(),
                "SetTargetBalanceAllocation 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .set_target_balance_allocation(&body)
                .await
                .expect_err("SetTargetBalanceAllocation 403");
            assert!(
                error.payload.is_some(),
                "SetTargetBalanceAllocation 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .set_target_balance_allocation(&body)
                .await
                .expect_err("SetTargetBalanceAllocation 500");
            assert!(
                error.payload.is_some(),
                "SetTargetBalanceAllocation 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFCMPositions"];
        let parameters: GetFcmPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_positions(&parameters)
                .await
                .expect_err("GetFCMPositions 400");
            assert!(
                error.payload.is_some(),
                "GetFCMPositions 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_positions(&parameters)
                .await
                .expect_err("GetFCMPositions 401");
            assert!(
                error.payload.is_some(),
                "GetFCMPositions 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_positions(&parameters)
                .await
                .expect_err("GetFCMPositions 403");
            assert!(
                error.payload.is_some(),
                "GetFCMPositions 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_positions(&parameters)
                .await
                .expect_err("GetFCMPositions 404");
            assert!(
                error.payload.is_some(),
                "GetFCMPositions 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fcm_positions(&parameters)
                .await
                .expect_err("GetFCMPositions 500");
            assert!(
                error.payload.is_some(),
                "GetFCMPositions 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["CreateFCMPositionTransfer"];
        let body: CreateFcmPositionTransferBody =
            serde_json::from_value(fixture["body"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 400");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 401");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(403, fixture["errors"]["403"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 403");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 403 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 403);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["403"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 404");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(409, fixture["errors"]["409"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 409");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 409 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 409);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["409"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(429, fixture["errors"]["429"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 429");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 429 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 429);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["429"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .create_fcm_position_transfer(&body)
                .await
                .expect_err("CreateFCMPositionTransfer 500");
            assert!(
                error.payload.is_some(),
                "CreateFCMPositionTransfer 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetHistoricalCutoff"];
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_cutoff()
                .await
                .expect_err("GetHistoricalCutoff 500");
            assert!(
                error.payload.is_some(),
                "GetHistoricalCutoff 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetMarketCandlesticksHistorical"];
        let parameters: GetMarketCandlesticksHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_historical(&parameters)
                .await
                .expect_err("GetMarketCandlesticksHistorical 400");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksHistorical 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_historical(&parameters)
                .await
                .expect_err("GetMarketCandlesticksHistorical 404");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksHistorical 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_market_candlesticks_historical(&parameters)
                .await
                .expect_err("GetMarketCandlesticksHistorical 500");
            assert!(
                error.payload.is_some(),
                "GetMarketCandlesticksHistorical 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetFillsHistorical"];
        let parameters: GetFillsHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills_historical(&parameters)
                .await
                .expect_err("GetFillsHistorical 400");
            assert!(
                error.payload.is_some(),
                "GetFillsHistorical 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills_historical(&parameters)
                .await
                .expect_err("GetFillsHistorical 401");
            assert!(
                error.payload.is_some(),
                "GetFillsHistorical 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills_historical(&parameters)
                .await
                .expect_err("GetFillsHistorical 404");
            assert!(
                error.payload.is_some(),
                "GetFillsHistorical 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_fills_historical(&parameters)
                .await
                .expect_err("GetFillsHistorical 500");
            assert!(
                error.payload.is_some(),
                "GetFillsHistorical 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetHistoricalOrders"];
        let parameters: GetHistoricalOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_orders(&parameters)
                .await
                .expect_err("GetHistoricalOrders 400");
            assert!(
                error.payload.is_some(),
                "GetHistoricalOrders 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_orders(&parameters)
                .await
                .expect_err("GetHistoricalOrders 401");
            assert!(
                error.payload.is_some(),
                "GetHistoricalOrders 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_orders(&parameters)
                .await
                .expect_err("GetHistoricalOrders 500");
            assert!(
                error.payload.is_some(),
                "GetHistoricalOrders 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetHistoricalPositions"];
        let parameters: GetHistoricalPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_positions(&parameters)
                .await
                .expect_err("GetHistoricalPositions 400");
            assert!(
                error.payload.is_some(),
                "GetHistoricalPositions 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(401, fixture["errors"]["401"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_positions(&parameters)
                .await
                .expect_err("GetHistoricalPositions 401");
            assert!(
                error.payload.is_some(),
                "GetHistoricalPositions 401 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 401);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["401"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_positions(&parameters)
                .await
                .expect_err("GetHistoricalPositions 500");
            assert!(
                error.payload.is_some(),
                "GetHistoricalPositions 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetTradesHistorical"];
        let parameters: GetTradesHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_trades_historical(&parameters)
                .await
                .expect_err("GetTradesHistorical 400");
            assert!(
                error.payload.is_some(),
                "GetTradesHistorical 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_trades_historical(&parameters)
                .await
                .expect_err("GetTradesHistorical 404");
            assert!(
                error.payload.is_some(),
                "GetTradesHistorical 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_trades_historical(&parameters)
                .await
                .expect_err("GetTradesHistorical 500");
            assert!(
                error.payload.is_some(),
                "GetTradesHistorical 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetHistoricalMarkets"];
        let parameters: GetHistoricalMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(400, fixture["errors"]["400"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_markets(&parameters)
                .await
                .expect_err("GetHistoricalMarkets 400");
            assert!(
                error.payload.is_some(),
                "GetHistoricalMarkets 400 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 400);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["400"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_markets(&parameters)
                .await
                .expect_err("GetHistoricalMarkets 500");
            assert!(
                error.payload.is_some(),
                "GetHistoricalMarkets 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
    {
        let fixture = &fixtures["GetHistoricalMarket"];
        let parameters: GetHistoricalMarketParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        {
            let (client, server) = typed_support::server(
                vec![(404, fixture["errors"]["404"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_market(&parameters)
                .await
                .expect_err("GetHistoricalMarket 404");
            assert!(
                error.payload.is_some(),
                "GetHistoricalMarket 404 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 404);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["404"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
        {
            let (client, server) = typed_support::server(
                vec![(500, fixture["errors"]["500"].clone())],
                Some(auth.clone()),
            );
            let error = client
                .get_historical_market(&parameters)
                .await
                .expect_err("GetHistoricalMarket 500");
            assert!(
                error.payload.is_some(),
                "GetHistoricalMarket 500 missing typed error"
            );
            if let kalshi_client::KalshiError::Api(api) = error.source {
                assert_eq!(api.status.as_u16(), 500);
                assert_eq!(api.headers["x-request-id"], "fake-probe");
                assert_eq!(api.body, fixture["errors"]["500"]);
            } else {
                panic!("lost structured ApiError");
            }
            server.join().unwrap();
        }
    }
}
