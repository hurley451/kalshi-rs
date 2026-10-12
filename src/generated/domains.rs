// Generated domain facades delegate to the canonical typed methods.
use crate::client::KalshiClient;
use crate::generated::{models::*, typed::*};
use crate::typed::{Pager, TypedError, UnspecifiedResponse};
pub struct ExchangeApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn exchange(&self) -> ExchangeApi<'_> {
        ExchangeApi { client: self }
    }
}
impl<'a> ExchangeApi<'a> {
    pub async fn get_exchange_status(
        &self,
    ) -> Result<GetExchangeStatusStatus200, TypedError<GetExchangeStatusError>> {
        self.client.get_exchange_status().await
    }
    pub async fn get_exchange_schedule(
        &self,
    ) -> Result<GetExchangeScheduleStatus200, TypedError<GetExchangeScheduleError>> {
        self.client.get_exchange_schedule().await
    }
    pub async fn get_user_data_timestamp(
        &self,
    ) -> Result<GetUserDataTimestampStatus200, TypedError<GetUserDataTimestampError>> {
        self.client.get_user_data_timestamp().await
    }
}
pub struct SeriesApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn series(&self) -> SeriesApi<'_> {
        SeriesApi { client: self }
    }
}
impl<'a> SeriesApi<'a> {
    pub async fn get_series_fee_changes(
        &self,
        parameters: &GetSeriesFeeChangesParameters,
    ) -> Result<GetSeriesFeeChangesStatus200, TypedError<GetSeriesFeeChangesError>> {
        self.client.get_series_fee_changes(parameters).await
    }
    pub async fn get_market_candlesticks(
        &self,
        parameters: &GetMarketCandlesticksParameters,
    ) -> Result<GetMarketCandlesticksStatus200, TypedError<GetMarketCandlesticksError>> {
        self.client.get_market_candlesticks(parameters).await
    }
    pub async fn get_series(
        &self,
        parameters: &GetSeriesParameters,
    ) -> Result<GetSeriesStatus200, TypedError<GetSeriesError>> {
        self.client.get_series(parameters).await
    }
    pub async fn get_series_list(
        &self,
        parameters: &GetSeriesListParameters,
    ) -> Result<GetSeriesListStatus200, TypedError<GetSeriesListError>> {
        self.client.get_series_list(parameters).await
    }
    pub async fn get_market_candlesticks_by_event(
        &self,
        parameters: &GetMarketCandlesticksByEventParameters,
    ) -> Result<GetMarketCandlesticksByEventStatus200, TypedError<GetMarketCandlesticksByEventError>>
    {
        self.client
            .get_market_candlesticks_by_event(parameters)
            .await
    }
    pub async fn get_event_forecast_percentiles_history(
        &self,
        parameters: &GetEventForecastPercentilesHistoryParameters,
    ) -> Result<
        GetEventForecastPercentilesHistoryStatus200,
        TypedError<GetEventForecastPercentilesHistoryError>,
    > {
        self.client
            .get_event_forecast_percentiles_history(parameters)
            .await
    }
}
pub struct MarketsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn markets(&self) -> MarketsApi<'_> {
        MarketsApi { client: self }
    }
}
impl<'a> MarketsApi<'a> {
    pub async fn get_trades(
        &self,
        parameters: &GetTradesParameters,
    ) -> Result<GetTradesStatus200, TypedError<GetTradesError>> {
        self.client.get_trades(parameters).await
    }
    pub fn get_trades_pages(
        &self,
        parameters: GetTradesParameters,
    ) -> Pager<'a, GetTradesParameters, GetTradesStatus200, GetTradesError> {
        self.client.get_trades_pages(parameters)
    }
    pub async fn get_market_orderbook(
        &self,
        parameters: &GetMarketOrderbookParameters,
    ) -> Result<GetMarketOrderbookStatus200, TypedError<GetMarketOrderbookError>> {
        self.client.get_market_orderbook(parameters).await
    }
    pub async fn get_market_orderbooks(
        &self,
        parameters: &GetMarketOrderbooksParameters,
    ) -> Result<GetMarketOrderbooksStatus200, TypedError<GetMarketOrderbooksError>> {
        self.client.get_market_orderbooks(parameters).await
    }
    pub async fn get_markets(
        &self,
        parameters: &GetMarketsParameters,
    ) -> Result<GetMarketsStatus200, TypedError<GetMarketsError>> {
        self.client.get_markets(parameters).await
    }
    pub fn get_markets_pages(
        &self,
        parameters: GetMarketsParameters,
    ) -> Pager<'a, GetMarketsParameters, GetMarketsStatus200, GetMarketsError> {
        self.client.get_markets_pages(parameters)
    }
    pub async fn get_market(
        &self,
        parameters: &GetMarketParameters,
    ) -> Result<GetMarketStatus200, TypedError<GetMarketError>> {
        self.client.get_market(parameters).await
    }
    pub async fn batch_get_market_candlesticks(
        &self,
        parameters: &BatchGetMarketCandlesticksParameters,
    ) -> Result<BatchGetMarketCandlesticksStatus200, TypedError<BatchGetMarketCandlesticksError>>
    {
        self.client.batch_get_market_candlesticks(parameters).await
    }
}
pub struct EventsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn events(&self) -> EventsApi<'_> {
        EventsApi { client: self }
    }
}
impl<'a> EventsApi<'a> {
    pub async fn get_events(
        &self,
        parameters: &GetEventsParameters,
    ) -> Result<GetEventsStatus200, TypedError<GetEventsError>> {
        self.client.get_events(parameters).await
    }
    pub fn get_events_pages(
        &self,
        parameters: GetEventsParameters,
    ) -> Pager<'a, GetEventsParameters, GetEventsStatus200, GetEventsError> {
        self.client.get_events_pages(parameters)
    }
    pub async fn get_multivariate_events(
        &self,
        parameters: &GetMultivariateEventsParameters,
    ) -> Result<GetMultivariateEventsStatus200, TypedError<GetMultivariateEventsError>> {
        self.client.get_multivariate_events(parameters).await
    }
    pub fn get_multivariate_events_pages(
        &self,
        parameters: GetMultivariateEventsParameters,
    ) -> Pager<
        'a,
        GetMultivariateEventsParameters,
        GetMultivariateEventsStatus200,
        GetMultivariateEventsError,
    > {
        self.client.get_multivariate_events_pages(parameters)
    }
    pub async fn get_event_fee_changes(
        &self,
        parameters: &GetEventFeeChangesParameters,
    ) -> Result<GetEventFeeChangesStatus200, TypedError<GetEventFeeChangesError>> {
        self.client.get_event_fee_changes(parameters).await
    }
    pub fn get_event_fee_changes_pages(
        &self,
        parameters: GetEventFeeChangesParameters,
    ) -> Pager<'a, GetEventFeeChangesParameters, GetEventFeeChangesStatus200, GetEventFeeChangesError>
    {
        self.client.get_event_fee_changes_pages(parameters)
    }
    pub async fn get_event(
        &self,
        parameters: &GetEventParameters,
    ) -> Result<GetEventStatus200, TypedError<GetEventError>> {
        self.client.get_event(parameters).await
    }
    pub async fn get_event_metadata(
        &self,
        parameters: &GetEventMetadataParameters,
    ) -> Result<GetEventMetadataStatus200, TypedError<GetEventMetadataError>> {
        self.client.get_event_metadata(parameters).await
    }
}
pub struct PortfolioApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn portfolio(&self) -> PortfolioApi<'_> {
        PortfolioApi { client: self }
    }
}
impl<'a> PortfolioApi<'a> {
    pub async fn get_orders(
        &self,
        parameters: &GetOrdersParameters,
    ) -> Result<GetOrdersStatus200, TypedError<GetOrdersError>> {
        self.client.get_orders(parameters).await
    }
    pub fn get_orders_pages(
        &self,
        parameters: GetOrdersParameters,
    ) -> Pager<'a, GetOrdersParameters, GetOrdersStatus200, GetOrdersError> {
        self.client.get_orders_pages(parameters)
    }
    pub async fn get_order(
        &self,
        parameters: &GetOrderParameters,
    ) -> Result<GetOrderStatus200, TypedError<GetOrderError>> {
        self.client.get_order(parameters).await
    }
    pub async fn get_order_queue_positions(
        &self,
        parameters: &GetOrderQueuePositionsParameters,
    ) -> Result<GetOrderQueuePositionsStatus200, TypedError<GetOrderQueuePositionsError>> {
        self.client.get_order_queue_positions(parameters).await
    }
    pub async fn get_order_queue_position(
        &self,
        parameters: &GetOrderQueuePositionParameters,
    ) -> Result<GetOrderQueuePositionStatus200, TypedError<GetOrderQueuePositionError>> {
        self.client.get_order_queue_position(parameters).await
    }
    pub async fn create_order_v2(
        &self,
        body: &CreateOrderV2Body,
    ) -> Result<CreateOrderV2Status201, TypedError<CreateOrderV2Error>> {
        self.client.create_order_v2(body).await
    }
    pub async fn cancel_all_orders(
        &self,
        parameters: &CancelAllOrdersParameters,
    ) -> Result<(), TypedError<CancelAllOrdersError>> {
        self.client.cancel_all_orders(parameters).await
    }
    pub async fn batch_create_orders_v2(
        &self,
        body: &BatchCreateOrdersV2Body,
    ) -> Result<BatchCreateOrdersV2Status201, TypedError<BatchCreateOrdersV2Error>> {
        self.client.batch_create_orders_v2(body).await
    }
    pub async fn batch_cancel_orders_v2(
        &self,
        body: &BatchCancelOrdersV2Body,
    ) -> Result<BatchCancelOrdersV2Status200, TypedError<BatchCancelOrdersV2Error>> {
        self.client.batch_cancel_orders_v2(body).await
    }
    pub async fn cancel_order_v2(
        &self,
        parameters: &CancelOrderV2Parameters,
    ) -> Result<CancelOrderV2Status200, TypedError<CancelOrderV2Error>> {
        self.client.cancel_order_v2(parameters).await
    }
    pub async fn amend_order_v2(
        &self,
        parameters: &AmendOrderV2Parameters,
        body: &AmendOrderV2Body,
    ) -> Result<AmendOrderV2Status200, TypedError<AmendOrderV2Error>> {
        self.client.amend_order_v2(parameters, body).await
    }
    pub async fn decrease_order_v2(
        &self,
        parameters: &DecreaseOrderV2Parameters,
        body: &DecreaseOrderV2Body,
    ) -> Result<DecreaseOrderV2Status200, TypedError<DecreaseOrderV2Error>> {
        self.client.decrease_order_v2(parameters, body).await
    }
    pub async fn get_order_groups(
        &self,
        parameters: &GetOrderGroupsParameters,
    ) -> Result<GetOrderGroupsStatus200, TypedError<GetOrderGroupsError>> {
        self.client.get_order_groups(parameters).await
    }
    pub async fn create_order_group(
        &self,
        body: &CreateOrderGroupBody,
    ) -> Result<CreateOrderGroupStatus201, TypedError<CreateOrderGroupError>> {
        self.client.create_order_group(body).await
    }
    pub async fn get_order_group(
        &self,
        parameters: &GetOrderGroupParameters,
    ) -> Result<GetOrderGroupStatus200, TypedError<GetOrderGroupError>> {
        self.client.get_order_group(parameters).await
    }
    pub async fn delete_order_group(
        &self,
        parameters: &DeleteOrderGroupParameters,
    ) -> Result<DeleteOrderGroupStatus200, TypedError<DeleteOrderGroupError>> {
        self.client.delete_order_group(parameters).await
    }
    pub async fn reset_order_group(
        &self,
        parameters: &ResetOrderGroupParameters,
        body: Option<&ResetOrderGroupBody>,
    ) -> Result<ResetOrderGroupStatus200, TypedError<ResetOrderGroupError>> {
        self.client.reset_order_group(parameters, body).await
    }
    pub async fn trigger_order_group(
        &self,
        parameters: &TriggerOrderGroupParameters,
        body: Option<&TriggerOrderGroupBody>,
    ) -> Result<TriggerOrderGroupStatus200, TypedError<TriggerOrderGroupError>> {
        self.client.trigger_order_group(parameters, body).await
    }
    pub async fn update_order_group_limit(
        &self,
        parameters: &UpdateOrderGroupLimitParameters,
        body: &UpdateOrderGroupLimitBody,
    ) -> Result<UpdateOrderGroupLimitStatus200, TypedError<UpdateOrderGroupLimitError>> {
        self.client.update_order_group_limit(parameters, body).await
    }
    pub async fn get_balance(
        &self,
        parameters: &GetBalanceParameters,
    ) -> Result<GetBalanceStatus200, TypedError<GetBalanceError>> {
        self.client.get_balance(parameters).await
    }
    pub async fn intra_exchange_instance_transfer(
        &self,
        body: &IntraExchangeInstanceTransferBody,
    ) -> Result<
        IntraExchangeInstanceTransferStatus200,
        TypedError<IntraExchangeInstanceTransferError>,
    > {
        self.client.intra_exchange_instance_transfer(body).await
    }
    pub async fn get_intra_exchange_instance_transfers(
        &self,
        parameters: &GetIntraExchangeInstanceTransfersParameters,
    ) -> Result<
        GetIntraExchangeInstanceTransfersStatus200,
        TypedError<GetIntraExchangeInstanceTransfersError>,
    > {
        self.client
            .get_intra_exchange_instance_transfers(parameters)
            .await
    }
    pub fn get_intra_exchange_instance_transfers_pages(
        &self,
        parameters: GetIntraExchangeInstanceTransfersParameters,
    ) -> Pager<
        'a,
        GetIntraExchangeInstanceTransfersParameters,
        GetIntraExchangeInstanceTransfersStatus200,
        GetIntraExchangeInstanceTransfersError,
    > {
        self.client
            .get_intra_exchange_instance_transfers_pages(parameters)
    }
    pub async fn get_intra_exchange_instance_transfer(
        &self,
        parameters: &GetIntraExchangeInstanceTransferParameters,
    ) -> Result<
        GetIntraExchangeInstanceTransferStatus200,
        TypedError<GetIntraExchangeInstanceTransferError>,
    > {
        self.client
            .get_intra_exchange_instance_transfer(parameters)
            .await
    }
    pub async fn create_subaccount(
        &self,
        body: Option<&CreateSubaccountBody>,
    ) -> Result<CreateSubaccountStatus201, TypedError<CreateSubaccountError>> {
        self.client.create_subaccount(body).await
    }
    pub async fn apply_subaccount_transfer(
        &self,
        body: &ApplySubaccountTransferBody,
    ) -> Result<ApplySubaccountTransferStatus200, TypedError<ApplySubaccountTransferError>> {
        self.client.apply_subaccount_transfer(body).await
    }
    pub async fn get_subaccount_balances(
        &self,
    ) -> Result<GetSubaccountBalancesStatus200, TypedError<GetSubaccountBalancesError>> {
        self.client.get_subaccount_balances().await
    }
    pub async fn get_subaccount_transfers(
        &self,
        parameters: &GetSubaccountTransfersParameters,
    ) -> Result<GetSubaccountTransfersStatus200, TypedError<GetSubaccountTransfersError>> {
        self.client.get_subaccount_transfers(parameters).await
    }
    pub fn get_subaccount_transfers_pages(
        &self,
        parameters: GetSubaccountTransfersParameters,
    ) -> Pager<
        'a,
        GetSubaccountTransfersParameters,
        GetSubaccountTransfersStatus200,
        GetSubaccountTransfersError,
    > {
        self.client.get_subaccount_transfers_pages(parameters)
    }
    pub async fn get_subaccount_netting(
        &self,
    ) -> Result<GetSubaccountNettingStatus200, TypedError<GetSubaccountNettingError>> {
        self.client.get_subaccount_netting().await
    }
    pub async fn update_subaccount_netting(
        &self,
        body: &UpdateSubaccountNettingBody,
    ) -> Result<UnspecifiedResponse, TypedError<UpdateSubaccountNettingError>> {
        self.client.update_subaccount_netting(body).await
    }
    pub async fn get_positions(
        &self,
        parameters: &GetPositionsParameters,
    ) -> Result<GetPositionsStatus200, TypedError<GetPositionsError>> {
        self.client.get_positions(parameters).await
    }
    pub fn get_positions_pages(
        &self,
        parameters: GetPositionsParameters,
    ) -> Pager<'a, GetPositionsParameters, GetPositionsStatus200, GetPositionsError> {
        self.client.get_positions_pages(parameters)
    }
    pub async fn get_settlements(
        &self,
        parameters: &GetSettlementsParameters,
    ) -> Result<GetSettlementsStatus200, TypedError<GetSettlementsError>> {
        self.client.get_settlements(parameters).await
    }
    pub fn get_settlements_pages(
        &self,
        parameters: GetSettlementsParameters,
    ) -> Pager<'a, GetSettlementsParameters, GetSettlementsStatus200, GetSettlementsError> {
        self.client.get_settlements_pages(parameters)
    }
    pub async fn get_deposits(
        &self,
        parameters: &GetDepositsParameters,
    ) -> Result<GetDepositsStatus200, TypedError<GetDepositsError>> {
        self.client.get_deposits(parameters).await
    }
    pub fn get_deposits_pages(
        &self,
        parameters: GetDepositsParameters,
    ) -> Pager<'a, GetDepositsParameters, GetDepositsStatus200, GetDepositsError> {
        self.client.get_deposits_pages(parameters)
    }
    pub async fn get_withdrawals(
        &self,
        parameters: &GetWithdrawalsParameters,
    ) -> Result<GetWithdrawalsStatus200, TypedError<GetWithdrawalsError>> {
        self.client.get_withdrawals(parameters).await
    }
    pub fn get_withdrawals_pages(
        &self,
        parameters: GetWithdrawalsParameters,
    ) -> Pager<'a, GetWithdrawalsParameters, GetWithdrawalsStatus200, GetWithdrawalsError> {
        self.client.get_withdrawals_pages(parameters)
    }
    pub async fn get_portfolio_resting_order_total_value(
        &self,
    ) -> Result<
        GetPortfolioRestingOrderTotalValueStatus200,
        TypedError<GetPortfolioRestingOrderTotalValueError>,
    > {
        self.client.get_portfolio_resting_order_total_value().await
    }
    pub async fn get_fills(
        &self,
        parameters: &GetFillsParameters,
    ) -> Result<GetFillsStatus200, TypedError<GetFillsError>> {
        self.client.get_fills(parameters).await
    }
    pub fn get_fills_pages(
        &self,
        parameters: GetFillsParameters,
    ) -> Pager<'a, GetFillsParameters, GetFillsStatus200, GetFillsError> {
        self.client.get_fills_pages(parameters)
    }
    pub async fn get_target_balance_allocation(
        &self,
    ) -> Result<GetTargetBalanceAllocationStatus200, TypedError<GetTargetBalanceAllocationError>>
    {
        self.client.get_target_balance_allocation().await
    }
    pub async fn set_target_balance_allocation(
        &self,
        body: &SetTargetBalanceAllocationBody,
    ) -> Result<SetTargetBalanceAllocationStatus200, TypedError<SetTargetBalanceAllocationError>>
    {
        self.client.set_target_balance_allocation(body).await
    }
}
pub struct CommunicationsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn communications(&self) -> CommunicationsApi<'_> {
        CommunicationsApi { client: self }
    }
}
impl<'a> CommunicationsApi<'a> {
    pub async fn get_communications_id(
        &self,
    ) -> Result<GetCommunicationsIdStatus200, TypedError<GetCommunicationsIdError>> {
        self.client.get_communications_id().await
    }
    pub async fn get_block_trade_proposals(
        &self,
        parameters: &GetBlockTradeProposalsParameters,
    ) -> Result<GetBlockTradeProposalsStatus200, TypedError<GetBlockTradeProposalsError>> {
        self.client.get_block_trade_proposals(parameters).await
    }
    pub fn get_block_trade_proposals_pages(
        &self,
        parameters: GetBlockTradeProposalsParameters,
    ) -> Pager<
        'a,
        GetBlockTradeProposalsParameters,
        GetBlockTradeProposalsStatus200,
        GetBlockTradeProposalsError,
    > {
        self.client.get_block_trade_proposals_pages(parameters)
    }
    pub async fn propose_block_trade(
        &self,
        body: &ProposeBlockTradeBody,
    ) -> Result<ProposeBlockTradeStatus201, TypedError<ProposeBlockTradeError>> {
        self.client.propose_block_trade(body).await
    }
    pub async fn accept_block_trade_proposal(
        &self,
        parameters: &AcceptBlockTradeProposalParameters,
        body: Option<&AcceptBlockTradeProposalBody>,
    ) -> Result<(), TypedError<AcceptBlockTradeProposalError>> {
        self.client
            .accept_block_trade_proposal(parameters, body)
            .await
    }
    pub async fn get_rf_qs(
        &self,
        parameters: &GetRfQsParameters,
    ) -> Result<GetRfQsStatus200, TypedError<GetRfQsError>> {
        self.client.get_rf_qs(parameters).await
    }
    pub fn get_rf_qs_pages(
        &self,
        parameters: GetRfQsParameters,
    ) -> Pager<'a, GetRfQsParameters, GetRfQsStatus200, GetRfQsError> {
        self.client.get_rf_qs_pages(parameters)
    }
    pub async fn create_rfq(
        &self,
        body: &CreateRfqBody,
    ) -> Result<CreateRfqStatus201, TypedError<CreateRfqError>> {
        self.client.create_rfq(body).await
    }
    pub async fn get_rfq(
        &self,
        parameters: &GetRfqParameters,
    ) -> Result<GetRfqStatus200, TypedError<GetRfqError>> {
        self.client.get_rfq(parameters).await
    }
    pub async fn delete_rfq(
        &self,
        parameters: &DeleteRfqParameters,
    ) -> Result<(), TypedError<DeleteRfqError>> {
        self.client.delete_rfq(parameters).await
    }
    pub async fn get_rfq_quote(
        &self,
        parameters: &GetRfqQuoteParameters,
    ) -> Result<GetRfqQuoteStatus200, TypedError<GetRfqQuoteError>> {
        self.client.get_rfq_quote(parameters).await
    }
    pub async fn delete_rfq_quote(
        &self,
        parameters: &DeleteRfqQuoteParameters,
    ) -> Result<(), TypedError<DeleteRfqQuoteError>> {
        self.client.delete_rfq_quote(parameters).await
    }
    pub async fn accept_rfq_quote(
        &self,
        parameters: &AcceptRfqQuoteParameters,
        body: &AcceptRfqQuoteBody,
    ) -> Result<(), TypedError<AcceptRfqQuoteError>> {
        self.client.accept_rfq_quote(parameters, body).await
    }
    pub async fn confirm_rfq_quote(
        &self,
        parameters: &ConfirmRfqQuoteParameters,
        body: Option<&ConfirmRfqQuoteBody>,
    ) -> Result<(), TypedError<ConfirmRfqQuoteError>> {
        self.client.confirm_rfq_quote(parameters, body).await
    }
    pub async fn get_quotes(
        &self,
        parameters: &GetQuotesParameters,
    ) -> Result<GetQuotesStatus200, TypedError<GetQuotesError>> {
        self.client.get_quotes(parameters).await
    }
    pub fn get_quotes_pages(
        &self,
        parameters: GetQuotesParameters,
    ) -> Pager<'a, GetQuotesParameters, GetQuotesStatus200, GetQuotesError> {
        self.client.get_quotes_pages(parameters)
    }
    pub async fn create_quote(
        &self,
        body: &CreateQuoteBody,
    ) -> Result<CreateQuoteStatus201, TypedError<CreateQuoteError>> {
        self.client.create_quote(body).await
    }
    pub async fn get_quote(
        &self,
        parameters: &GetQuoteParameters,
    ) -> Result<GetQuoteStatus200, TypedError<GetQuoteError>> {
        self.client.get_quote(parameters).await
    }
    pub async fn delete_quote(
        &self,
        parameters: &DeleteQuoteParameters,
    ) -> Result<(), TypedError<DeleteQuoteError>> {
        self.client.delete_quote(parameters).await
    }
    pub async fn accept_quote(
        &self,
        parameters: &AcceptQuoteParameters,
        body: &AcceptQuoteBody,
    ) -> Result<(), TypedError<AcceptQuoteError>> {
        self.client.accept_quote(parameters, body).await
    }
    pub async fn confirm_quote(
        &self,
        parameters: &ConfirmQuoteParameters,
        body: Option<&ConfirmQuoteBody>,
    ) -> Result<(), TypedError<ConfirmQuoteError>> {
        self.client.confirm_quote(parameters, body).await
    }
}
pub struct ApiKeysApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn api_keys(&self) -> ApiKeysApi<'_> {
        ApiKeysApi { client: self }
    }
}
impl<'a> ApiKeysApi<'a> {
    pub async fn get_api_keys(
        &self,
        parameters: &GetApiKeysParameters,
    ) -> Result<GetApiKeysStatus200, TypedError<GetApiKeysError>> {
        self.client.get_api_keys(parameters).await
    }
    pub async fn create_api_key(
        &self,
        body: &CreateApiKeyBody,
    ) -> Result<CreateApiKeyStatus201, TypedError<CreateApiKeyError>> {
        self.client.create_api_key(body).await
    }
    pub async fn generate_api_key(
        &self,
        body: &GenerateApiKeyBody,
    ) -> Result<GenerateApiKeyStatus201, TypedError<GenerateApiKeyError>> {
        self.client.generate_api_key(body).await
    }
    pub async fn delete_api_key(
        &self,
        parameters: &DeleteApiKeyParameters,
    ) -> Result<(), TypedError<DeleteApiKeyError>> {
        self.client.delete_api_key(parameters).await
    }
}
pub struct AccountApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn account(&self) -> AccountApi<'_> {
        AccountApi { client: self }
    }
}
impl<'a> AccountApi<'a> {
    pub async fn get_account_api_limits(
        &self,
    ) -> Result<GetAccountApiLimitsStatus200, TypedError<GetAccountApiLimitsError>> {
        self.client.get_account_api_limits().await
    }
    pub async fn upgrade_account_api_usage_level(
        &self,
    ) -> Result<UnspecifiedResponse, TypedError<UpgradeAccountApiUsageLevelError>> {
        self.client.upgrade_account_api_usage_level().await
    }
    pub async fn get_account_api_usage_level_volume_progress(
        &self,
    ) -> Result<
        GetAccountApiUsageLevelVolumeProgressStatus200,
        TypedError<GetAccountApiUsageLevelVolumeProgressError>,
    > {
        self.client
            .get_account_api_usage_level_volume_progress()
            .await
    }
    pub async fn get_account_endpoint_costs(
        &self,
    ) -> Result<GetAccountEndpointCostsStatus200, TypedError<GetAccountEndpointCostsError>> {
        self.client.get_account_endpoint_costs().await
    }
}
pub struct SearchApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn search(&self) -> SearchApi<'_> {
        SearchApi { client: self }
    }
}
impl<'a> SearchApi<'a> {
    pub async fn get_tags_for_series_categories(
        &self,
    ) -> Result<GetTagsForSeriesCategoriesStatus200, TypedError<GetTagsForSeriesCategoriesError>>
    {
        self.client.get_tags_for_series_categories().await
    }
    pub async fn get_filters_for_sports(
        &self,
    ) -> Result<GetFiltersForSportsStatus200, TypedError<GetFiltersForSportsError>> {
        self.client.get_filters_for_sports().await
    }
}
pub struct LiveDataApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn live_data(&self) -> LiveDataApi<'_> {
        LiveDataApi { client: self }
    }
}
impl<'a> LiveDataApi<'a> {
    pub async fn get_live_data_by_milestone(
        &self,
        parameters: &GetLiveDataByMilestoneParameters,
    ) -> Result<GetLiveDataByMilestoneStatus200, TypedError<GetLiveDataByMilestoneError>> {
        self.client.get_live_data_by_milestone(parameters).await
    }
    pub async fn get_live_data(
        &self,
        parameters: &GetLiveDataParameters,
    ) -> Result<GetLiveDataStatus200, TypedError<GetLiveDataError>> {
        self.client.get_live_data(parameters).await
    }
    pub async fn get_live_datas(
        &self,
        parameters: &GetLiveDatasParameters,
    ) -> Result<GetLiveDatasStatus200, TypedError<GetLiveDatasError>> {
        self.client.get_live_datas(parameters).await
    }
    pub async fn get_game_stats(
        &self,
        parameters: &GetGameStatsParameters,
    ) -> Result<GetGameStatsStatus200, TypedError<GetGameStatsError>> {
        self.client.get_game_stats(parameters).await
    }
    pub async fn get_event_live_data(
        &self,
        parameters: &GetEventLiveDataParameters,
    ) -> Result<GetEventLiveDataStatus200, TypedError<GetEventLiveDataError>> {
        self.client.get_event_live_data(parameters).await
    }
    pub async fn get_weather_index(
        &self,
        parameters: &GetWeatherIndexParameters,
    ) -> Result<GetWeatherIndexStatus200, TypedError<GetWeatherIndexError>> {
        self.client.get_weather_index(parameters).await
    }
    pub async fn get_weather_index_calibrations(
        &self,
        parameters: &GetWeatherIndexCalibrationsParameters,
    ) -> Result<GetWeatherIndexCalibrationsStatus200, TypedError<GetWeatherIndexCalibrationsError>>
    {
        self.client.get_weather_index_calibrations(parameters).await
    }
}
pub struct StructuredTargetsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn structured_targets(&self) -> StructuredTargetsApi<'_> {
        StructuredTargetsApi { client: self }
    }
}
impl<'a> StructuredTargetsApi<'a> {
    pub async fn get_structured_targets(
        &self,
        parameters: &GetStructuredTargetsParameters,
    ) -> Result<GetStructuredTargetsStatus200, TypedError<GetStructuredTargetsError>> {
        self.client.get_structured_targets(parameters).await
    }
    pub fn get_structured_targets_pages(
        &self,
        parameters: GetStructuredTargetsParameters,
    ) -> Pager<
        'a,
        GetStructuredTargetsParameters,
        GetStructuredTargetsStatus200,
        GetStructuredTargetsError,
    > {
        self.client.get_structured_targets_pages(parameters)
    }
    pub async fn get_structured_target(
        &self,
        parameters: &GetStructuredTargetParameters,
    ) -> Result<GetStructuredTargetStatus200, TypedError<GetStructuredTargetError>> {
        self.client.get_structured_target(parameters).await
    }
}
pub struct MilestonesApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn milestones(&self) -> MilestonesApi<'_> {
        MilestonesApi { client: self }
    }
}
impl<'a> MilestonesApi<'a> {
    pub async fn get_milestone(
        &self,
        parameters: &GetMilestoneParameters,
    ) -> Result<GetMilestoneStatus200, TypedError<GetMilestoneError>> {
        self.client.get_milestone(parameters).await
    }
    pub async fn get_milestones(
        &self,
        parameters: &GetMilestonesParameters,
    ) -> Result<GetMilestonesStatus200, TypedError<GetMilestonesError>> {
        self.client.get_milestones(parameters).await
    }
    pub fn get_milestones_pages(
        &self,
        parameters: GetMilestonesParameters,
    ) -> Pager<'a, GetMilestonesParameters, GetMilestonesStatus200, GetMilestonesError> {
        self.client.get_milestones_pages(parameters)
    }
}
pub struct MultivariateEventCollectionsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn multivariate_event_collections(&self) -> MultivariateEventCollectionsApi<'_> {
        MultivariateEventCollectionsApi { client: self }
    }
}
impl<'a> MultivariateEventCollectionsApi<'a> {
    pub async fn get_multivariate_event_collection(
        &self,
        parameters: &GetMultivariateEventCollectionParameters,
    ) -> Result<
        GetMultivariateEventCollectionStatus200,
        TypedError<GetMultivariateEventCollectionError>,
    > {
        self.client
            .get_multivariate_event_collection(parameters)
            .await
    }
    pub async fn create_market_in_multivariate_event_collection(
        &self,
        parameters: &CreateMarketInMultivariateEventCollectionParameters,
        body: &CreateMarketInMultivariateEventCollectionBody,
    ) -> Result<
        CreateMarketInMultivariateEventCollectionStatus200,
        TypedError<CreateMarketInMultivariateEventCollectionError>,
    > {
        self.client
            .create_market_in_multivariate_event_collection(parameters, body)
            .await
    }
    pub async fn get_multivariate_event_collections(
        &self,
        parameters: &GetMultivariateEventCollectionsParameters,
    ) -> Result<
        GetMultivariateEventCollectionsStatus200,
        TypedError<GetMultivariateEventCollectionsError>,
    > {
        self.client
            .get_multivariate_event_collections(parameters)
            .await
    }
    pub fn get_multivariate_event_collections_pages(
        &self,
        parameters: GetMultivariateEventCollectionsParameters,
    ) -> Pager<
        'a,
        GetMultivariateEventCollectionsParameters,
        GetMultivariateEventCollectionsStatus200,
        GetMultivariateEventCollectionsError,
    > {
        self.client
            .get_multivariate_event_collections_pages(parameters)
    }
}
pub struct IncentiveProgramsApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn incentive_programs(&self) -> IncentiveProgramsApi<'_> {
        IncentiveProgramsApi { client: self }
    }
}
impl<'a> IncentiveProgramsApi<'a> {
    pub async fn get_incentive_programs(
        &self,
        parameters: &GetIncentiveProgramsParameters,
    ) -> Result<GetIncentiveProgramsStatus200, TypedError<GetIncentiveProgramsError>> {
        self.client.get_incentive_programs(parameters).await
    }
    pub fn get_incentive_programs_pages(
        &self,
        parameters: GetIncentiveProgramsParameters,
    ) -> Pager<
        'a,
        GetIncentiveProgramsParameters,
        GetIncentiveProgramsStatus200,
        GetIncentiveProgramsError,
    > {
        self.client.get_incentive_programs_pages(parameters)
    }
}
pub struct FcmApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn fcm(&self) -> FcmApi<'_> {
        FcmApi { client: self }
    }
}
impl<'a> FcmApi<'a> {
    pub async fn get_fcm_fills(
        &self,
        parameters: &GetFcmFillsParameters,
    ) -> Result<GetFcmFillsStatus200, TypedError<GetFcmFillsError>> {
        self.client.get_fcm_fills(parameters).await
    }
    pub fn get_fcm_fills_pages(
        &self,
        parameters: GetFcmFillsParameters,
    ) -> Pager<'a, GetFcmFillsParameters, GetFcmFillsStatus200, GetFcmFillsError> {
        self.client.get_fcm_fills_pages(parameters)
    }
    pub async fn get_fcm_orders(
        &self,
        parameters: &GetFcmOrdersParameters,
    ) -> Result<GetFcmOrdersStatus200, TypedError<GetFcmOrdersError>> {
        self.client.get_fcm_orders(parameters).await
    }
    pub fn get_fcm_orders_pages(
        &self,
        parameters: GetFcmOrdersParameters,
    ) -> Pager<'a, GetFcmOrdersParameters, GetFcmOrdersStatus200, GetFcmOrdersError> {
        self.client.get_fcm_orders_pages(parameters)
    }
    pub async fn list_fcm_subtraders(
        &self,
    ) -> Result<ListFcmSubtradersStatus200, TypedError<ListFcmSubtradersError>> {
        self.client.list_fcm_subtraders().await
    }
    pub async fn create_fcm_subtrader(
        &self,
        body: &CreateFcmSubtraderBody,
    ) -> Result<CreateFcmSubtraderStatus201, TypedError<CreateFcmSubtraderError>> {
        self.client.create_fcm_subtrader(body).await
    }
    pub async fn get_fcm_event_contract_daily_cap(
        &self,
        parameters: &GetFcmEventContractDailyCapParameters,
    ) -> Result<GetFcmEventContractDailyCapStatus200, TypedError<GetFcmEventContractDailyCapError>>
    {
        self.client
            .get_fcm_event_contract_daily_cap(parameters)
            .await
    }
    pub async fn update_fcm_event_contract_daily_cap(
        &self,
        body: &UpdateFcmEventContractDailyCapBody,
    ) -> Result<
        UpdateFcmEventContractDailyCapStatus200,
        TypedError<UpdateFcmEventContractDailyCapError>,
    > {
        self.client.update_fcm_event_contract_daily_cap(body).await
    }
    pub async fn delete_fcm_event_contract_daily_cap(
        &self,
        parameters: &DeleteFcmEventContractDailyCapParameters,
    ) -> Result<
        DeleteFcmEventContractDailyCapStatus200,
        TypedError<DeleteFcmEventContractDailyCapError>,
    > {
        self.client
            .delete_fcm_event_contract_daily_cap(parameters)
            .await
    }
    pub async fn get_fcm_subtrader_blocked_categories(
        &self,
        parameters: &GetFcmSubtraderBlockedCategoriesParameters,
    ) -> Result<
        GetFcmSubtraderBlockedCategoriesStatus200,
        TypedError<GetFcmSubtraderBlockedCategoriesError>,
    > {
        self.client
            .get_fcm_subtrader_blocked_categories(parameters)
            .await
    }
    pub async fn update_fcm_subtrader_blocked_categories(
        &self,
        body: &UpdateFcmSubtraderBlockedCategoriesBody,
    ) -> Result<
        UpdateFcmSubtraderBlockedCategoriesStatus200,
        TypedError<UpdateFcmSubtraderBlockedCategoriesError>,
    > {
        self.client
            .update_fcm_subtrader_blocked_categories(body)
            .await
    }
    pub async fn get_fcm_positions(
        &self,
        parameters: &GetFcmPositionsParameters,
    ) -> Result<GetFcmPositionsStatus200, TypedError<GetFcmPositionsError>> {
        self.client.get_fcm_positions(parameters).await
    }
    pub fn get_fcm_positions_pages(
        &self,
        parameters: GetFcmPositionsParameters,
    ) -> Pager<'a, GetFcmPositionsParameters, GetFcmPositionsStatus200, GetFcmPositionsError> {
        self.client.get_fcm_positions_pages(parameters)
    }
    pub async fn create_fcm_position_transfer(
        &self,
        body: &CreateFcmPositionTransferBody,
    ) -> Result<CreateFcmPositionTransferStatus201, TypedError<CreateFcmPositionTransferError>>
    {
        self.client.create_fcm_position_transfer(body).await
    }
}
pub struct HistoricalApi<'a> {
    client: &'a KalshiClient,
}
impl KalshiClient {
    pub fn historical(&self) -> HistoricalApi<'_> {
        HistoricalApi { client: self }
    }
}
impl<'a> HistoricalApi<'a> {
    pub async fn get_historical_cutoff(
        &self,
    ) -> Result<GetHistoricalCutoffStatus200, TypedError<GetHistoricalCutoffError>> {
        self.client.get_historical_cutoff().await
    }
    pub async fn get_market_candlesticks_historical(
        &self,
        parameters: &GetMarketCandlesticksHistoricalParameters,
    ) -> Result<
        GetMarketCandlesticksHistoricalStatus200,
        TypedError<GetMarketCandlesticksHistoricalError>,
    > {
        self.client
            .get_market_candlesticks_historical(parameters)
            .await
    }
    pub async fn get_fills_historical(
        &self,
        parameters: &GetFillsHistoricalParameters,
    ) -> Result<GetFillsHistoricalStatus200, TypedError<GetFillsHistoricalError>> {
        self.client.get_fills_historical(parameters).await
    }
    pub fn get_fills_historical_pages(
        &self,
        parameters: GetFillsHistoricalParameters,
    ) -> Pager<'a, GetFillsHistoricalParameters, GetFillsHistoricalStatus200, GetFillsHistoricalError>
    {
        self.client.get_fills_historical_pages(parameters)
    }
    pub async fn get_historical_orders(
        &self,
        parameters: &GetHistoricalOrdersParameters,
    ) -> Result<GetHistoricalOrdersStatus200, TypedError<GetHistoricalOrdersError>> {
        self.client.get_historical_orders(parameters).await
    }
    pub fn get_historical_orders_pages(
        &self,
        parameters: GetHistoricalOrdersParameters,
    ) -> Pager<
        'a,
        GetHistoricalOrdersParameters,
        GetHistoricalOrdersStatus200,
        GetHistoricalOrdersError,
    > {
        self.client.get_historical_orders_pages(parameters)
    }
    pub async fn get_historical_positions(
        &self,
        parameters: &GetHistoricalPositionsParameters,
    ) -> Result<GetHistoricalPositionsStatus200, TypedError<GetHistoricalPositionsError>> {
        self.client.get_historical_positions(parameters).await
    }
    pub fn get_historical_positions_pages(
        &self,
        parameters: GetHistoricalPositionsParameters,
    ) -> Pager<
        'a,
        GetHistoricalPositionsParameters,
        GetHistoricalPositionsStatus200,
        GetHistoricalPositionsError,
    > {
        self.client.get_historical_positions_pages(parameters)
    }
    pub async fn get_trades_historical(
        &self,
        parameters: &GetTradesHistoricalParameters,
    ) -> Result<GetTradesHistoricalStatus200, TypedError<GetTradesHistoricalError>> {
        self.client.get_trades_historical(parameters).await
    }
    pub fn get_trades_historical_pages(
        &self,
        parameters: GetTradesHistoricalParameters,
    ) -> Pager<
        'a,
        GetTradesHistoricalParameters,
        GetTradesHistoricalStatus200,
        GetTradesHistoricalError,
    > {
        self.client.get_trades_historical_pages(parameters)
    }
    pub async fn get_historical_markets(
        &self,
        parameters: &GetHistoricalMarketsParameters,
    ) -> Result<GetHistoricalMarketsStatus200, TypedError<GetHistoricalMarketsError>> {
        self.client.get_historical_markets(parameters).await
    }
    pub fn get_historical_markets_pages(
        &self,
        parameters: GetHistoricalMarketsParameters,
    ) -> Pager<
        'a,
        GetHistoricalMarketsParameters,
        GetHistoricalMarketsStatus200,
        GetHistoricalMarketsError,
    > {
        self.client.get_historical_markets_pages(parameters)
    }
    pub async fn get_historical_market(
        &self,
        parameters: &GetHistoricalMarketParameters,
    ) -> Result<GetHistoricalMarketStatus200, TypedError<GetHistoricalMarketError>> {
        self.client.get_historical_market(parameters).await
    }
}
