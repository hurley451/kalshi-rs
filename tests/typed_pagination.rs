// Generated cursor wire probes cover every paginated operation.
mod typed_support;
use kalshi_client::models::*;
use serde_json::{Value, json};
#[tokio::test]
async fn all_28_paginators_advance_keep_filters_and_terminate() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-fixtures.json")).unwrap();
    let auth = typed_support::auth();
    {
        let fixture = &fixtures["GetTrades"];
        let parameters: GetTradesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_trades_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetTrades first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetTrades last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTrades",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetTrades",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMarkets"];
        let parameters: GetMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_markets_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMarkets first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMarkets last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMarkets",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetMarkets",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEvents"];
        let parameters: GetEventsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_events_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetEvents first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetEvents last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEvents",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetEvents",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMultivariateEvents"];
        let parameters: GetMultivariateEventsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_multivariate_events_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMultivariateEvents first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMultivariateEvents last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMultivariateEvents",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetMultivariateEvents",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetEventFeeChanges"];
        let parameters: GetEventFeeChangesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_event_fee_changes_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetEventFeeChanges first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetEventFeeChanges last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetEventFeeChanges",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetEventFeeChanges",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetOrders"];
        let parameters: GetOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_orders_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetOrders first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetOrders last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetOrders",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetOrders",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetIntraExchangeInstanceTransfers"];
        let parameters: GetIntraExchangeInstanceTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_intra_exchange_instance_transfers_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetIntraExchangeInstanceTransfers first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetIntraExchangeInstanceTransfers last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetIntraExchangeInstanceTransfers",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetIntraExchangeInstanceTransfers",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSubaccountTransfers"];
        let parameters: GetSubaccountTransfersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_subaccount_transfers_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetSubaccountTransfers first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetSubaccountTransfers last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSubaccountTransfers",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetSubaccountTransfers",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetPositions"];
        let parameters: GetPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_positions_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetPositions first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetPositions last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetPositions",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetPositions",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetSettlements"];
        let parameters: GetSettlementsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_settlements_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetSettlements first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetSettlements last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetSettlements",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetSettlements",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetDeposits"];
        let parameters: GetDepositsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_deposits_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetDeposits first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetDeposits last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetDeposits",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetDeposits",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetWithdrawals"];
        let parameters: GetWithdrawalsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_withdrawals_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetWithdrawals first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetWithdrawals last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetWithdrawals",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetWithdrawals",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFills"];
        let parameters: GetFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_fills_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFills first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFills last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request("GetFills", &requests[0], Some(&expected), None, true, &auth);
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request("GetFills", &requests[1], Some(&expected), None, true, &auth);
    }
    {
        let fixture = &fixtures["GetBlockTradeProposals"];
        let parameters: GetBlockTradeProposalsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_block_trade_proposals_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetBlockTradeProposals first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetBlockTradeProposals last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetBlockTradeProposals",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetBlockTradeProposals",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetRFQs"];
        let parameters: GetRfQsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_rf_qs_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetRFQs first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetRFQs last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request("GetRFQs", &requests[0], Some(&expected), None, true, &auth);
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request("GetRFQs", &requests[1], Some(&expected), None, true, &auth);
    }
    {
        let fixture = &fixtures["GetQuotes"];
        let parameters: GetQuotesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_quotes_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetQuotes first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetQuotes last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetQuotes",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetQuotes",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetStructuredTargets"];
        let parameters: GetStructuredTargetsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_structured_targets_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetStructuredTargets first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetStructuredTargets last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetStructuredTargets",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetStructuredTargets",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMilestones"];
        let parameters: GetMilestonesParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_milestones_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMilestones first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMilestones last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMilestones",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetMilestones",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetMultivariateEventCollections"];
        let parameters: GetMultivariateEventCollectionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_multivariate_event_collections_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMultivariateEventCollections first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetMultivariateEventCollections last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetMultivariateEventCollections",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetMultivariateEventCollections",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetIncentivePrograms"];
        let parameters: GetIncentiveProgramsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["next_cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("next_cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_incentive_programs_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetIncentivePrograms first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetIncentivePrograms last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetIncentivePrograms",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetIncentivePrograms",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMFills"];
        let parameters: GetFcmFillsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_fcm_fills_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMFills first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMFills last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMFills",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetFCMFills",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMOrders"];
        let parameters: GetFcmOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_fcm_orders_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMOrders first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMOrders last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMOrders",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetFCMOrders",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFCMPositions"];
        let parameters: GetFcmPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_fcm_positions_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMPositions first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFCMPositions last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFCMPositions",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetFCMPositions",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetFillsHistorical"];
        let parameters: GetFillsHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_fills_historical_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFillsHistorical first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetFillsHistorical last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetFillsHistorical",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetFillsHistorical",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalOrders"];
        let parameters: GetHistoricalOrdersParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_historical_orders_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalOrders first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalOrders last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalOrders",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetHistoricalOrders",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalPositions"];
        let parameters: GetHistoricalPositionsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last.as_object_mut().unwrap().remove("cursor");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_historical_positions_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalPositions first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalPositions last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalPositions",
            &requests[0],
            Some(&expected),
            None,
            true,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetHistoricalPositions",
            &requests[1],
            Some(&expected),
            None,
            true,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetTradesHistorical"];
        let parameters: GetTradesHistoricalParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_trades_historical_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetTradesHistorical first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetTradesHistorical last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetTradesHistorical",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetTradesHistorical",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
    {
        let fixture = &fixtures["GetHistoricalMarkets"];
        let parameters: GetHistoricalMarketsParameters =
            serde_json::from_value(fixture["parameters"].clone()).unwrap();
        let mut expected = serde_json::to_value(&parameters).unwrap();
        let mut first = fixture["response"].clone();
        first["cursor"] = json!("next cursor");
        let mut last = fixture["response"].clone();
        last["cursor"] = json!("");
        let (client, server) =
            typed_support::server(vec![(200, first), (200, last)], Some(auth.clone()));
        let mut pager = client.get_historical_markets_pages(parameters);
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalMarkets first page")
                .is_some()
        );
        assert!(
            pager
                .next_page()
                .await
                .expect("GetHistoricalMarkets last page")
                .is_some()
        );
        assert!(pager.next_page().await.unwrap().is_none());
        let requests = server.join().unwrap();
        typed_support::assert_request(
            "GetHistoricalMarkets",
            &requests[0],
            Some(&expected),
            None,
            false,
            &auth,
        );
        expected["cursor"] = json!("next cursor");
        typed_support::assert_request(
            "GetHistoricalMarkets",
            &requests[1],
            Some(&expected),
            None,
            false,
            &auth,
        );
    }
}
