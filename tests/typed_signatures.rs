// Generated signature probes: every operation is type checked.
#![allow(dead_code)]
use kalshi_client::{
    KalshiClient,
    generated::{models::*, typed::*},
    typed::{TypedError, UnspecifiedResponse},
};
async fn probe_get_exchange_status(client: &KalshiClient) {
    let _: Result<GetExchangeStatusStatus200, TypedError<GetExchangeStatusError>> =
        client.get_exchange_status().await;
}
async fn probe_get_series_fee_changes(
    client: &KalshiClient,
    parameters: &GetSeriesFeeChangesParameters,
) {
    let _: Result<GetSeriesFeeChangesStatus200, TypedError<GetSeriesFeeChangesError>> =
        client.get_series_fee_changes(parameters).await;
}
async fn probe_get_exchange_schedule(client: &KalshiClient) {
    let _: Result<GetExchangeScheduleStatus200, TypedError<GetExchangeScheduleError>> =
        client.get_exchange_schedule().await;
}
async fn probe_get_user_data_timestamp(client: &KalshiClient) {
    let _: Result<GetUserDataTimestampStatus200, TypedError<GetUserDataTimestampError>> =
        client.get_user_data_timestamp().await;
}
async fn probe_get_market_candlesticks(
    client: &KalshiClient,
    parameters: &GetMarketCandlesticksParameters,
) {
    let _: Result<GetMarketCandlesticksStatus200, TypedError<GetMarketCandlesticksError>> =
        client.get_market_candlesticks(parameters).await;
}
async fn probe_get_trades(client: &KalshiClient, parameters: &GetTradesParameters) {
    let _: Result<GetTradesStatus200, TypedError<GetTradesError>> =
        client.get_trades(parameters).await;
}
async fn probe_get_market_orderbook(
    client: &KalshiClient,
    parameters: &GetMarketOrderbookParameters,
) {
    let _: Result<GetMarketOrderbookStatus200, TypedError<GetMarketOrderbookError>> =
        client.get_market_orderbook(parameters).await;
}
async fn probe_get_market_orderbooks(
    client: &KalshiClient,
    parameters: &GetMarketOrderbooksParameters,
) {
    let _: Result<GetMarketOrderbooksStatus200, TypedError<GetMarketOrderbooksError>> =
        client.get_market_orderbooks(parameters).await;
}
async fn probe_get_series(client: &KalshiClient, parameters: &GetSeriesParameters) {
    let _: Result<GetSeriesStatus200, TypedError<GetSeriesError>> =
        client.get_series(parameters).await;
}
async fn probe_get_series_list(client: &KalshiClient, parameters: &GetSeriesListParameters) {
    let _: Result<GetSeriesListStatus200, TypedError<GetSeriesListError>> =
        client.get_series_list(parameters).await;
}
async fn probe_get_markets(client: &KalshiClient, parameters: &GetMarketsParameters) {
    let _: Result<GetMarketsStatus200, TypedError<GetMarketsError>> =
        client.get_markets(parameters).await;
}
async fn probe_get_market(client: &KalshiClient, parameters: &GetMarketParameters) {
    let _: Result<GetMarketStatus200, TypedError<GetMarketError>> =
        client.get_market(parameters).await;
}
async fn probe_batch_get_market_candlesticks(
    client: &KalshiClient,
    parameters: &BatchGetMarketCandlesticksParameters,
) {
    let _: Result<
        BatchGetMarketCandlesticksStatus200,
        TypedError<BatchGetMarketCandlesticksError>,
    > = client.batch_get_market_candlesticks(parameters).await;
}
async fn probe_get_market_candlesticks_by_event(
    client: &KalshiClient,
    parameters: &GetMarketCandlesticksByEventParameters,
) {
    let _: Result<
        GetMarketCandlesticksByEventStatus200,
        TypedError<GetMarketCandlesticksByEventError>,
    > = client.get_market_candlesticks_by_event(parameters).await;
}
async fn probe_get_events(client: &KalshiClient, parameters: &GetEventsParameters) {
    let _: Result<GetEventsStatus200, TypedError<GetEventsError>> =
        client.get_events(parameters).await;
}
async fn probe_get_multivariate_events(
    client: &KalshiClient,
    parameters: &GetMultivariateEventsParameters,
) {
    let _: Result<GetMultivariateEventsStatus200, TypedError<GetMultivariateEventsError>> =
        client.get_multivariate_events(parameters).await;
}
async fn probe_get_event_fee_changes(
    client: &KalshiClient,
    parameters: &GetEventFeeChangesParameters,
) {
    let _: Result<GetEventFeeChangesStatus200, TypedError<GetEventFeeChangesError>> =
        client.get_event_fee_changes(parameters).await;
}
async fn probe_get_event(client: &KalshiClient, parameters: &GetEventParameters) {
    let _: Result<GetEventStatus200, TypedError<GetEventError>> =
        client.get_event(parameters).await;
}
async fn probe_get_event_metadata(client: &KalshiClient, parameters: &GetEventMetadataParameters) {
    let _: Result<GetEventMetadataStatus200, TypedError<GetEventMetadataError>> =
        client.get_event_metadata(parameters).await;
}
async fn probe_get_event_forecast_percentiles_history(
    client: &KalshiClient,
    parameters: &GetEventForecastPercentilesHistoryParameters,
) {
    let _: Result<
        GetEventForecastPercentilesHistoryStatus200,
        TypedError<GetEventForecastPercentilesHistoryError>,
    > = client
        .get_event_forecast_percentiles_history(parameters)
        .await;
}
async fn probe_get_orders(client: &KalshiClient, parameters: &GetOrdersParameters) {
    let _: Result<GetOrdersStatus200, TypedError<GetOrdersError>> =
        client.get_orders(parameters).await;
}
async fn probe_get_order(client: &KalshiClient, parameters: &GetOrderParameters) {
    let _: Result<GetOrderStatus200, TypedError<GetOrderError>> =
        client.get_order(parameters).await;
}
async fn probe_get_order_queue_positions(
    client: &KalshiClient,
    parameters: &GetOrderQueuePositionsParameters,
) {
    let _: Result<GetOrderQueuePositionsStatus200, TypedError<GetOrderQueuePositionsError>> =
        client.get_order_queue_positions(parameters).await;
}
async fn probe_get_order_queue_position(
    client: &KalshiClient,
    parameters: &GetOrderQueuePositionParameters,
) {
    let _: Result<GetOrderQueuePositionStatus200, TypedError<GetOrderQueuePositionError>> =
        client.get_order_queue_position(parameters).await;
}
async fn probe_create_order_v2(client: &KalshiClient, body: &CreateOrderV2Body) {
    let _: Result<CreateOrderV2Status201, TypedError<CreateOrderV2Error>> =
        client.create_order_v2(body).await;
}
async fn probe_cancel_all_orders(client: &KalshiClient, parameters: &CancelAllOrdersParameters) {
    let _: Result<(), TypedError<CancelAllOrdersError>> =
        client.cancel_all_orders(parameters).await;
}
async fn probe_batch_create_orders_v2(client: &KalshiClient, body: &BatchCreateOrdersV2Body) {
    let _: Result<BatchCreateOrdersV2Status201, TypedError<BatchCreateOrdersV2Error>> =
        client.batch_create_orders_v2(body).await;
}
async fn probe_batch_cancel_orders_v2(client: &KalshiClient, body: &BatchCancelOrdersV2Body) {
    let _: Result<BatchCancelOrdersV2Status200, TypedError<BatchCancelOrdersV2Error>> =
        client.batch_cancel_orders_v2(body).await;
}
async fn probe_cancel_order_v2(client: &KalshiClient, parameters: &CancelOrderV2Parameters) {
    let _: Result<CancelOrderV2Status200, TypedError<CancelOrderV2Error>> =
        client.cancel_order_v2(parameters).await;
}
async fn probe_amend_order_v2(
    client: &KalshiClient,
    parameters: &AmendOrderV2Parameters,
    body: &AmendOrderV2Body,
) {
    let _: Result<AmendOrderV2Status200, TypedError<AmendOrderV2Error>> =
        client.amend_order_v2(parameters, body).await;
}
async fn probe_decrease_order_v2(
    client: &KalshiClient,
    parameters: &DecreaseOrderV2Parameters,
    body: &DecreaseOrderV2Body,
) {
    let _: Result<DecreaseOrderV2Status200, TypedError<DecreaseOrderV2Error>> =
        client.decrease_order_v2(parameters, body).await;
}
async fn probe_get_order_groups(client: &KalshiClient, parameters: &GetOrderGroupsParameters) {
    let _: Result<GetOrderGroupsStatus200, TypedError<GetOrderGroupsError>> =
        client.get_order_groups(parameters).await;
}
async fn probe_create_order_group(client: &KalshiClient, body: &CreateOrderGroupBody) {
    let _: Result<CreateOrderGroupStatus201, TypedError<CreateOrderGroupError>> =
        client.create_order_group(body).await;
}
async fn probe_get_order_group(client: &KalshiClient, parameters: &GetOrderGroupParameters) {
    let _: Result<GetOrderGroupStatus200, TypedError<GetOrderGroupError>> =
        client.get_order_group(parameters).await;
}
async fn probe_delete_order_group(client: &KalshiClient, parameters: &DeleteOrderGroupParameters) {
    let _: Result<DeleteOrderGroupStatus200, TypedError<DeleteOrderGroupError>> =
        client.delete_order_group(parameters).await;
}
async fn probe_reset_order_group(
    client: &KalshiClient,
    parameters: &ResetOrderGroupParameters,
    body: Option<&ResetOrderGroupBody>,
) {
    let _: Result<ResetOrderGroupStatus200, TypedError<ResetOrderGroupError>> =
        client.reset_order_group(parameters, body).await;
}
async fn probe_trigger_order_group(
    client: &KalshiClient,
    parameters: &TriggerOrderGroupParameters,
    body: Option<&TriggerOrderGroupBody>,
) {
    let _: Result<TriggerOrderGroupStatus200, TypedError<TriggerOrderGroupError>> =
        client.trigger_order_group(parameters, body).await;
}
async fn probe_update_order_group_limit(
    client: &KalshiClient,
    parameters: &UpdateOrderGroupLimitParameters,
    body: &UpdateOrderGroupLimitBody,
) {
    let _: Result<UpdateOrderGroupLimitStatus200, TypedError<UpdateOrderGroupLimitError>> =
        client.update_order_group_limit(parameters, body).await;
}
async fn probe_get_balance(client: &KalshiClient, parameters: &GetBalanceParameters) {
    let _: Result<GetBalanceStatus200, TypedError<GetBalanceError>> =
        client.get_balance(parameters).await;
}
async fn probe_intra_exchange_instance_transfer(
    client: &KalshiClient,
    body: &IntraExchangeInstanceTransferBody,
) {
    let _: Result<
        IntraExchangeInstanceTransferStatus200,
        TypedError<IntraExchangeInstanceTransferError>,
    > = client.intra_exchange_instance_transfer(body).await;
}
async fn probe_get_intra_exchange_instance_transfers(
    client: &KalshiClient,
    parameters: &GetIntraExchangeInstanceTransfersParameters,
) {
    let _: Result<
        GetIntraExchangeInstanceTransfersStatus200,
        TypedError<GetIntraExchangeInstanceTransfersError>,
    > = client
        .get_intra_exchange_instance_transfers(parameters)
        .await;
}
async fn probe_get_intra_exchange_instance_transfer(
    client: &KalshiClient,
    parameters: &GetIntraExchangeInstanceTransferParameters,
) {
    let _: Result<
        GetIntraExchangeInstanceTransferStatus200,
        TypedError<GetIntraExchangeInstanceTransferError>,
    > = client
        .get_intra_exchange_instance_transfer(parameters)
        .await;
}
async fn probe_create_subaccount(client: &KalshiClient, body: Option<&CreateSubaccountBody>) {
    let _: Result<CreateSubaccountStatus201, TypedError<CreateSubaccountError>> =
        client.create_subaccount(body).await;
}
async fn probe_apply_subaccount_transfer(
    client: &KalshiClient,
    body: &ApplySubaccountTransferBody,
) {
    let _: Result<ApplySubaccountTransferStatus200, TypedError<ApplySubaccountTransferError>> =
        client.apply_subaccount_transfer(body).await;
}
async fn probe_get_subaccount_balances(client: &KalshiClient) {
    let _: Result<GetSubaccountBalancesStatus200, TypedError<GetSubaccountBalancesError>> =
        client.get_subaccount_balances().await;
}
async fn probe_get_subaccount_transfers(
    client: &KalshiClient,
    parameters: &GetSubaccountTransfersParameters,
) {
    let _: Result<GetSubaccountTransfersStatus200, TypedError<GetSubaccountTransfersError>> =
        client.get_subaccount_transfers(parameters).await;
}
async fn probe_get_subaccount_netting(client: &KalshiClient) {
    let _: Result<GetSubaccountNettingStatus200, TypedError<GetSubaccountNettingError>> =
        client.get_subaccount_netting().await;
}
async fn probe_update_subaccount_netting(
    client: &KalshiClient,
    body: &UpdateSubaccountNettingBody,
) {
    let _: Result<UnspecifiedResponse, TypedError<UpdateSubaccountNettingError>> =
        client.update_subaccount_netting(body).await;
}
async fn probe_get_positions(client: &KalshiClient, parameters: &GetPositionsParameters) {
    let _: Result<GetPositionsStatus200, TypedError<GetPositionsError>> =
        client.get_positions(parameters).await;
}
async fn probe_get_settlements(client: &KalshiClient, parameters: &GetSettlementsParameters) {
    let _: Result<GetSettlementsStatus200, TypedError<GetSettlementsError>> =
        client.get_settlements(parameters).await;
}
async fn probe_get_deposits(client: &KalshiClient, parameters: &GetDepositsParameters) {
    let _: Result<GetDepositsStatus200, TypedError<GetDepositsError>> =
        client.get_deposits(parameters).await;
}
async fn probe_get_withdrawals(client: &KalshiClient, parameters: &GetWithdrawalsParameters) {
    let _: Result<GetWithdrawalsStatus200, TypedError<GetWithdrawalsError>> =
        client.get_withdrawals(parameters).await;
}
async fn probe_get_portfolio_resting_order_total_value(client: &KalshiClient) {
    let _: Result<
        GetPortfolioRestingOrderTotalValueStatus200,
        TypedError<GetPortfolioRestingOrderTotalValueError>,
    > = client.get_portfolio_resting_order_total_value().await;
}
async fn probe_get_fills(client: &KalshiClient, parameters: &GetFillsParameters) {
    let _: Result<GetFillsStatus200, TypedError<GetFillsError>> =
        client.get_fills(parameters).await;
}
async fn probe_get_communications_id(client: &KalshiClient) {
    let _: Result<GetCommunicationsIdStatus200, TypedError<GetCommunicationsIdError>> =
        client.get_communications_id().await;
}
async fn probe_get_block_trade_proposals(
    client: &KalshiClient,
    parameters: &GetBlockTradeProposalsParameters,
) {
    let _: Result<GetBlockTradeProposalsStatus200, TypedError<GetBlockTradeProposalsError>> =
        client.get_block_trade_proposals(parameters).await;
}
async fn probe_propose_block_trade(client: &KalshiClient, body: &ProposeBlockTradeBody) {
    let _: Result<ProposeBlockTradeStatus201, TypedError<ProposeBlockTradeError>> =
        client.propose_block_trade(body).await;
}
async fn probe_accept_block_trade_proposal(
    client: &KalshiClient,
    parameters: &AcceptBlockTradeProposalParameters,
    body: Option<&AcceptBlockTradeProposalBody>,
) {
    let _: Result<(), TypedError<AcceptBlockTradeProposalError>> =
        client.accept_block_trade_proposal(parameters, body).await;
}
async fn probe_get_rf_qs(client: &KalshiClient, parameters: &GetRfQsParameters) {
    let _: Result<GetRfQsStatus200, TypedError<GetRfQsError>> = client.get_rf_qs(parameters).await;
}
async fn probe_create_rfq(client: &KalshiClient, body: &CreateRfqBody) {
    let _: Result<CreateRfqStatus201, TypedError<CreateRfqError>> = client.create_rfq(body).await;
}
async fn probe_get_rfq(client: &KalshiClient, parameters: &GetRfqParameters) {
    let _: Result<GetRfqStatus200, TypedError<GetRfqError>> = client.get_rfq(parameters).await;
}
async fn probe_delete_rfq(client: &KalshiClient, parameters: &DeleteRfqParameters) {
    let _: Result<(), TypedError<DeleteRfqError>> = client.delete_rfq(parameters).await;
}
async fn probe_get_rfq_quote(client: &KalshiClient, parameters: &GetRfqQuoteParameters) {
    let _: Result<GetRfqQuoteStatus200, TypedError<GetRfqQuoteError>> =
        client.get_rfq_quote(parameters).await;
}
async fn probe_delete_rfq_quote(client: &KalshiClient, parameters: &DeleteRfqQuoteParameters) {
    let _: Result<(), TypedError<DeleteRfqQuoteError>> = client.delete_rfq_quote(parameters).await;
}
async fn probe_accept_rfq_quote(
    client: &KalshiClient,
    parameters: &AcceptRfqQuoteParameters,
    body: &AcceptRfqQuoteBody,
) {
    let _: Result<(), TypedError<AcceptRfqQuoteError>> =
        client.accept_rfq_quote(parameters, body).await;
}
async fn probe_confirm_rfq_quote(
    client: &KalshiClient,
    parameters: &ConfirmRfqQuoteParameters,
    body: Option<&ConfirmRfqQuoteBody>,
) {
    let _: Result<(), TypedError<ConfirmRfqQuoteError>> =
        client.confirm_rfq_quote(parameters, body).await;
}
async fn probe_get_quotes(client: &KalshiClient, parameters: &GetQuotesParameters) {
    let _: Result<GetQuotesStatus200, TypedError<GetQuotesError>> =
        client.get_quotes(parameters).await;
}
async fn probe_create_quote(client: &KalshiClient, body: &CreateQuoteBody) {
    let _: Result<CreateQuoteStatus201, TypedError<CreateQuoteError>> =
        client.create_quote(body).await;
}
async fn probe_get_quote(client: &KalshiClient, parameters: &GetQuoteParameters) {
    let _: Result<GetQuoteStatus200, TypedError<GetQuoteError>> =
        client.get_quote(parameters).await;
}
async fn probe_delete_quote(client: &KalshiClient, parameters: &DeleteQuoteParameters) {
    let _: Result<(), TypedError<DeleteQuoteError>> = client.delete_quote(parameters).await;
}
async fn probe_accept_quote(
    client: &KalshiClient,
    parameters: &AcceptQuoteParameters,
    body: &AcceptQuoteBody,
) {
    let _: Result<(), TypedError<AcceptQuoteError>> = client.accept_quote(parameters, body).await;
}
async fn probe_confirm_quote(
    client: &KalshiClient,
    parameters: &ConfirmQuoteParameters,
    body: Option<&ConfirmQuoteBody>,
) {
    let _: Result<(), TypedError<ConfirmQuoteError>> = client.confirm_quote(parameters, body).await;
}
async fn probe_get_api_keys(client: &KalshiClient, parameters: &GetApiKeysParameters) {
    let _: Result<GetApiKeysStatus200, TypedError<GetApiKeysError>> =
        client.get_api_keys(parameters).await;
}
async fn probe_create_api_key(client: &KalshiClient, body: &CreateApiKeyBody) {
    let _: Result<CreateApiKeyStatus201, TypedError<CreateApiKeyError>> =
        client.create_api_key(body).await;
}
async fn probe_generate_api_key(client: &KalshiClient, body: &GenerateApiKeyBody) {
    let _: Result<GenerateApiKeyStatus201, TypedError<GenerateApiKeyError>> =
        client.generate_api_key(body).await;
}
async fn probe_delete_api_key(client: &KalshiClient, parameters: &DeleteApiKeyParameters) {
    let _: Result<(), TypedError<DeleteApiKeyError>> = client.delete_api_key(parameters).await;
}
async fn probe_get_account_api_limits(client: &KalshiClient) {
    let _: Result<GetAccountApiLimitsStatus200, TypedError<GetAccountApiLimitsError>> =
        client.get_account_api_limits().await;
}
async fn probe_upgrade_account_api_usage_level(client: &KalshiClient) {
    let _: Result<UnspecifiedResponse, TypedError<UpgradeAccountApiUsageLevelError>> =
        client.upgrade_account_api_usage_level().await;
}
async fn probe_get_account_api_usage_level_volume_progress(client: &KalshiClient) {
    let _: Result<
        GetAccountApiUsageLevelVolumeProgressStatus200,
        TypedError<GetAccountApiUsageLevelVolumeProgressError>,
    > = client.get_account_api_usage_level_volume_progress().await;
}
async fn probe_get_account_endpoint_costs(client: &KalshiClient) {
    let _: Result<GetAccountEndpointCostsStatus200, TypedError<GetAccountEndpointCostsError>> =
        client.get_account_endpoint_costs().await;
}
async fn probe_get_tags_for_series_categories(client: &KalshiClient) {
    let _: Result<
        GetTagsForSeriesCategoriesStatus200,
        TypedError<GetTagsForSeriesCategoriesError>,
    > = client.get_tags_for_series_categories().await;
}
async fn probe_get_filters_for_sports(client: &KalshiClient) {
    let _: Result<GetFiltersForSportsStatus200, TypedError<GetFiltersForSportsError>> =
        client.get_filters_for_sports().await;
}
async fn probe_get_live_data_by_milestone(
    client: &KalshiClient,
    parameters: &GetLiveDataByMilestoneParameters,
) {
    let _: Result<GetLiveDataByMilestoneStatus200, TypedError<GetLiveDataByMilestoneError>> =
        client.get_live_data_by_milestone(parameters).await;
}
async fn probe_get_live_data(client: &KalshiClient, parameters: &GetLiveDataParameters) {
    let _: Result<GetLiveDataStatus200, TypedError<GetLiveDataError>> =
        client.get_live_data(parameters).await;
}
async fn probe_get_live_datas(client: &KalshiClient, parameters: &GetLiveDatasParameters) {
    let _: Result<GetLiveDatasStatus200, TypedError<GetLiveDatasError>> =
        client.get_live_datas(parameters).await;
}
async fn probe_get_game_stats(client: &KalshiClient, parameters: &GetGameStatsParameters) {
    let _: Result<GetGameStatsStatus200, TypedError<GetGameStatsError>> =
        client.get_game_stats(parameters).await;
}
async fn probe_get_event_live_data(client: &KalshiClient, parameters: &GetEventLiveDataParameters) {
    let _: Result<GetEventLiveDataStatus200, TypedError<GetEventLiveDataError>> =
        client.get_event_live_data(parameters).await;
}
async fn probe_get_weather_index(client: &KalshiClient, parameters: &GetWeatherIndexParameters) {
    let _: Result<GetWeatherIndexStatus200, TypedError<GetWeatherIndexError>> =
        client.get_weather_index(parameters).await;
}
async fn probe_get_weather_index_calibrations(
    client: &KalshiClient,
    parameters: &GetWeatherIndexCalibrationsParameters,
) {
    let _: Result<
        GetWeatherIndexCalibrationsStatus200,
        TypedError<GetWeatherIndexCalibrationsError>,
    > = client.get_weather_index_calibrations(parameters).await;
}
async fn probe_get_structured_targets(
    client: &KalshiClient,
    parameters: &GetStructuredTargetsParameters,
) {
    let _: Result<GetStructuredTargetsStatus200, TypedError<GetStructuredTargetsError>> =
        client.get_structured_targets(parameters).await;
}
async fn probe_get_structured_target(
    client: &KalshiClient,
    parameters: &GetStructuredTargetParameters,
) {
    let _: Result<GetStructuredTargetStatus200, TypedError<GetStructuredTargetError>> =
        client.get_structured_target(parameters).await;
}
async fn probe_get_milestone(client: &KalshiClient, parameters: &GetMilestoneParameters) {
    let _: Result<GetMilestoneStatus200, TypedError<GetMilestoneError>> =
        client.get_milestone(parameters).await;
}
async fn probe_get_milestones(client: &KalshiClient, parameters: &GetMilestonesParameters) {
    let _: Result<GetMilestonesStatus200, TypedError<GetMilestonesError>> =
        client.get_milestones(parameters).await;
}
async fn probe_get_multivariate_event_collection(
    client: &KalshiClient,
    parameters: &GetMultivariateEventCollectionParameters,
) {
    let _: Result<
        GetMultivariateEventCollectionStatus200,
        TypedError<GetMultivariateEventCollectionError>,
    > = client.get_multivariate_event_collection(parameters).await;
}
async fn probe_create_market_in_multivariate_event_collection(
    client: &KalshiClient,
    parameters: &CreateMarketInMultivariateEventCollectionParameters,
    body: &CreateMarketInMultivariateEventCollectionBody,
) {
    let _: Result<
        CreateMarketInMultivariateEventCollectionStatus200,
        TypedError<CreateMarketInMultivariateEventCollectionError>,
    > = client
        .create_market_in_multivariate_event_collection(parameters, body)
        .await;
}
async fn probe_get_multivariate_event_collections(
    client: &KalshiClient,
    parameters: &GetMultivariateEventCollectionsParameters,
) {
    let _: Result<
        GetMultivariateEventCollectionsStatus200,
        TypedError<GetMultivariateEventCollectionsError>,
    > = client.get_multivariate_event_collections(parameters).await;
}
async fn probe_get_incentive_programs(
    client: &KalshiClient,
    parameters: &GetIncentiveProgramsParameters,
) {
    let _: Result<GetIncentiveProgramsStatus200, TypedError<GetIncentiveProgramsError>> =
        client.get_incentive_programs(parameters).await;
}
async fn probe_get_fcm_fills(client: &KalshiClient, parameters: &GetFcmFillsParameters) {
    let _: Result<GetFcmFillsStatus200, TypedError<GetFcmFillsError>> =
        client.get_fcm_fills(parameters).await;
}
async fn probe_get_fcm_orders(client: &KalshiClient, parameters: &GetFcmOrdersParameters) {
    let _: Result<GetFcmOrdersStatus200, TypedError<GetFcmOrdersError>> =
        client.get_fcm_orders(parameters).await;
}
async fn probe_list_fcm_subtraders(client: &KalshiClient) {
    let _: Result<ListFcmSubtradersStatus200, TypedError<ListFcmSubtradersError>> =
        client.list_fcm_subtraders().await;
}
async fn probe_create_fcm_subtrader(client: &KalshiClient, body: &CreateFcmSubtraderBody) {
    let _: Result<CreateFcmSubtraderStatus201, TypedError<CreateFcmSubtraderError>> =
        client.create_fcm_subtrader(body).await;
}
async fn probe_get_fcm_event_contract_daily_cap(
    client: &KalshiClient,
    parameters: &GetFcmEventContractDailyCapParameters,
) {
    let _: Result<
        GetFcmEventContractDailyCapStatus200,
        TypedError<GetFcmEventContractDailyCapError>,
    > = client.get_fcm_event_contract_daily_cap(parameters).await;
}
async fn probe_update_fcm_event_contract_daily_cap(
    client: &KalshiClient,
    body: &UpdateFcmEventContractDailyCapBody,
) {
    let _: Result<
        UpdateFcmEventContractDailyCapStatus200,
        TypedError<UpdateFcmEventContractDailyCapError>,
    > = client.update_fcm_event_contract_daily_cap(body).await;
}
async fn probe_delete_fcm_event_contract_daily_cap(
    client: &KalshiClient,
    parameters: &DeleteFcmEventContractDailyCapParameters,
) {
    let _: Result<
        DeleteFcmEventContractDailyCapStatus200,
        TypedError<DeleteFcmEventContractDailyCapError>,
    > = client.delete_fcm_event_contract_daily_cap(parameters).await;
}
async fn probe_get_fcm_subtrader_blocked_categories(
    client: &KalshiClient,
    parameters: &GetFcmSubtraderBlockedCategoriesParameters,
) {
    let _: Result<
        GetFcmSubtraderBlockedCategoriesStatus200,
        TypedError<GetFcmSubtraderBlockedCategoriesError>,
    > = client
        .get_fcm_subtrader_blocked_categories(parameters)
        .await;
}
async fn probe_update_fcm_subtrader_blocked_categories(
    client: &KalshiClient,
    body: &UpdateFcmSubtraderBlockedCategoriesBody,
) {
    let _: Result<
        UpdateFcmSubtraderBlockedCategoriesStatus200,
        TypedError<UpdateFcmSubtraderBlockedCategoriesError>,
    > = client.update_fcm_subtrader_blocked_categories(body).await;
}
async fn probe_get_target_balance_allocation(client: &KalshiClient) {
    let _: Result<
        GetTargetBalanceAllocationStatus200,
        TypedError<GetTargetBalanceAllocationError>,
    > = client.get_target_balance_allocation().await;
}
async fn probe_set_target_balance_allocation(
    client: &KalshiClient,
    body: &SetTargetBalanceAllocationBody,
) {
    let _: Result<
        SetTargetBalanceAllocationStatus200,
        TypedError<SetTargetBalanceAllocationError>,
    > = client.set_target_balance_allocation(body).await;
}
async fn probe_get_fcm_positions(client: &KalshiClient, parameters: &GetFcmPositionsParameters) {
    let _: Result<GetFcmPositionsStatus200, TypedError<GetFcmPositionsError>> =
        client.get_fcm_positions(parameters).await;
}
async fn probe_create_fcm_position_transfer(
    client: &KalshiClient,
    body: &CreateFcmPositionTransferBody,
) {
    let _: Result<CreateFcmPositionTransferStatus201, TypedError<CreateFcmPositionTransferError>> =
        client.create_fcm_position_transfer(body).await;
}
async fn probe_get_historical_cutoff(client: &KalshiClient) {
    let _: Result<GetHistoricalCutoffStatus200, TypedError<GetHistoricalCutoffError>> =
        client.get_historical_cutoff().await;
}
async fn probe_get_market_candlesticks_historical(
    client: &KalshiClient,
    parameters: &GetMarketCandlesticksHistoricalParameters,
) {
    let _: Result<
        GetMarketCandlesticksHistoricalStatus200,
        TypedError<GetMarketCandlesticksHistoricalError>,
    > = client.get_market_candlesticks_historical(parameters).await;
}
async fn probe_get_fills_historical(
    client: &KalshiClient,
    parameters: &GetFillsHistoricalParameters,
) {
    let _: Result<GetFillsHistoricalStatus200, TypedError<GetFillsHistoricalError>> =
        client.get_fills_historical(parameters).await;
}
async fn probe_get_historical_orders(
    client: &KalshiClient,
    parameters: &GetHistoricalOrdersParameters,
) {
    let _: Result<GetHistoricalOrdersStatus200, TypedError<GetHistoricalOrdersError>> =
        client.get_historical_orders(parameters).await;
}
async fn probe_get_historical_positions(
    client: &KalshiClient,
    parameters: &GetHistoricalPositionsParameters,
) {
    let _: Result<GetHistoricalPositionsStatus200, TypedError<GetHistoricalPositionsError>> =
        client.get_historical_positions(parameters).await;
}
async fn probe_get_trades_historical(
    client: &KalshiClient,
    parameters: &GetTradesHistoricalParameters,
) {
    let _: Result<GetTradesHistoricalStatus200, TypedError<GetTradesHistoricalError>> =
        client.get_trades_historical(parameters).await;
}
async fn probe_get_historical_markets(
    client: &KalshiClient,
    parameters: &GetHistoricalMarketsParameters,
) {
    let _: Result<GetHistoricalMarketsStatus200, TypedError<GetHistoricalMarketsError>> =
        client.get_historical_markets(parameters).await;
}
async fn probe_get_historical_market(
    client: &KalshiClient,
    parameters: &GetHistoricalMarketParameters,
) {
    let _: Result<GetHistoricalMarketStatus200, TypedError<GetHistoricalMarketError>> =
        client.get_historical_market(parameters).await;
}
