// Generated wire probes: every named method sends and decodes a schema fixture.
mod typed_support;
use kalshi_client::models::*;
use serde_json::Value;
#[tokio::test]
async fn all_118_named_methods_send_and_decode_official_schema_models() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
    let auth = typed_support::auth();
    {
        let fixture = &fixtures["GetExchangeStatus"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_exchange_status()
            .await
            .expect("GetExchangeStatus response");
        kalshi_client::typed::validate(
            "GetExchangeStatusStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetExchangeStatus roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request("GetExchangeStatus", &requests[0], None, None, false, &auth);
    }
    {
        let fixture = &fixtures["GetSeriesFeeChanges"];
        let parameters: GetSeriesFeeChangesParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetSeriesFeeChanges parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_series_fee_changes(&parameters)
            .await
            .expect("GetSeriesFeeChanges response");
        kalshi_client::typed::validate(
            "GetSeriesFeeChangesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSeriesFeeChanges roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSeriesFeeChanges",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetExchangeSchedule"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_exchange_schedule()
            .await
            .expect("GetExchangeSchedule response");
        kalshi_client::typed::validate(
            "GetExchangeScheduleStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetExchangeSchedule roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetExchangeSchedule",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetUserDataTimestamp"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_user_data_timestamp()
            .await
            .expect("GetUserDataTimestamp response");
        kalshi_client::typed::validate(
            "GetUserDataTimestampStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetUserDataTimestamp roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetUserDataTimestamp",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarketCandlesticks"];
        let parameters: GetMarketCandlesticksParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMarketCandlesticks parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market_candlesticks(&parameters)
            .await
            .expect("GetMarketCandlesticks response");
        kalshi_client::typed::validate(
            "GetMarketCandlesticksStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarketCandlesticks roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarketCandlesticks",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetTrades"];
        let parameters: GetTradesParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetTrades parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_trades(&parameters)
            .await
            .expect("GetTrades response");
        kalshi_client::typed::validate(
            "GetTradesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetTrades roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTrades",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarketOrderbook"];
        let parameters: GetMarketOrderbookParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMarketOrderbook parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market_orderbook(&parameters)
            .await
            .expect("GetMarketOrderbook response");
        kalshi_client::typed::validate(
            "GetMarketOrderbookStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarketOrderbook roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarketOrderbook",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarketOrderbooks"];
        let parameters: GetMarketOrderbooksParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMarketOrderbooks parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market_orderbooks(&parameters)
            .await
            .expect("GetMarketOrderbooks response");
        kalshi_client::typed::validate(
            "GetMarketOrderbooksStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarketOrderbooks roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarketOrderbooks",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSeries"];
        let parameters: GetSeriesParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetSeries parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_series(&parameters)
            .await
            .expect("GetSeries response");
        kalshi_client::typed::validate(
            "GetSeriesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSeries roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSeries",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSeriesList"];
        let parameters: GetSeriesListParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetSeriesList parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_series_list(&parameters)
            .await
            .expect("GetSeriesList response");
        kalshi_client::typed::validate(
            "GetSeriesListStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSeriesList roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSeriesList",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarkets"];
        let parameters: GetMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetMarkets parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_markets(&parameters)
            .await
            .expect("GetMarkets response");
        kalshi_client::typed::validate(
            "GetMarketsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarkets roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarkets",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarket"];
        let parameters: GetMarketParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetMarket parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market(&parameters)
            .await
            .expect("GetMarket response");
        kalshi_client::typed::validate(
            "GetMarketStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarket roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarket",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["BatchGetMarketCandlesticks"];
        let parameters: BatchGetMarketCandlesticksParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("BatchGetMarketCandlesticks parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .batch_get_market_candlesticks(&parameters)
            .await
            .expect("BatchGetMarketCandlesticks response");
        kalshi_client::typed::validate(
            "BatchGetMarketCandlesticksStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("BatchGetMarketCandlesticks roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "BatchGetMarketCandlesticks",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarketCandlesticksByEvent"];
        let parameters: GetMarketCandlesticksByEventParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMarketCandlesticksByEvent parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market_candlesticks_by_event(&parameters)
            .await
            .expect("GetMarketCandlesticksByEvent response");
        kalshi_client::typed::validate(
            "GetMarketCandlesticksByEventStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarketCandlesticksByEvent roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarketCandlesticksByEvent",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEvents"];
        let parameters: GetEventsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetEvents parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_events(&parameters)
            .await
            .expect("GetEvents response");
        kalshi_client::typed::validate(
            "GetEventsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetEvents roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEvents",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMultivariateEvents"];
        let parameters: GetMultivariateEventsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMultivariateEvents parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_multivariate_events(&parameters)
            .await
            .expect("GetMultivariateEvents response");
        kalshi_client::typed::validate(
            "GetMultivariateEventsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMultivariateEvents roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMultivariateEvents",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEventFeeChanges"];
        let parameters: GetEventFeeChangesParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetEventFeeChanges parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_event_fee_changes(&parameters)
            .await
            .expect("GetEventFeeChanges response");
        kalshi_client::typed::validate(
            "GetEventFeeChangesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetEventFeeChanges roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEventFeeChanges",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEvent"];
        let parameters: GetEventParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetEvent parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_event(&parameters)
            .await
            .expect("GetEvent response");
        kalshi_client::typed::validate("GetEventStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetEvent roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEvent",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEventMetadata"];
        let parameters: GetEventMetadataParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetEventMetadata parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_event_metadata(&parameters)
            .await
            .expect("GetEventMetadata response");
        kalshi_client::typed::validate(
            "GetEventMetadataStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetEventMetadata roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEventMetadata",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEventForecastPercentilesHistory"];
        let parameters: GetEventForecastPercentilesHistoryParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetEventForecastPercentilesHistory parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_event_forecast_percentiles_history(&parameters)
            .await
            .expect("GetEventForecastPercentilesHistory response");
        kalshi_client::typed::validate(
            "GetEventForecastPercentilesHistoryStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetEventForecastPercentilesHistory roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEventForecastPercentilesHistory",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrders"];
        let parameters: GetOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetOrders parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_orders(&parameters)
            .await
            .expect("GetOrders response");
        kalshi_client::typed::validate(
            "GetOrdersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetOrders roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrders",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrder"];
        let parameters: GetOrderParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetOrder parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_order(&parameters)
            .await
            .expect("GetOrder response");
        kalshi_client::typed::validate("GetOrderStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetOrder roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrder",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrderQueuePositions"];
        let parameters: GetOrderQueuePositionsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetOrderQueuePositions parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_order_queue_positions(&parameters)
            .await
            .expect("GetOrderQueuePositions response");
        kalshi_client::typed::validate(
            "GetOrderQueuePositionsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetOrderQueuePositions roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrderQueuePositions",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrderQueuePosition"];
        let parameters: GetOrderQueuePositionParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetOrderQueuePosition parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_order_queue_position(&parameters)
            .await
            .expect("GetOrderQueuePosition response");
        kalshi_client::typed::validate(
            "GetOrderQueuePositionStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetOrderQueuePosition roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrderQueuePosition",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateOrderV2"];
        let body: CreateOrderV2Body =
            serde_json::from_value(fixture["body"].clone()).expect("CreateOrderV2 body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_order_v2(&body)
            .await
            .expect("CreateOrderV2 response");
        kalshi_client::typed::validate(
            "CreateOrderV2Status201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateOrderV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateOrderV2",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CancelAllOrders"];
        let parameters: CancelAllOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("CancelAllOrders parameters");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .cancel_all_orders(&parameters)
            .await
            .expect("CancelAllOrders response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CancelAllOrders",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["BatchCreateOrdersV2"];
        let body: BatchCreateOrdersV2Body =
            serde_json::from_value(fixture["body"].clone()).expect("BatchCreateOrdersV2 body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .batch_create_orders_v2(&body)
            .await
            .expect("BatchCreateOrdersV2 response");
        kalshi_client::typed::validate(
            "BatchCreateOrdersV2Status201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("BatchCreateOrdersV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "BatchCreateOrdersV2",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["BatchCancelOrdersV2"];
        let body: BatchCancelOrdersV2Body =
            serde_json::from_value(fixture["body"].clone()).expect("BatchCancelOrdersV2 body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .batch_cancel_orders_v2(&body)
            .await
            .expect("BatchCancelOrdersV2 response");
        kalshi_client::typed::validate(
            "BatchCancelOrdersV2Status200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("BatchCancelOrdersV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "BatchCancelOrdersV2",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CancelOrderV2"];
        let parameters: CancelOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("CancelOrderV2 parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .cancel_order_v2(&parameters)
            .await
            .expect("CancelOrderV2 response");
        kalshi_client::typed::validate(
            "CancelOrderV2Status200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CancelOrderV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CancelOrderV2",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["AmendOrderV2"];
        let parameters: AmendOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("AmendOrderV2 parameters");
        let body: AmendOrderV2Body =
            serde_json::from_value(fixture["body"].clone()).expect("AmendOrderV2 body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .amend_order_v2(&parameters, &body)
            .await
            .expect("AmendOrderV2 response");
        kalshi_client::typed::validate(
            "AmendOrderV2Status200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("AmendOrderV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "AmendOrderV2",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DecreaseOrderV2"];
        let parameters: DecreaseOrderV2Parameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("DecreaseOrderV2 parameters");
        let body: DecreaseOrderV2Body =
            serde_json::from_value(fixture["body"].clone()).expect("DecreaseOrderV2 body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .decrease_order_v2(&parameters, &body)
            .await
            .expect("DecreaseOrderV2 response");
        kalshi_client::typed::validate(
            "DecreaseOrderV2Status200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("DecreaseOrderV2 roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DecreaseOrderV2",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrderGroups"];
        let parameters: GetOrderGroupsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetOrderGroups parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_order_groups(&parameters)
            .await
            .expect("GetOrderGroups response");
        kalshi_client::typed::validate(
            "GetOrderGroupsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetOrderGroups roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrderGroups",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateOrderGroup"];
        let body: CreateOrderGroupBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateOrderGroup body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_order_group(&body)
            .await
            .expect("CreateOrderGroup response");
        kalshi_client::typed::validate(
            "CreateOrderGroupStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateOrderGroup roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateOrderGroup",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrderGroup"];
        let parameters: GetOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetOrderGroup parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_order_group(&parameters)
            .await
            .expect("GetOrderGroup response");
        kalshi_client::typed::validate(
            "GetOrderGroupStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetOrderGroup roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrderGroup",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteOrderGroup"];
        let parameters: DeleteOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("DeleteOrderGroup parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .delete_order_group(&parameters)
            .await
            .expect("DeleteOrderGroup response");
        kalshi_client::typed::validate(
            "DeleteOrderGroupStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("DeleteOrderGroup roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteOrderGroup",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ResetOrderGroup"];
        let parameters: ResetOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("ResetOrderGroup parameters");
        let body: ResetOrderGroupBody =
            serde_json::from_value(fixture["body"].clone()).expect("ResetOrderGroup body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .reset_order_group(&parameters, Some(&body))
            .await
            .expect("ResetOrderGroup response");
        kalshi_client::typed::validate(
            "ResetOrderGroupStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("ResetOrderGroup roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "ResetOrderGroup",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["TriggerOrderGroup"];
        let parameters: TriggerOrderGroupParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("TriggerOrderGroup parameters");
        let body: TriggerOrderGroupBody =
            serde_json::from_value(fixture["body"].clone()).expect("TriggerOrderGroup body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .trigger_order_group(&parameters, Some(&body))
            .await
            .expect("TriggerOrderGroup response");
        kalshi_client::typed::validate(
            "TriggerOrderGroupStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("TriggerOrderGroup roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "TriggerOrderGroup",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["UpdateOrderGroupLimit"];
        let parameters: UpdateOrderGroupLimitParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("UpdateOrderGroupLimit parameters");
        let body: UpdateOrderGroupLimitBody =
            serde_json::from_value(fixture["body"].clone()).expect("UpdateOrderGroupLimit body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .update_order_group_limit(&parameters, &body)
            .await
            .expect("UpdateOrderGroupLimit response");
        kalshi_client::typed::validate(
            "UpdateOrderGroupLimitStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("UpdateOrderGroupLimit roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "UpdateOrderGroupLimit",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetBalance"];
        let parameters: GetBalanceParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetBalance parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_balance(&parameters)
            .await
            .expect("GetBalance response");
        kalshi_client::typed::validate(
            "GetBalanceStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetBalance roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetBalance",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["IntraExchangeInstanceTransfer"];
        let body: IntraExchangeInstanceTransferBody =
            serde_json::from_value(fixture["body"].clone())
                .expect("IntraExchangeInstanceTransfer body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .intra_exchange_instance_transfer(&body)
            .await
            .expect("IntraExchangeInstanceTransfer response");
        kalshi_client::typed::validate(
            "IntraExchangeInstanceTransferStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("IntraExchangeInstanceTransfer roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "IntraExchangeInstanceTransfer",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetIntraExchangeInstanceTransfers"];
        let parameters: GetIntraExchangeInstanceTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetIntraExchangeInstanceTransfers parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_intra_exchange_instance_transfers(&parameters)
            .await
            .expect("GetIntraExchangeInstanceTransfers response");
        kalshi_client::typed::validate(
            "GetIntraExchangeInstanceTransfersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetIntraExchangeInstanceTransfers roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetIntraExchangeInstanceTransfers",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetIntraExchangeInstanceTransfer"];
        let parameters: GetIntraExchangeInstanceTransferParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetIntraExchangeInstanceTransfer parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_intra_exchange_instance_transfer(&parameters)
            .await
            .expect("GetIntraExchangeInstanceTransfer response");
        kalshi_client::typed::validate(
            "GetIntraExchangeInstanceTransferStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetIntraExchangeInstanceTransfer roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetIntraExchangeInstanceTransfer",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateSubaccount"];
        let body: CreateSubaccountBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateSubaccount body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_subaccount(Some(&body))
            .await
            .expect("CreateSubaccount response");
        kalshi_client::typed::validate(
            "CreateSubaccountStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateSubaccount roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateSubaccount",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ApplySubaccountTransfer"];
        let body: ApplySubaccountTransferBody =
            serde_json::from_value(fixture["body"].clone()).expect("ApplySubaccountTransfer body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .apply_subaccount_transfer(&body)
            .await
            .expect("ApplySubaccountTransfer response");
        kalshi_client::typed::validate(
            "ApplySubaccountTransferStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("ApplySubaccountTransfer roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "ApplySubaccountTransfer",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSubaccountBalances"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_subaccount_balances()
            .await
            .expect("GetSubaccountBalances response");
        kalshi_client::typed::validate(
            "GetSubaccountBalancesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSubaccountBalances roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSubaccountBalances",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSubaccountTransfers"];
        let parameters: GetSubaccountTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetSubaccountTransfers parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_subaccount_transfers(&parameters)
            .await
            .expect("GetSubaccountTransfers response");
        kalshi_client::typed::validate(
            "GetSubaccountTransfersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSubaccountTransfers roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSubaccountTransfers",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSubaccountNetting"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_subaccount_netting()
            .await
            .expect("GetSubaccountNetting response");
        kalshi_client::typed::validate(
            "GetSubaccountNettingStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSubaccountNetting roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSubaccountNetting",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["UpdateSubaccountNetting"];
        let body: UpdateSubaccountNettingBody =
            serde_json::from_value(fixture["body"].clone()).expect("UpdateSubaccountNetting body");
        let (client, server) = typed_support::server(vec![(200, Value::Null)], Some(auth.clone()));
        let result = client
            .update_subaccount_netting(&body)
            .await
            .expect("UpdateSubaccountNetting response");
        assert!(result.body.is_null());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "UpdateSubaccountNetting",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetPositions"];
        let parameters: GetPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetPositions parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_positions(&parameters)
            .await
            .expect("GetPositions response");
        kalshi_client::typed::validate(
            "GetPositionsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetPositions roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetPositions",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSettlements"];
        let parameters: GetSettlementsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetSettlements parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_settlements(&parameters)
            .await
            .expect("GetSettlements response");
        kalshi_client::typed::validate(
            "GetSettlementsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetSettlements roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSettlements",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetDeposits"];
        let parameters: GetDepositsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetDeposits parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_deposits(&parameters)
            .await
            .expect("GetDeposits response");
        kalshi_client::typed::validate(
            "GetDepositsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetDeposits roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetDeposits",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetWithdrawals"];
        let parameters: GetWithdrawalsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetWithdrawals parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_withdrawals(&parameters)
            .await
            .expect("GetWithdrawals response");
        kalshi_client::typed::validate(
            "GetWithdrawalsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetWithdrawals roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetWithdrawals",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetPortfolioRestingOrderTotalValue"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_portfolio_resting_order_total_value()
            .await
            .expect("GetPortfolioRestingOrderTotalValue response");
        kalshi_client::typed::validate(
            "GetPortfolioRestingOrderTotalValueStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetPortfolioRestingOrderTotalValue roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetPortfolioRestingOrderTotalValue",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFills"];
        let parameters: GetFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetFills parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fills(&parameters)
            .await
            .expect("GetFills response");
        kalshi_client::typed::validate("GetFillsStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetFills roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFills",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetCommunicationsID"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_communications_id()
            .await
            .expect("GetCommunicationsID response");
        kalshi_client::typed::validate(
            "GetCommunicationsIdStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetCommunicationsID roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request("GetCommunicationsID", &requests[0], None, None, true, &auth);
    }
    {
        let fixture = &fixtures["GetBlockTradeProposals"];
        let parameters: GetBlockTradeProposalsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetBlockTradeProposals parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_block_trade_proposals(&parameters)
            .await
            .expect("GetBlockTradeProposals response");
        kalshi_client::typed::validate(
            "GetBlockTradeProposalsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetBlockTradeProposals roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetBlockTradeProposals",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ProposeBlockTrade"];
        let body: ProposeBlockTradeBody =
            serde_json::from_value(fixture["body"].clone()).expect("ProposeBlockTrade body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .propose_block_trade(&body)
            .await
            .expect("ProposeBlockTrade response");
        kalshi_client::typed::validate(
            "ProposeBlockTradeStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("ProposeBlockTrade roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "ProposeBlockTrade",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["AcceptBlockTradeProposal"];
        let parameters: AcceptBlockTradeProposalParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("AcceptBlockTradeProposal parameters");
        let body: AcceptBlockTradeProposalBody =
            serde_json::from_value(fixture["body"].clone()).expect("AcceptBlockTradeProposal body");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .accept_block_trade_proposal(&parameters, Some(&body))
            .await
            .expect("AcceptBlockTradeProposal response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "AcceptBlockTradeProposal",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetRFQs"];
        let parameters: GetRfQsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetRFQs parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_rf_qs(&parameters)
            .await
            .expect("GetRFQs response");
        kalshi_client::typed::validate("GetRfQsStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetRFQs roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetRFQs",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateRFQ"];
        let body: CreateRfqBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateRFQ body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client.create_rfq(&body).await.expect("CreateRFQ response");
        kalshi_client::typed::validate(
            "CreateRfqStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateRFQ roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateRFQ",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetRFQ"];
        let parameters: GetRfqParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetRFQ parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client.get_rfq(&parameters).await.expect("GetRFQ response");
        kalshi_client::typed::validate("GetRfqStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetRFQ roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetRFQ",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteRFQ"];
        let parameters: DeleteRfqParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("DeleteRFQ parameters");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .delete_rfq(&parameters)
            .await
            .expect("DeleteRFQ response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteRFQ",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetRFQQuote"];
        let parameters: GetRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetRFQQuote parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_rfq_quote(&parameters)
            .await
            .expect("GetRFQQuote response");
        kalshi_client::typed::validate(
            "GetRfqQuoteStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetRFQQuote roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetRFQQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteRFQQuote"];
        let parameters: DeleteRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("DeleteRFQQuote parameters");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .delete_rfq_quote(&parameters)
            .await
            .expect("DeleteRFQQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteRFQQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["AcceptRFQQuote"];
        let parameters: AcceptRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("AcceptRFQQuote parameters");
        let body: AcceptRfqQuoteBody =
            serde_json::from_value(fixture["body"].clone()).expect("AcceptRFQQuote body");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .accept_rfq_quote(&parameters, &body)
            .await
            .expect("AcceptRFQQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "AcceptRFQQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ConfirmRFQQuote"];
        let parameters: ConfirmRfqQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("ConfirmRFQQuote parameters");
        let body: ConfirmRfqQuoteBody =
            serde_json::from_value(fixture["body"].clone()).expect("ConfirmRFQQuote body");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .confirm_rfq_quote(&parameters, Some(&body))
            .await
            .expect("ConfirmRFQQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "ConfirmRFQQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetQuotes"];
        let parameters: GetQuotesParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetQuotes parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_quotes(&parameters)
            .await
            .expect("GetQuotes response");
        kalshi_client::typed::validate(
            "GetQuotesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetQuotes roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetQuotes",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateQuote"];
        let body: CreateQuoteBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateQuote body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_quote(&body)
            .await
            .expect("CreateQuote response");
        kalshi_client::typed::validate(
            "CreateQuoteStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateQuote roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateQuote",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetQuote"];
        let parameters: GetQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetQuote parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_quote(&parameters)
            .await
            .expect("GetQuote response");
        kalshi_client::typed::validate("GetQuoteStatus200", &serde_json::to_value(result).unwrap())
            .expect("GetQuote roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteQuote"];
        let parameters: DeleteQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("DeleteQuote parameters");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .delete_quote(&parameters)
            .await
            .expect("DeleteQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["AcceptQuote"];
        let parameters: AcceptQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("AcceptQuote parameters");
        let body: AcceptQuoteBody =
            serde_json::from_value(fixture["body"].clone()).expect("AcceptQuote body");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .accept_quote(&parameters, &body)
            .await
            .expect("AcceptQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "AcceptQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ConfirmQuote"];
        let parameters: ConfirmQuoteParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("ConfirmQuote parameters");
        let body: ConfirmQuoteBody =
            serde_json::from_value(fixture["body"].clone()).expect("ConfirmQuote body");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .confirm_quote(&parameters, Some(&body))
            .await
            .expect("ConfirmQuote response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "ConfirmQuote",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetApiKeys"];
        let parameters: GetApiKeysParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetApiKeys parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_api_keys(&parameters)
            .await
            .expect("GetApiKeys response");
        kalshi_client::typed::validate(
            "GetApiKeysStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetApiKeys roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetApiKeys",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateApiKey"];
        let body: CreateApiKeyBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateApiKey body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_api_key(&body)
            .await
            .expect("CreateApiKey response");
        kalshi_client::typed::validate(
            "CreateApiKeyStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateApiKey roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateApiKey",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GenerateApiKey"];
        let body: GenerateApiKeyBody =
            serde_json::from_value(fixture["body"].clone()).expect("GenerateApiKey body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .generate_api_key(&body)
            .await
            .expect("GenerateApiKey response");
        kalshi_client::typed::validate(
            "GenerateApiKeyStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GenerateApiKey roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GenerateApiKey",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteApiKey"];
        let parameters: DeleteApiKeyParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("DeleteApiKey parameters");
        let (client, server) = typed_support::server(vec![(204, Value::Null)], Some(auth.clone()));
        client
            .delete_api_key(&parameters)
            .await
            .expect("DeleteApiKey response");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteApiKey",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetAccountApiLimits"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_account_api_limits()
            .await
            .expect("GetAccountApiLimits response");
        kalshi_client::typed::validate(
            "GetAccountApiLimitsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetAccountApiLimits roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request("GetAccountApiLimits", &requests[0], None, None, true, &auth);
    }
    {
        let _fixture = &fixtures["UpgradeAccountApiUsageLevel"];
        let (client, server) = typed_support::server(vec![(201, Value::Null)], Some(auth.clone()));
        let result = client
            .upgrade_account_api_usage_level()
            .await
            .expect("UpgradeAccountApiUsageLevel response");
        assert!(result.body.is_null());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "UpgradeAccountApiUsageLevel",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetAccountApiUsageLevelVolumeProgress"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_account_api_usage_level_volume_progress()
            .await
            .expect("GetAccountApiUsageLevelVolumeProgress response");
        kalshi_client::typed::validate(
            "GetAccountApiUsageLevelVolumeProgressStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetAccountApiUsageLevelVolumeProgress roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetAccountApiUsageLevelVolumeProgress",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetAccountEndpointCosts"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_account_endpoint_costs()
            .await
            .expect("GetAccountEndpointCosts response");
        kalshi_client::typed::validate(
            "GetAccountEndpointCostsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetAccountEndpointCosts roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetAccountEndpointCosts",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetTagsForSeriesCategories"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_tags_for_series_categories()
            .await
            .expect("GetTagsForSeriesCategories response");
        kalshi_client::typed::validate(
            "GetTagsForSeriesCategoriesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetTagsForSeriesCategories roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTagsForSeriesCategories",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFiltersForSports"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_filters_for_sports()
            .await
            .expect("GetFiltersForSports response");
        kalshi_client::typed::validate(
            "GetFiltersForSportsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFiltersForSports roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFiltersForSports",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetLiveDataByMilestone"];
        let parameters: GetLiveDataByMilestoneParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetLiveDataByMilestone parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_live_data_by_milestone(&parameters)
            .await
            .expect("GetLiveDataByMilestone response");
        kalshi_client::typed::validate(
            "GetLiveDataByMilestoneStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetLiveDataByMilestone roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetLiveDataByMilestone",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetLiveData"];
        let parameters: GetLiveDataParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetLiveData parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_live_data(&parameters)
            .await
            .expect("GetLiveData response");
        kalshi_client::typed::validate(
            "GetLiveDataStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetLiveData roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetLiveData",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetLiveDatas"];
        let parameters: GetLiveDatasParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetLiveDatas parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_live_datas(&parameters)
            .await
            .expect("GetLiveDatas response");
        kalshi_client::typed::validate(
            "GetLiveDatasStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetLiveDatas roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetLiveDatas",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetGameStats"];
        let parameters: GetGameStatsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetGameStats parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_game_stats(&parameters)
            .await
            .expect("GetGameStats response");
        kalshi_client::typed::validate(
            "GetGameStatsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetGameStats roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetGameStats",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEventLiveData"];
        let parameters: GetEventLiveDataParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetEventLiveData parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_event_live_data(&parameters)
            .await
            .expect("GetEventLiveData response");
        kalshi_client::typed::validate(
            "GetEventLiveDataStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetEventLiveData roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEventLiveData",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetWeatherIndex"];
        let parameters: GetWeatherIndexParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetWeatherIndex parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_weather_index(&parameters)
            .await
            .expect("GetWeatherIndex response");
        kalshi_client::typed::validate(
            "GetWeatherIndexStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetWeatherIndex roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetWeatherIndex",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetWeatherIndexCalibrations"];
        let parameters: GetWeatherIndexCalibrationsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetWeatherIndexCalibrations parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_weather_index_calibrations(&parameters)
            .await
            .expect("GetWeatherIndexCalibrations response");
        kalshi_client::typed::validate(
            "GetWeatherIndexCalibrationsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetWeatherIndexCalibrations roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetWeatherIndexCalibrations",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetStructuredTargets"];
        let parameters: GetStructuredTargetsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetStructuredTargets parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_structured_targets(&parameters)
            .await
            .expect("GetStructuredTargets response");
        kalshi_client::typed::validate(
            "GetStructuredTargetsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetStructuredTargets roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetStructuredTargets",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetStructuredTarget"];
        let parameters: GetStructuredTargetParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetStructuredTarget parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_structured_target(&parameters)
            .await
            .expect("GetStructuredTarget response");
        kalshi_client::typed::validate(
            "GetStructuredTargetStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetStructuredTarget roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetStructuredTarget",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMilestone"];
        let parameters: GetMilestoneParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetMilestone parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_milestone(&parameters)
            .await
            .expect("GetMilestone response");
        kalshi_client::typed::validate(
            "GetMilestoneStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMilestone roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMilestone",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMilestones"];
        let parameters: GetMilestonesParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMilestones parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_milestones(&parameters)
            .await
            .expect("GetMilestones response");
        kalshi_client::typed::validate(
            "GetMilestonesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMilestones roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMilestones",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMultivariateEventCollection"];
        let parameters: GetMultivariateEventCollectionParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMultivariateEventCollection parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_multivariate_event_collection(&parameters)
            .await
            .expect("GetMultivariateEventCollection response");
        kalshi_client::typed::validate(
            "GetMultivariateEventCollectionStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMultivariateEventCollection roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMultivariateEventCollection",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateMarketInMultivariateEventCollection"];
        let parameters: CreateMarketInMultivariateEventCollectionParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("CreateMarketInMultivariateEventCollection parameters");
        let body: CreateMarketInMultivariateEventCollectionBody =
            serde_json::from_value(fixture["body"].clone())
                .expect("CreateMarketInMultivariateEventCollection body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_market_in_multivariate_event_collection(&parameters, &body)
            .await
            .expect("CreateMarketInMultivariateEventCollection response");
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateMarketInMultivariateEventCollection roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateMarketInMultivariateEventCollection",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMultivariateEventCollections"];
        let parameters: GetMultivariateEventCollectionsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMultivariateEventCollections parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_multivariate_event_collections(&parameters)
            .await
            .expect("GetMultivariateEventCollections response");
        kalshi_client::typed::validate(
            "GetMultivariateEventCollectionsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMultivariateEventCollections roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMultivariateEventCollections",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetIncentivePrograms"];
        let parameters: GetIncentiveProgramsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetIncentivePrograms parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_incentive_programs(&parameters)
            .await
            .expect("GetIncentivePrograms response");
        kalshi_client::typed::validate(
            "GetIncentiveProgramsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetIncentivePrograms roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetIncentivePrograms",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMFills"];
        let parameters: GetFcmFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetFCMFills parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fcm_fills(&parameters)
            .await
            .expect("GetFCMFills response");
        kalshi_client::typed::validate(
            "GetFcmFillsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFCMFills roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMFills",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMOrders"];
        let parameters: GetFcmOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).expect("GetFCMOrders parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fcm_orders(&parameters)
            .await
            .expect("GetFCMOrders response");
        kalshi_client::typed::validate(
            "GetFcmOrdersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFCMOrders roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMOrders",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["ListFCMSubtraders"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .list_fcm_subtraders()
            .await
            .expect("ListFCMSubtraders response");
        kalshi_client::typed::validate(
            "ListFcmSubtradersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("ListFCMSubtraders roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request("ListFCMSubtraders", &requests[0], None, None, true, &auth);
    }
    {
        let fixture = &fixtures["CreateFCMSubtrader"];
        let body: CreateFcmSubtraderBody =
            serde_json::from_value(fixture["body"].clone()).expect("CreateFCMSubtrader body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_fcm_subtrader(&body)
            .await
            .expect("CreateFCMSubtrader response");
        kalshi_client::typed::validate(
            "CreateFcmSubtraderStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateFCMSubtrader roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateFCMSubtrader",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMEventContractDailyCap"];
        let parameters: GetFcmEventContractDailyCapParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetFCMEventContractDailyCap parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fcm_event_contract_daily_cap(&parameters)
            .await
            .expect("GetFCMEventContractDailyCap response");
        kalshi_client::typed::validate(
            "GetFcmEventContractDailyCapStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFCMEventContractDailyCap roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMEventContractDailyCap",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["UpdateFCMEventContractDailyCap"];
        let body: UpdateFcmEventContractDailyCapBody =
            serde_json::from_value(fixture["body"].clone())
                .expect("UpdateFCMEventContractDailyCap body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .update_fcm_event_contract_daily_cap(&body)
            .await
            .expect("UpdateFCMEventContractDailyCap response");
        kalshi_client::typed::validate(
            "UpdateFcmEventContractDailyCapStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("UpdateFCMEventContractDailyCap roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "UpdateFCMEventContractDailyCap",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["DeleteFCMEventContractDailyCap"];
        let parameters: DeleteFcmEventContractDailyCapParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("DeleteFCMEventContractDailyCap parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .delete_fcm_event_contract_daily_cap(&parameters)
            .await
            .expect("DeleteFCMEventContractDailyCap response");
        kalshi_client::typed::validate(
            "DeleteFcmEventContractDailyCapStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("DeleteFCMEventContractDailyCap roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "DeleteFCMEventContractDailyCap",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMSubtraderBlockedCategories"];
        let parameters: GetFcmSubtraderBlockedCategoriesParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetFCMSubtraderBlockedCategories parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fcm_subtrader_blocked_categories(&parameters)
            .await
            .expect("GetFCMSubtraderBlockedCategories response");
        kalshi_client::typed::validate(
            "GetFcmSubtraderBlockedCategoriesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFCMSubtraderBlockedCategories roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMSubtraderBlockedCategories",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["UpdateFCMSubtraderBlockedCategories"];
        let body: UpdateFcmSubtraderBlockedCategoriesBody =
            serde_json::from_value(fixture["body"].clone())
                .expect("UpdateFCMSubtraderBlockedCategories body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .update_fcm_subtrader_blocked_categories(&body)
            .await
            .expect("UpdateFCMSubtraderBlockedCategories response");
        kalshi_client::typed::validate(
            "UpdateFcmSubtraderBlockedCategoriesStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("UpdateFCMSubtraderBlockedCategories roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "UpdateFCMSubtraderBlockedCategories",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetTargetBalanceAllocation"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_target_balance_allocation()
            .await
            .expect("GetTargetBalanceAllocation response");
        kalshi_client::typed::validate(
            "GetTargetBalanceAllocationStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetTargetBalanceAllocation roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTargetBalanceAllocation",
            &requests[0],
            None,
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["SetTargetBalanceAllocation"];
        let body: SetTargetBalanceAllocationBody = serde_json::from_value(fixture["body"].clone())
            .expect("SetTargetBalanceAllocation body");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .set_target_balance_allocation(&body)
            .await
            .expect("SetTargetBalanceAllocation response");
        kalshi_client::typed::validate(
            "SetTargetBalanceAllocationStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("SetTargetBalanceAllocation roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "SetTargetBalanceAllocation",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMPositions"];
        let parameters: GetFcmPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetFCMPositions parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fcm_positions(&parameters)
            .await
            .expect("GetFCMPositions response");
        kalshi_client::typed::validate(
            "GetFcmPositionsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFCMPositions roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMPositions",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["CreateFCMPositionTransfer"];
        let body: CreateFcmPositionTransferBody = serde_json::from_value(fixture["body"].clone())
            .expect("CreateFCMPositionTransfer body");
        let (client, server) =
            typed_support::server(vec![(201, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .create_fcm_position_transfer(&body)
            .await
            .expect("CreateFCMPositionTransfer response");
        kalshi_client::typed::validate(
            "CreateFcmPositionTransferStatus201",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("CreateFCMPositionTransfer roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "CreateFCMPositionTransfer",
            &requests[0],
            None,
            Some(&serde_json::to_value(&body).unwrap()),
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalCutoff"];
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_historical_cutoff()
            .await
            .expect("GetHistoricalCutoff response");
        kalshi_client::typed::validate(
            "GetHistoricalCutoffStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetHistoricalCutoff roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalCutoff",
            &requests[0],
            None,
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarketCandlesticksHistorical"];
        let parameters: GetMarketCandlesticksHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetMarketCandlesticksHistorical parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_market_candlesticks_historical(&parameters)
            .await
            .expect("GetMarketCandlesticksHistorical response");
        kalshi_client::typed::validate(
            "GetMarketCandlesticksHistoricalStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetMarketCandlesticksHistorical roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarketCandlesticksHistorical",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFillsHistorical"];
        let parameters: GetFillsHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetFillsHistorical parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_fills_historical(&parameters)
            .await
            .expect("GetFillsHistorical response");
        kalshi_client::typed::validate(
            "GetFillsHistoricalStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetFillsHistorical roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFillsHistorical",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalOrders"];
        let parameters: GetHistoricalOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetHistoricalOrders parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_historical_orders(&parameters)
            .await
            .expect("GetHistoricalOrders response");
        kalshi_client::typed::validate(
            "GetHistoricalOrdersStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetHistoricalOrders roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalOrders",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalPositions"];
        let parameters: GetHistoricalPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetHistoricalPositions parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_historical_positions(&parameters)
            .await
            .expect("GetHistoricalPositions response");
        kalshi_client::typed::validate(
            "GetHistoricalPositionsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetHistoricalPositions roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalPositions",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetTradesHistorical"];
        let parameters: GetTradesHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetTradesHistorical parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_trades_historical(&parameters)
            .await
            .expect("GetTradesHistorical response");
        kalshi_client::typed::validate(
            "GetTradesHistoricalStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetTradesHistorical roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTradesHistorical",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalMarkets"];
        let parameters: GetHistoricalMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetHistoricalMarkets parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_historical_markets(&parameters)
            .await
            .expect("GetHistoricalMarkets response");
        kalshi_client::typed::validate(
            "GetHistoricalMarketsStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetHistoricalMarkets roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalMarkets",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalMarket"];
        let parameters: GetHistoricalMarketParameters =
            serde_json::from_value(fixture["parameters"].clone())
                .expect("GetHistoricalMarket parameters");
        let (client, server) =
            typed_support::server(vec![(200, fixture["response"].clone())], Some(auth.clone()));
        let result = client
            .get_historical_market(&parameters)
            .await
            .expect("GetHistoricalMarket response");
        kalshi_client::typed::validate(
            "GetHistoricalMarketStatus200",
            &serde_json::to_value(result).unwrap(),
        )
        .expect("GetHistoricalMarket roundtrip");
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalMarket",
            &requests[0],
            Some(&serde_json::to_value(&parameters).unwrap()),
            None,
            false,
            &auth,
        );
    }
}
