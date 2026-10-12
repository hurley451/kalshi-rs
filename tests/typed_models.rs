// Generated model probes cover every named schema definition and recursive inline model.
mod model_support;
use kalshi_client::models::*;
use serde_json::Value;
#[test]
fn all_schema_definitions_decode_and_roundtrip() {
    let fixtures: Value = serde_json::from_str(include_str!("typed-model-fixtures.json")).unwrap();
    let full: Value = serde_json::from_str(include_str!("typed-model-full-fixtures.json")).unwrap();
    for value in [&fixtures["FixedPointDollars"], &full["FixedPointDollars"]] {
        kalshi_client::typed::validate("FixedPointDollars", value)
            .expect("FixedPointDollars fixture");
        let model: FixedPointDollars =
            serde_json::from_value(value.clone()).expect("FixedPointDollars decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("FixedPointDollars", &output)
            .expect("FixedPointDollars roundtrip");
        model_support::assert_presence(value, &output, "FixedPointDollars");
    }
    for value in [&fixtures["FixedPointCount"], &full["FixedPointCount"]] {
        kalshi_client::typed::validate("FixedPointCount", value).expect("FixedPointCount fixture");
        let model: FixedPointCount =
            serde_json::from_value(value.clone()).expect("FixedPointCount decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("FixedPointCount", &output)
            .expect("FixedPointCount roundtrip");
        model_support::assert_presence(value, &output, "FixedPointCount");
    }
    for value in [&fixtures["ExchangeIndex"], &full["ExchangeIndex"]] {
        kalshi_client::typed::validate("ExchangeIndex", value).expect("ExchangeIndex fixture");
        let model: ExchangeIndex =
            serde_json::from_value(value.clone()).expect("ExchangeIndex decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ExchangeIndex", &output).expect("ExchangeIndex roundtrip");
        model_support::assert_presence(value, &output, "ExchangeIndex");
    }
    for value in [&fixtures["FeeType"], &full["FeeType"]] {
        kalshi_client::typed::validate("FeeType", value).expect("FeeType fixture");
        let model: FeeType = serde_json::from_value(value.clone()).expect("FeeType decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("FeeType", &output).expect("FeeType roundtrip");
        model_support::assert_presence(value, &output, "FeeType");
    }
    for value in [
        &fixtures["GetMarketCandlesticksHistoricalResponse"],
        &full["GetMarketCandlesticksHistoricalResponse"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalResponse", value)
            .expect("GetMarketCandlesticksHistoricalResponse fixture");
        let model: GetMarketCandlesticksHistoricalResponse = serde_json::from_value(value.clone())
            .expect("GetMarketCandlesticksHistoricalResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalResponse", &output)
            .expect("GetMarketCandlesticksHistoricalResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksHistoricalResponse");
    }
    for value in [
        &fixtures["MarketCandlestickHistorical"],
        &full["MarketCandlestickHistorical"],
    ] {
        kalshi_client::typed::validate("MarketCandlestickHistorical", value)
            .expect("MarketCandlestickHistorical fixture");
        let model: MarketCandlestickHistorical =
            serde_json::from_value(value.clone()).expect("MarketCandlestickHistorical decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketCandlestickHistorical", &output)
            .expect("MarketCandlestickHistorical roundtrip");
        model_support::assert_presence(value, &output, "MarketCandlestickHistorical");
    }
    for value in [
        &fixtures["BidAskDistributionHistorical"],
        &full["BidAskDistributionHistorical"],
    ] {
        kalshi_client::typed::validate("BidAskDistributionHistorical", value)
            .expect("BidAskDistributionHistorical fixture");
        let model: BidAskDistributionHistorical =
            serde_json::from_value(value.clone()).expect("BidAskDistributionHistorical decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BidAskDistributionHistorical", &output)
            .expect("BidAskDistributionHistorical roundtrip");
        model_support::assert_presence(value, &output, "BidAskDistributionHistorical");
    }
    for value in [
        &fixtures["PriceDistributionHistorical"],
        &full["PriceDistributionHistorical"],
    ] {
        kalshi_client::typed::validate("PriceDistributionHistorical", value)
            .expect("PriceDistributionHistorical fixture");
        let model: PriceDistributionHistorical =
            serde_json::from_value(value.clone()).expect("PriceDistributionHistorical decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PriceDistributionHistorical", &output)
            .expect("PriceDistributionHistorical roundtrip");
        model_support::assert_presence(value, &output, "PriceDistributionHistorical");
    }
    for value in [&fixtures["ErrorResponse"], &full["ErrorResponse"]] {
        kalshi_client::typed::validate("ErrorResponse", value).expect("ErrorResponse fixture");
        let model: ErrorResponse =
            serde_json::from_value(value.clone()).expect("ErrorResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ErrorResponse", &output).expect("ErrorResponse roundtrip");
        model_support::assert_presence(value, &output, "ErrorResponse");
    }
    for value in [
        &fixtures["SelfTradePreventionType"],
        &full["SelfTradePreventionType"],
    ] {
        kalshi_client::typed::validate("SelfTradePreventionType", value)
            .expect("SelfTradePreventionType fixture");
        let model: SelfTradePreventionType =
            serde_json::from_value(value.clone()).expect("SelfTradePreventionType decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SelfTradePreventionType", &output)
            .expect("SelfTradePreventionType roundtrip");
        model_support::assert_presence(value, &output, "SelfTradePreventionType");
    }
    for value in [&fixtures["BookSide"], &full["BookSide"]] {
        kalshi_client::typed::validate("BookSide", value).expect("BookSide fixture");
        let model: BookSide = serde_json::from_value(value.clone()).expect("BookSide decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BookSide", &output).expect("BookSide roundtrip");
        model_support::assert_presence(value, &output, "BookSide");
    }
    for value in [&fixtures["OrderStatus"], &full["OrderStatus"]] {
        kalshi_client::typed::validate("OrderStatus", value).expect("OrderStatus fixture");
        let model: OrderStatus = serde_json::from_value(value.clone()).expect("OrderStatus decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("OrderStatus", &output).expect("OrderStatus roundtrip");
        model_support::assert_presence(value, &output, "OrderStatus");
    }
    for value in [&fixtures["ExchangeInstance"], &full["ExchangeInstance"]] {
        kalshi_client::typed::validate("ExchangeInstance", value)
            .expect("ExchangeInstance fixture");
        let model: ExchangeInstance =
            serde_json::from_value(value.clone()).expect("ExchangeInstance decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ExchangeInstance", &output)
            .expect("ExchangeInstance roundtrip");
        model_support::assert_presence(value, &output, "ExchangeInstance");
    }
    for value in [&fixtures["UserFilter"], &full["UserFilter"]] {
        kalshi_client::typed::validate("UserFilter", value).expect("UserFilter fixture");
        let model: UserFilter = serde_json::from_value(value.clone()).expect("UserFilter decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UserFilter", &output).expect("UserFilter roundtrip");
        model_support::assert_presence(value, &output, "UserFilter");
    }
    for value in [&fixtures["ApiKeyScope"], &full["ApiKeyScope"]] {
        kalshi_client::typed::validate("ApiKeyScope", value).expect("ApiKeyScope fixture");
        let model: ApiKeyScope = serde_json::from_value(value.clone()).expect("ApiKeyScope decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApiKeyScope", &output).expect("ApiKeyScope roundtrip");
        model_support::assert_presence(value, &output, "ApiKeyScope");
    }
    for value in [&fixtures["ApiKeyType"], &full["ApiKeyType"]] {
        kalshi_client::typed::validate("ApiKeyType", value).expect("ApiKeyType fixture");
        let model: ApiKeyType = serde_json::from_value(value.clone()).expect("ApiKeyType decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApiKeyType", &output).expect("ApiKeyType roundtrip");
        model_support::assert_presence(value, &output, "ApiKeyType");
    }
    for value in [&fixtures["ApiKey"], &full["ApiKey"]] {
        kalshi_client::typed::validate("ApiKey", value).expect("ApiKey fixture");
        let model: ApiKey = serde_json::from_value(value.clone()).expect("ApiKey decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApiKey", &output).expect("ApiKey roundtrip");
        model_support::assert_presence(value, &output, "ApiKey");
    }
    for value in [&fixtures["GetApiKeysResponse"], &full["GetApiKeysResponse"]] {
        kalshi_client::typed::validate("GetApiKeysResponse", value)
            .expect("GetApiKeysResponse fixture");
        let model: GetApiKeysResponse =
            serde_json::from_value(value.clone()).expect("GetApiKeysResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetApiKeysResponse", &output)
            .expect("GetApiKeysResponse roundtrip");
        model_support::assert_presence(value, &output, "GetApiKeysResponse");
    }
    for value in [
        &fixtures["CreateApiKeyRequest"],
        &full["CreateApiKeyRequest"],
    ] {
        kalshi_client::typed::validate("CreateApiKeyRequest", value)
            .expect("CreateApiKeyRequest fixture");
        let model: CreateApiKeyRequest =
            serde_json::from_value(value.clone()).expect("CreateApiKeyRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateApiKeyRequest", &output)
            .expect("CreateApiKeyRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateApiKeyRequest");
    }
    for value in [
        &fixtures["CreateApiKeyResponse"],
        &full["CreateApiKeyResponse"],
    ] {
        kalshi_client::typed::validate("CreateApiKeyResponse", value)
            .expect("CreateApiKeyResponse fixture");
        let model: CreateApiKeyResponse =
            serde_json::from_value(value.clone()).expect("CreateApiKeyResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateApiKeyResponse", &output)
            .expect("CreateApiKeyResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateApiKeyResponse");
    }
    for value in [
        &fixtures["GenerateApiKeyRequest"],
        &full["GenerateApiKeyRequest"],
    ] {
        kalshi_client::typed::validate("GenerateApiKeyRequest", value)
            .expect("GenerateApiKeyRequest fixture");
        let model: GenerateApiKeyRequest =
            serde_json::from_value(value.clone()).expect("GenerateApiKeyRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GenerateApiKeyRequest", &output)
            .expect("GenerateApiKeyRequest roundtrip");
        model_support::assert_presence(value, &output, "GenerateApiKeyRequest");
    }
    for value in [
        &fixtures["GenerateApiKeyResponse"],
        &full["GenerateApiKeyResponse"],
    ] {
        kalshi_client::typed::validate("GenerateApiKeyResponse", value)
            .expect("GenerateApiKeyResponse fixture");
        let model: GenerateApiKeyResponse =
            serde_json::from_value(value.clone()).expect("GenerateApiKeyResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GenerateApiKeyResponse", &output)
            .expect("GenerateApiKeyResponse roundtrip");
        model_support::assert_presence(value, &output, "GenerateApiKeyResponse");
    }
    for value in [
        &fixtures["GetTagsForSeriesCategoriesResponse"],
        &full["GetTagsForSeriesCategoriesResponse"],
    ] {
        kalshi_client::typed::validate("GetTagsForSeriesCategoriesResponse", value)
            .expect("GetTagsForSeriesCategoriesResponse fixture");
        let model: GetTagsForSeriesCategoriesResponse = serde_json::from_value(value.clone())
            .expect("GetTagsForSeriesCategoriesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTagsForSeriesCategoriesResponse", &output)
            .expect("GetTagsForSeriesCategoriesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetTagsForSeriesCategoriesResponse");
    }
    for value in [&fixtures["ScopeList"], &full["ScopeList"]] {
        kalshi_client::typed::validate("ScopeList", value).expect("ScopeList fixture");
        let model: ScopeList = serde_json::from_value(value.clone()).expect("ScopeList decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ScopeList", &output).expect("ScopeList roundtrip");
        model_support::assert_presence(value, &output, "ScopeList");
    }
    for value in [&fixtures["SportFilterDetails"], &full["SportFilterDetails"]] {
        kalshi_client::typed::validate("SportFilterDetails", value)
            .expect("SportFilterDetails fixture");
        let model: SportFilterDetails =
            serde_json::from_value(value.clone()).expect("SportFilterDetails decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SportFilterDetails", &output)
            .expect("SportFilterDetails roundtrip");
        model_support::assert_presence(value, &output, "SportFilterDetails");
    }
    for value in [
        &fixtures["GetFiltersBySportsResponse"],
        &full["GetFiltersBySportsResponse"],
    ] {
        kalshi_client::typed::validate("GetFiltersBySportsResponse", value)
            .expect("GetFiltersBySportsResponse fixture");
        let model: GetFiltersBySportsResponse =
            serde_json::from_value(value.clone()).expect("GetFiltersBySportsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFiltersBySportsResponse", &output)
            .expect("GetFiltersBySportsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetFiltersBySportsResponse");
    }
    for value in [&fixtures["BucketLimit"], &full["BucketLimit"]] {
        kalshi_client::typed::validate("BucketLimit", value).expect("BucketLimit fixture");
        let model: BucketLimit = serde_json::from_value(value.clone()).expect("BucketLimit decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BucketLimit", &output).expect("BucketLimit roundtrip");
        model_support::assert_presence(value, &output, "BucketLimit");
    }
    for value in [
        &fixtures["GetAccountApiLimitsResponse"],
        &full["GetAccountApiLimitsResponse"],
    ] {
        kalshi_client::typed::validate("GetAccountApiLimitsResponse", value)
            .expect("GetAccountApiLimitsResponse fixture");
        let model: GetAccountApiLimitsResponse =
            serde_json::from_value(value.clone()).expect("GetAccountApiLimitsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountApiLimitsResponse", &output)
            .expect("GetAccountApiLimitsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetAccountApiLimitsResponse");
    }
    for value in [&fixtures["ApiUsageLevelGrant"], &full["ApiUsageLevelGrant"]] {
        kalshi_client::typed::validate("ApiUsageLevelGrant", value)
            .expect("ApiUsageLevelGrant fixture");
        let model: ApiUsageLevelGrant =
            serde_json::from_value(value.clone()).expect("ApiUsageLevelGrant decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApiUsageLevelGrant", &output)
            .expect("ApiUsageLevelGrant roundtrip");
        model_support::assert_presence(value, &output, "ApiUsageLevelGrant");
    }
    for value in [
        &fixtures["GetAccountApiUsageLevelVolumeProgressResponse"],
        &full["GetAccountApiUsageLevelVolumeProgressResponse"],
    ] {
        kalshi_client::typed::validate("GetAccountApiUsageLevelVolumeProgressResponse", value)
            .expect("GetAccountApiUsageLevelVolumeProgressResponse fixture");
        let model: GetAccountApiUsageLevelVolumeProgressResponse =
            serde_json::from_value(value.clone())
                .expect("GetAccountApiUsageLevelVolumeProgressResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountApiUsageLevelVolumeProgressResponse", &output)
            .expect("GetAccountApiUsageLevelVolumeProgressResponse roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetAccountApiUsageLevelVolumeProgressResponse",
        );
    }
    for value in [
        &fixtures["AccountApiUsageLevelVolumeProgress"],
        &full["AccountApiUsageLevelVolumeProgress"],
    ] {
        kalshi_client::typed::validate("AccountApiUsageLevelVolumeProgress", value)
            .expect("AccountApiUsageLevelVolumeProgress fixture");
        let model: AccountApiUsageLevelVolumeProgress = serde_json::from_value(value.clone())
            .expect("AccountApiUsageLevelVolumeProgress decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AccountApiUsageLevelVolumeProgress", &output)
            .expect("AccountApiUsageLevelVolumeProgress roundtrip");
        model_support::assert_presence(value, &output, "AccountApiUsageLevelVolumeProgress");
    }
    for value in [
        &fixtures["AccountApiUsageLevelVolumeGoal"],
        &full["AccountApiUsageLevelVolumeGoal"],
    ] {
        kalshi_client::typed::validate("AccountApiUsageLevelVolumeGoal", value)
            .expect("AccountApiUsageLevelVolumeGoal fixture");
        let model: AccountApiUsageLevelVolumeGoal =
            serde_json::from_value(value.clone()).expect("AccountApiUsageLevelVolumeGoal decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AccountApiUsageLevelVolumeGoal", &output)
            .expect("AccountApiUsageLevelVolumeGoal roundtrip");
        model_support::assert_presence(value, &output, "AccountApiUsageLevelVolumeGoal");
    }
    for value in [&fixtures["EndpointTokenCost"], &full["EndpointTokenCost"]] {
        kalshi_client::typed::validate("EndpointTokenCost", value)
            .expect("EndpointTokenCost fixture");
        let model: EndpointTokenCost =
            serde_json::from_value(value.clone()).expect("EndpointTokenCost decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EndpointTokenCost", &output)
            .expect("EndpointTokenCost roundtrip");
        model_support::assert_presence(value, &output, "EndpointTokenCost");
    }
    for value in [
        &fixtures["GetAccountEndpointCostsResponse"],
        &full["GetAccountEndpointCostsResponse"],
    ] {
        kalshi_client::typed::validate("GetAccountEndpointCostsResponse", value)
            .expect("GetAccountEndpointCostsResponse fixture");
        let model: GetAccountEndpointCostsResponse =
            serde_json::from_value(value.clone()).expect("GetAccountEndpointCostsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountEndpointCostsResponse", &output)
            .expect("GetAccountEndpointCostsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetAccountEndpointCostsResponse");
    }
    for value in [&fixtures["ExchangeStatus"], &full["ExchangeStatus"]] {
        kalshi_client::typed::validate("ExchangeStatus", value).expect("ExchangeStatus fixture");
        let model: ExchangeStatus =
            serde_json::from_value(value.clone()).expect("ExchangeStatus decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ExchangeStatus", &output)
            .expect("ExchangeStatus roundtrip");
        model_support::assert_presence(value, &output, "ExchangeStatus");
    }
    for value in [
        &fixtures["ExchangeIndexStatus"],
        &full["ExchangeIndexStatus"],
    ] {
        kalshi_client::typed::validate("ExchangeIndexStatus", value)
            .expect("ExchangeIndexStatus fixture");
        let model: ExchangeIndexStatus =
            serde_json::from_value(value.clone()).expect("ExchangeIndexStatus decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ExchangeIndexStatus", &output)
            .expect("ExchangeIndexStatus roundtrip");
        model_support::assert_presence(value, &output, "ExchangeIndexStatus");
    }
    for value in [
        &fixtures["GetExchangeScheduleResponse"],
        &full["GetExchangeScheduleResponse"],
    ] {
        kalshi_client::typed::validate("GetExchangeScheduleResponse", value)
            .expect("GetExchangeScheduleResponse fixture");
        let model: GetExchangeScheduleResponse =
            serde_json::from_value(value.clone()).expect("GetExchangeScheduleResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeScheduleResponse", &output)
            .expect("GetExchangeScheduleResponse roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeScheduleResponse");
    }
    for value in [&fixtures["Schedule"], &full["Schedule"]] {
        kalshi_client::typed::validate("Schedule", value).expect("Schedule fixture");
        let model: Schedule = serde_json::from_value(value.clone()).expect("Schedule decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Schedule", &output).expect("Schedule roundtrip");
        model_support::assert_presence(value, &output, "Schedule");
    }
    for value in [&fixtures["WeeklySchedule"], &full["WeeklySchedule"]] {
        kalshi_client::typed::validate("WeeklySchedule", value).expect("WeeklySchedule fixture");
        let model: WeeklySchedule =
            serde_json::from_value(value.clone()).expect("WeeklySchedule decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("WeeklySchedule", &output)
            .expect("WeeklySchedule roundtrip");
        model_support::assert_presence(value, &output, "WeeklySchedule");
    }
    for value in [&fixtures["DailySchedule"], &full["DailySchedule"]] {
        kalshi_client::typed::validate("DailySchedule", value).expect("DailySchedule fixture");
        let model: DailySchedule =
            serde_json::from_value(value.clone()).expect("DailySchedule decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DailySchedule", &output).expect("DailySchedule roundtrip");
        model_support::assert_presence(value, &output, "DailySchedule");
    }
    for value in [&fixtures["MaintenanceWindow"], &full["MaintenanceWindow"]] {
        kalshi_client::typed::validate("MaintenanceWindow", value)
            .expect("MaintenanceWindow fixture");
        let model: MaintenanceWindow =
            serde_json::from_value(value.clone()).expect("MaintenanceWindow decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MaintenanceWindow", &output)
            .expect("MaintenanceWindow roundtrip");
        model_support::assert_presence(value, &output, "MaintenanceWindow");
    }
    for value in [
        &fixtures["GetHistoricalCutoffResponse"],
        &full["GetHistoricalCutoffResponse"],
    ] {
        kalshi_client::typed::validate("GetHistoricalCutoffResponse", value)
            .expect("GetHistoricalCutoffResponse fixture");
        let model: GetHistoricalCutoffResponse =
            serde_json::from_value(value.clone()).expect("GetHistoricalCutoffResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalCutoffResponse", &output)
            .expect("GetHistoricalCutoffResponse roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalCutoffResponse");
    }
    for value in [
        &fixtures["GetUserDataTimestampResponse"],
        &full["GetUserDataTimestampResponse"],
    ] {
        kalshi_client::typed::validate("GetUserDataTimestampResponse", value)
            .expect("GetUserDataTimestampResponse fixture");
        let model: GetUserDataTimestampResponse =
            serde_json::from_value(value.clone()).expect("GetUserDataTimestampResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetUserDataTimestampResponse", &output)
            .expect("GetUserDataTimestampResponse roundtrip");
        model_support::assert_presence(value, &output, "GetUserDataTimestampResponse");
    }
    for value in [
        &fixtures["GetMarketCandlesticksResponse"],
        &full["GetMarketCandlesticksResponse"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksResponse", value)
            .expect("GetMarketCandlesticksResponse fixture");
        let model: GetMarketCandlesticksResponse =
            serde_json::from_value(value.clone()).expect("GetMarketCandlesticksResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksResponse", &output)
            .expect("GetMarketCandlesticksResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksResponse");
    }
    for value in [
        &fixtures["GetEventCandlesticksResponse"],
        &full["GetEventCandlesticksResponse"],
    ] {
        kalshi_client::typed::validate("GetEventCandlesticksResponse", value)
            .expect("GetEventCandlesticksResponse fixture");
        let model: GetEventCandlesticksResponse =
            serde_json::from_value(value.clone()).expect("GetEventCandlesticksResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventCandlesticksResponse", &output)
            .expect("GetEventCandlesticksResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventCandlesticksResponse");
    }
    for value in [
        &fixtures["BatchGetMarketCandlesticksResponse"],
        &full["BatchGetMarketCandlesticksResponse"],
    ] {
        kalshi_client::typed::validate("BatchGetMarketCandlesticksResponse", value)
            .expect("BatchGetMarketCandlesticksResponse fixture");
        let model: BatchGetMarketCandlesticksResponse = serde_json::from_value(value.clone())
            .expect("BatchGetMarketCandlesticksResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchGetMarketCandlesticksResponse", &output)
            .expect("BatchGetMarketCandlesticksResponse roundtrip");
        model_support::assert_presence(value, &output, "BatchGetMarketCandlesticksResponse");
    }
    for value in [
        &fixtures["MarketCandlesticksResponse"],
        &full["MarketCandlesticksResponse"],
    ] {
        kalshi_client::typed::validate("MarketCandlesticksResponse", value)
            .expect("MarketCandlesticksResponse fixture");
        let model: MarketCandlesticksResponse =
            serde_json::from_value(value.clone()).expect("MarketCandlesticksResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketCandlesticksResponse", &output)
            .expect("MarketCandlesticksResponse roundtrip");
        model_support::assert_presence(value, &output, "MarketCandlesticksResponse");
    }
    for value in [&fixtures["MarketCandlestick"], &full["MarketCandlestick"]] {
        kalshi_client::typed::validate("MarketCandlestick", value)
            .expect("MarketCandlestick fixture");
        let model: MarketCandlestick =
            serde_json::from_value(value.clone()).expect("MarketCandlestick decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketCandlestick", &output)
            .expect("MarketCandlestick roundtrip");
        model_support::assert_presence(value, &output, "MarketCandlestick");
    }
    for value in [&fixtures["BidAskDistribution"], &full["BidAskDistribution"]] {
        kalshi_client::typed::validate("BidAskDistribution", value)
            .expect("BidAskDistribution fixture");
        let model: BidAskDistribution =
            serde_json::from_value(value.clone()).expect("BidAskDistribution decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BidAskDistribution", &output)
            .expect("BidAskDistribution roundtrip");
        model_support::assert_presence(value, &output, "BidAskDistribution");
    }
    for value in [&fixtures["PriceDistribution"], &full["PriceDistribution"]] {
        kalshi_client::typed::validate("PriceDistribution", value)
            .expect("PriceDistribution fixture");
        let model: PriceDistribution =
            serde_json::from_value(value.clone()).expect("PriceDistribution decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PriceDistribution", &output)
            .expect("PriceDistribution roundtrip");
        model_support::assert_presence(value, &output, "PriceDistribution");
    }
    for value in [&fixtures["LiveData"], &full["LiveData"]] {
        kalshi_client::typed::validate("LiveData", value).expect("LiveData fixture");
        let model: LiveData = serde_json::from_value(value.clone()).expect("LiveData decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("LiveData", &output).expect("LiveData roundtrip");
        model_support::assert_presence(value, &output, "LiveData");
    }
    for value in [
        &fixtures["GetLiveDataResponse"],
        &full["GetLiveDataResponse"],
    ] {
        kalshi_client::typed::validate("GetLiveDataResponse", value)
            .expect("GetLiveDataResponse fixture");
        let model: GetLiveDataResponse =
            serde_json::from_value(value.clone()).expect("GetLiveDataResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDataResponse", &output)
            .expect("GetLiveDataResponse roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDataResponse");
    }
    for value in [
        &fixtures["GetLiveDatasResponse"],
        &full["GetLiveDatasResponse"],
    ] {
        kalshi_client::typed::validate("GetLiveDatasResponse", value)
            .expect("GetLiveDatasResponse fixture");
        let model: GetLiveDatasResponse =
            serde_json::from_value(value.clone()).expect("GetLiveDatasResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDatasResponse", &output)
            .expect("GetLiveDatasResponse roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDatasResponse");
    }
    for value in [&fixtures["EventLiveData"], &full["EventLiveData"]] {
        kalshi_client::typed::validate("EventLiveData", value).expect("EventLiveData fixture");
        let model: EventLiveData =
            serde_json::from_value(value.clone()).expect("EventLiveData decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EventLiveData", &output).expect("EventLiveData roundtrip");
        model_support::assert_presence(value, &output, "EventLiveData");
    }
    for value in [
        &fixtures["GetEventLiveDataResponse"],
        &full["GetEventLiveDataResponse"],
    ] {
        kalshi_client::typed::validate("GetEventLiveDataResponse", value)
            .expect("GetEventLiveDataResponse fixture");
        let model: GetEventLiveDataResponse =
            serde_json::from_value(value.clone()).expect("GetEventLiveDataResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventLiveDataResponse", &output)
            .expect("GetEventLiveDataResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventLiveDataResponse");
    }
    for value in [
        &fixtures["GetWeatherIndexResponse"],
        &full["GetWeatherIndexResponse"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexResponse", value)
            .expect("GetWeatherIndexResponse fixture");
        let model: GetWeatherIndexResponse =
            serde_json::from_value(value.clone()).expect("GetWeatherIndexResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexResponse", &output)
            .expect("GetWeatherIndexResponse roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexResponse");
    }
    for value in [&fixtures["WeatherIndexPoint"], &full["WeatherIndexPoint"]] {
        kalshi_client::typed::validate("WeatherIndexPoint", value)
            .expect("WeatherIndexPoint fixture");
        let model: WeatherIndexPoint =
            serde_json::from_value(value.clone()).expect("WeatherIndexPoint decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("WeatherIndexPoint", &output)
            .expect("WeatherIndexPoint roundtrip");
        model_support::assert_presence(value, &output, "WeatherIndexPoint");
    }
    for value in [
        &fixtures["WeatherIndexStationReading"],
        &full["WeatherIndexStationReading"],
    ] {
        kalshi_client::typed::validate("WeatherIndexStationReading", value)
            .expect("WeatherIndexStationReading fixture");
        let model: WeatherIndexStationReading =
            serde_json::from_value(value.clone()).expect("WeatherIndexStationReading decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("WeatherIndexStationReading", &output)
            .expect("WeatherIndexStationReading roundtrip");
        model_support::assert_presence(value, &output, "WeatherIndexStationReading");
    }
    for value in [
        &fixtures["GetWeatherIndexCalibrationsResponse"],
        &full["GetWeatherIndexCalibrationsResponse"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsResponse", value)
            .expect("GetWeatherIndexCalibrationsResponse fixture");
        let model: GetWeatherIndexCalibrationsResponse = serde_json::from_value(value.clone())
            .expect("GetWeatherIndexCalibrationsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsResponse", &output)
            .expect("GetWeatherIndexCalibrationsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexCalibrationsResponse");
    }
    for value in [
        &fixtures["WeatherIndexCalibration"],
        &full["WeatherIndexCalibration"],
    ] {
        kalshi_client::typed::validate("WeatherIndexCalibration", value)
            .expect("WeatherIndexCalibration fixture");
        let model: WeatherIndexCalibration =
            serde_json::from_value(value.clone()).expect("WeatherIndexCalibration decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("WeatherIndexCalibration", &output)
            .expect("WeatherIndexCalibration roundtrip");
        model_support::assert_presence(value, &output, "WeatherIndexCalibration");
    }
    for value in [
        &fixtures["WeatherIndexCalibrationStation"],
        &full["WeatherIndexCalibrationStation"],
    ] {
        kalshi_client::typed::validate("WeatherIndexCalibrationStation", value)
            .expect("WeatherIndexCalibrationStation fixture");
        let model: WeatherIndexCalibrationStation =
            serde_json::from_value(value.clone()).expect("WeatherIndexCalibrationStation decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("WeatherIndexCalibrationStation", &output)
            .expect("WeatherIndexCalibrationStation roundtrip");
        model_support::assert_presence(value, &output, "WeatherIndexCalibrationStation");
    }
    for value in [
        &fixtures["GetGameStatsResponse"],
        &full["GetGameStatsResponse"],
    ] {
        kalshi_client::typed::validate("GetGameStatsResponse", value)
            .expect("GetGameStatsResponse fixture");
        let model: GetGameStatsResponse =
            serde_json::from_value(value.clone()).expect("GetGameStatsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetGameStatsResponse", &output)
            .expect("GetGameStatsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetGameStatsResponse");
    }
    for value in [&fixtures["PlayByPlay"], &full["PlayByPlay"]] {
        kalshi_client::typed::validate("PlayByPlay", value).expect("PlayByPlay fixture");
        let model: PlayByPlay = serde_json::from_value(value.clone()).expect("PlayByPlay decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PlayByPlay", &output).expect("PlayByPlay roundtrip");
        model_support::assert_presence(value, &output, "PlayByPlay");
    }
    for value in [&fixtures["IndexedBalance"], &full["IndexedBalance"]] {
        kalshi_client::typed::validate("IndexedBalance", value).expect("IndexedBalance fixture");
        let model: IndexedBalance =
            serde_json::from_value(value.clone()).expect("IndexedBalance decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IndexedBalance", &output)
            .expect("IndexedBalance roundtrip");
        model_support::assert_presence(value, &output, "IndexedBalance");
    }
    for value in [&fixtures["GetBalanceResponse"], &full["GetBalanceResponse"]] {
        kalshi_client::typed::validate("GetBalanceResponse", value)
            .expect("GetBalanceResponse fixture");
        let model: GetBalanceResponse =
            serde_json::from_value(value.clone()).expect("GetBalanceResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBalanceResponse", &output)
            .expect("GetBalanceResponse roundtrip");
        model_support::assert_presence(value, &output, "GetBalanceResponse");
    }
    for value in [
        &fixtures["CreateSubaccountRequest"],
        &full["CreateSubaccountRequest"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountRequest", value)
            .expect("CreateSubaccountRequest fixture");
        let model: CreateSubaccountRequest =
            serde_json::from_value(value.clone()).expect("CreateSubaccountRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountRequest", &output)
            .expect("CreateSubaccountRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountRequest");
    }
    for value in [
        &fixtures["CreateSubaccountResponse"],
        &full["CreateSubaccountResponse"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountResponse", value)
            .expect("CreateSubaccountResponse fixture");
        let model: CreateSubaccountResponse =
            serde_json::from_value(value.clone()).expect("CreateSubaccountResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountResponse", &output)
            .expect("CreateSubaccountResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountResponse");
    }
    for value in [
        &fixtures["ApplySubaccountTransferRequest"],
        &full["ApplySubaccountTransferRequest"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferRequest", value)
            .expect("ApplySubaccountTransferRequest fixture");
        let model: ApplySubaccountTransferRequest =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferRequest", &output)
            .expect("ApplySubaccountTransferRequest roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferRequest");
    }
    for value in [
        &fixtures["ApplySubaccountTransferResponse"],
        &full["ApplySubaccountTransferResponse"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferResponse", value)
            .expect("ApplySubaccountTransferResponse fixture");
        let model: ApplySubaccountTransferResponse =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferResponse", &output)
            .expect("ApplySubaccountTransferResponse roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferResponse");
    }
    for value in [
        &fixtures["GetSubaccountBalancesResponse"],
        &full["GetSubaccountBalancesResponse"],
    ] {
        kalshi_client::typed::validate("GetSubaccountBalancesResponse", value)
            .expect("GetSubaccountBalancesResponse fixture");
        let model: GetSubaccountBalancesResponse =
            serde_json::from_value(value.clone()).expect("GetSubaccountBalancesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountBalancesResponse", &output)
            .expect("GetSubaccountBalancesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountBalancesResponse");
    }
    for value in [&fixtures["SubaccountBalance"], &full["SubaccountBalance"]] {
        kalshi_client::typed::validate("SubaccountBalance", value)
            .expect("SubaccountBalance fixture");
        let model: SubaccountBalance =
            serde_json::from_value(value.clone()).expect("SubaccountBalance decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SubaccountBalance", &output)
            .expect("SubaccountBalance roundtrip");
        model_support::assert_presence(value, &output, "SubaccountBalance");
    }
    for value in [
        &fixtures["GetSubaccountTransfersResponse"],
        &full["GetSubaccountTransfersResponse"],
    ] {
        kalshi_client::typed::validate("GetSubaccountTransfersResponse", value)
            .expect("GetSubaccountTransfersResponse fixture");
        let model: GetSubaccountTransfersResponse =
            serde_json::from_value(value.clone()).expect("GetSubaccountTransfersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountTransfersResponse", &output)
            .expect("GetSubaccountTransfersResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountTransfersResponse");
    }
    for value in [&fixtures["SubaccountTransfer"], &full["SubaccountTransfer"]] {
        kalshi_client::typed::validate("SubaccountTransfer", value)
            .expect("SubaccountTransfer fixture");
        let model: SubaccountTransfer =
            serde_json::from_value(value.clone()).expect("SubaccountTransfer decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SubaccountTransfer", &output)
            .expect("SubaccountTransfer roundtrip");
        model_support::assert_presence(value, &output, "SubaccountTransfer");
    }
    for value in [
        &fixtures["UpdateSubaccountNettingRequest"],
        &full["UpdateSubaccountNettingRequest"],
    ] {
        kalshi_client::typed::validate("UpdateSubaccountNettingRequest", value)
            .expect("UpdateSubaccountNettingRequest fixture");
        let model: UpdateSubaccountNettingRequest =
            serde_json::from_value(value.clone()).expect("UpdateSubaccountNettingRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateSubaccountNettingRequest", &output)
            .expect("UpdateSubaccountNettingRequest roundtrip");
        model_support::assert_presence(value, &output, "UpdateSubaccountNettingRequest");
    }
    for value in [
        &fixtures["GetSubaccountNettingResponse"],
        &full["GetSubaccountNettingResponse"],
    ] {
        kalshi_client::typed::validate("GetSubaccountNettingResponse", value)
            .expect("GetSubaccountNettingResponse fixture");
        let model: GetSubaccountNettingResponse =
            serde_json::from_value(value.clone()).expect("GetSubaccountNettingResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountNettingResponse", &output)
            .expect("GetSubaccountNettingResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountNettingResponse");
    }
    for value in [
        &fixtures["SubaccountNettingConfig"],
        &full["SubaccountNettingConfig"],
    ] {
        kalshi_client::typed::validate("SubaccountNettingConfig", value)
            .expect("SubaccountNettingConfig fixture");
        let model: SubaccountNettingConfig =
            serde_json::from_value(value.clone()).expect("SubaccountNettingConfig decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SubaccountNettingConfig", &output)
            .expect("SubaccountNettingConfig roundtrip");
        model_support::assert_presence(value, &output, "SubaccountNettingConfig");
    }
    for value in [
        &fixtures["GetSettlementsResponse"],
        &full["GetSettlementsResponse"],
    ] {
        kalshi_client::typed::validate("GetSettlementsResponse", value)
            .expect("GetSettlementsResponse fixture");
        let model: GetSettlementsResponse =
            serde_json::from_value(value.clone()).expect("GetSettlementsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsResponse", &output)
            .expect("GetSettlementsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsResponse");
    }
    for value in [&fixtures["Settlement"], &full["Settlement"]] {
        kalshi_client::typed::validate("Settlement", value).expect("Settlement fixture");
        let model: Settlement = serde_json::from_value(value.clone()).expect("Settlement decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Settlement", &output).expect("Settlement roundtrip");
        model_support::assert_presence(value, &output, "Settlement");
    }
    for value in [
        &fixtures["GetPortfolioRestingOrderTotalValueResponse"],
        &full["GetPortfolioRestingOrderTotalValueResponse"],
    ] {
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueResponse", value)
            .expect("GetPortfolioRestingOrderTotalValueResponse fixture");
        let model: GetPortfolioRestingOrderTotalValueResponse =
            serde_json::from_value(value.clone())
                .expect("GetPortfolioRestingOrderTotalValueResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueResponse", &output)
            .expect("GetPortfolioRestingOrderTotalValueResponse roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetPortfolioRestingOrderTotalValueResponse",
        );
    }
    for value in [
        &fixtures["GetDepositsResponse"],
        &full["GetDepositsResponse"],
    ] {
        kalshi_client::typed::validate("GetDepositsResponse", value)
            .expect("GetDepositsResponse fixture");
        let model: GetDepositsResponse =
            serde_json::from_value(value.clone()).expect("GetDepositsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsResponse", &output)
            .expect("GetDepositsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsResponse");
    }
    for value in [&fixtures["Deposit"], &full["Deposit"]] {
        kalshi_client::typed::validate("Deposit", value).expect("Deposit fixture");
        let model: Deposit = serde_json::from_value(value.clone()).expect("Deposit decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Deposit", &output).expect("Deposit roundtrip");
        model_support::assert_presence(value, &output, "Deposit");
    }
    for value in [
        &fixtures["GetWithdrawalsResponse"],
        &full["GetWithdrawalsResponse"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsResponse", value)
            .expect("GetWithdrawalsResponse fixture");
        let model: GetWithdrawalsResponse =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsResponse", &output)
            .expect("GetWithdrawalsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsResponse");
    }
    for value in [&fixtures["Withdrawal"], &full["Withdrawal"]] {
        kalshi_client::typed::validate("Withdrawal", value).expect("Withdrawal fixture");
        let model: Withdrawal = serde_json::from_value(value.clone()).expect("Withdrawal decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Withdrawal", &output).expect("Withdrawal roundtrip");
        model_support::assert_presence(value, &output, "Withdrawal");
    }
    for value in [&fixtures["Order"], &full["Order"]] {
        kalshi_client::typed::validate("Order", value).expect("Order fixture");
        let model: Order = serde_json::from_value(value.clone()).expect("Order decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Order", &output).expect("Order roundtrip");
        model_support::assert_presence(value, &output, "Order");
    }
    for value in [&fixtures["Milestone"], &full["Milestone"]] {
        kalshi_client::typed::validate("Milestone", value).expect("Milestone fixture");
        let model: Milestone = serde_json::from_value(value.clone()).expect("Milestone decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Milestone", &output).expect("Milestone roundtrip");
        model_support::assert_presence(value, &output, "Milestone");
    }
    for value in [
        &fixtures["GetMilestoneResponse"],
        &full["GetMilestoneResponse"],
    ] {
        kalshi_client::typed::validate("GetMilestoneResponse", value)
            .expect("GetMilestoneResponse fixture");
        let model: GetMilestoneResponse =
            serde_json::from_value(value.clone()).expect("GetMilestoneResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestoneResponse", &output)
            .expect("GetMilestoneResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMilestoneResponse");
    }
    for value in [
        &fixtures["GetMilestonesResponse"],
        &full["GetMilestonesResponse"],
    ] {
        kalshi_client::typed::validate("GetMilestonesResponse", value)
            .expect("GetMilestonesResponse fixture");
        let model: GetMilestonesResponse =
            serde_json::from_value(value.clone()).expect("GetMilestonesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestonesResponse", &output)
            .expect("GetMilestonesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMilestonesResponse");
    }
    for value in [&fixtures["GetOrdersResponse"], &full["GetOrdersResponse"]] {
        kalshi_client::typed::validate("GetOrdersResponse", value)
            .expect("GetOrdersResponse fixture");
        let model: GetOrdersResponse =
            serde_json::from_value(value.clone()).expect("GetOrdersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersResponse", &output)
            .expect("GetOrdersResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersResponse");
    }
    for value in [
        &fixtures["GetOrderQueuePositionResponse"],
        &full["GetOrderQueuePositionResponse"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionResponse", value)
            .expect("GetOrderQueuePositionResponse fixture");
        let model: GetOrderQueuePositionResponse =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionResponse", &output)
            .expect("GetOrderQueuePositionResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionResponse");
    }
    for value in [&fixtures["OrderQueuePosition"], &full["OrderQueuePosition"]] {
        kalshi_client::typed::validate("OrderQueuePosition", value)
            .expect("OrderQueuePosition fixture");
        let model: OrderQueuePosition =
            serde_json::from_value(value.clone()).expect("OrderQueuePosition decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("OrderQueuePosition", &output)
            .expect("OrderQueuePosition roundtrip");
        model_support::assert_presence(value, &output, "OrderQueuePosition");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsResponse"],
        &full["GetOrderQueuePositionsResponse"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsResponse", value)
            .expect("GetOrderQueuePositionsResponse fixture");
        let model: GetOrderQueuePositionsResponse =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsResponse", &output)
            .expect("GetOrderQueuePositionsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsResponse");
    }
    for value in [&fixtures["MarketPosition"], &full["MarketPosition"]] {
        kalshi_client::typed::validate("MarketPosition", value).expect("MarketPosition fixture");
        let model: MarketPosition =
            serde_json::from_value(value.clone()).expect("MarketPosition decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketPosition", &output)
            .expect("MarketPosition roundtrip");
        model_support::assert_presence(value, &output, "MarketPosition");
    }
    for value in [&fixtures["EventPosition"], &full["EventPosition"]] {
        kalshi_client::typed::validate("EventPosition", value).expect("EventPosition fixture");
        let model: EventPosition =
            serde_json::from_value(value.clone()).expect("EventPosition decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EventPosition", &output).expect("EventPosition roundtrip");
        model_support::assert_presence(value, &output, "EventPosition");
    }
    for value in [
        &fixtures["GetPositionsResponse"],
        &full["GetPositionsResponse"],
    ] {
        kalshi_client::typed::validate("GetPositionsResponse", value)
            .expect("GetPositionsResponse fixture");
        let model: GetPositionsResponse =
            serde_json::from_value(value.clone()).expect("GetPositionsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsResponse", &output)
            .expect("GetPositionsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsResponse");
    }
    for value in [&fixtures["Trade"], &full["Trade"]] {
        kalshi_client::typed::validate("Trade", value).expect("Trade fixture");
        let model: Trade = serde_json::from_value(value.clone()).expect("Trade decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Trade", &output).expect("Trade roundtrip");
        model_support::assert_presence(value, &output, "Trade");
    }
    for value in [
        &fixtures["GetIncentiveProgramsResponse"],
        &full["GetIncentiveProgramsResponse"],
    ] {
        kalshi_client::typed::validate("GetIncentiveProgramsResponse", value)
            .expect("GetIncentiveProgramsResponse fixture");
        let model: GetIncentiveProgramsResponse =
            serde_json::from_value(value.clone()).expect("GetIncentiveProgramsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIncentiveProgramsResponse", &output)
            .expect("GetIncentiveProgramsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetIncentiveProgramsResponse");
    }
    for value in [&fixtures["IncentiveProgram"], &full["IncentiveProgram"]] {
        kalshi_client::typed::validate("IncentiveProgram", value)
            .expect("IncentiveProgram fixture");
        let model: IncentiveProgram =
            serde_json::from_value(value.clone()).expect("IncentiveProgram decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IncentiveProgram", &output)
            .expect("IncentiveProgram roundtrip");
        model_support::assert_presence(value, &output, "IncentiveProgram");
    }
    for value in [&fixtures["GetTradesResponse"], &full["GetTradesResponse"]] {
        kalshi_client::typed::validate("GetTradesResponse", value)
            .expect("GetTradesResponse fixture");
        let model: GetTradesResponse =
            serde_json::from_value(value.clone()).expect("GetTradesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesResponse", &output)
            .expect("GetTradesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetTradesResponse");
    }
    for value in [&fixtures["OutcomeSide"], &full["OutcomeSide"]] {
        kalshi_client::typed::validate("OutcomeSide", value).expect("OutcomeSide fixture");
        let model: OutcomeSide = serde_json::from_value(value.clone()).expect("OutcomeSide decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("OutcomeSide", &output).expect("OutcomeSide roundtrip");
        model_support::assert_presence(value, &output, "OutcomeSide");
    }
    for value in [&fixtures["Fill"], &full["Fill"]] {
        kalshi_client::typed::validate("Fill", value).expect("Fill fixture");
        let model: Fill = serde_json::from_value(value.clone()).expect("Fill decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Fill", &output).expect("Fill roundtrip");
        model_support::assert_presence(value, &output, "Fill");
    }
    for value in [&fixtures["FcmFill"], &full["FcmFill"]] {
        kalshi_client::typed::validate("FcmFill", value).expect("FcmFill fixture");
        let model: FcmFill = serde_json::from_value(value.clone()).expect("FcmFill decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("FcmFill", &output).expect("FcmFill roundtrip");
        model_support::assert_presence(value, &output, "FcmFill");
    }
    for value in [&fixtures["GetFillsResponse"], &full["GetFillsResponse"]] {
        kalshi_client::typed::validate("GetFillsResponse", value)
            .expect("GetFillsResponse fixture");
        let model: GetFillsResponse =
            serde_json::from_value(value.clone()).expect("GetFillsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsResponse", &output)
            .expect("GetFillsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetFillsResponse");
    }
    for value in [
        &fixtures["GetFcmFillsResponse"],
        &full["GetFcmFillsResponse"],
    ] {
        kalshi_client::typed::validate("GetFcmFillsResponse", value)
            .expect("GetFcmFillsResponse fixture");
        let model: GetFcmFillsResponse =
            serde_json::from_value(value.clone()).expect("GetFcmFillsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmFillsResponse", &output)
            .expect("GetFcmFillsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetFcmFillsResponse");
    }
    for value in [&fixtures["StructuredTarget"], &full["StructuredTarget"]] {
        kalshi_client::typed::validate("StructuredTarget", value)
            .expect("StructuredTarget fixture");
        let model: StructuredTarget =
            serde_json::from_value(value.clone()).expect("StructuredTarget decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("StructuredTarget", &output)
            .expect("StructuredTarget roundtrip");
        model_support::assert_presence(value, &output, "StructuredTarget");
    }
    for value in [
        &fixtures["GetStructuredTargetsResponse"],
        &full["GetStructuredTargetsResponse"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetsResponse", value)
            .expect("GetStructuredTargetsResponse fixture");
        let model: GetStructuredTargetsResponse =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetsResponse", &output)
            .expect("GetStructuredTargetsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetsResponse");
    }
    for value in [
        &fixtures["GetStructuredTargetResponse"],
        &full["GetStructuredTargetResponse"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetResponse", value)
            .expect("GetStructuredTargetResponse fixture");
        let model: GetStructuredTargetResponse =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetResponse", &output)
            .expect("GetStructuredTargetResponse roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetResponse");
    }
    for value in [&fixtures["FCMSubtrader"], &full["FCMSubtrader"]] {
        kalshi_client::typed::validate("FCMSubtrader", value).expect("FCMSubtrader fixture");
        let model: FcmSubtrader =
            serde_json::from_value(value.clone()).expect("FCMSubtrader decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("FCMSubtrader", &output).expect("FCMSubtrader roundtrip");
        model_support::assert_presence(value, &output, "FCMSubtrader");
    }
    for value in [
        &fixtures["ListFCMSubtradersResponse"],
        &full["ListFCMSubtradersResponse"],
    ] {
        kalshi_client::typed::validate("ListFCMSubtradersResponse", value)
            .expect("ListFCMSubtradersResponse fixture");
        let model: ListFcmSubtradersResponse =
            serde_json::from_value(value.clone()).expect("ListFCMSubtradersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ListFCMSubtradersResponse", &output)
            .expect("ListFCMSubtradersResponse roundtrip");
        model_support::assert_presence(value, &output, "ListFCMSubtradersResponse");
    }
    for value in [
        &fixtures["CreateFCMSubtraderRequest"],
        &full["CreateFCMSubtraderRequest"],
    ] {
        kalshi_client::typed::validate("CreateFCMSubtraderRequest", value)
            .expect("CreateFCMSubtraderRequest fixture");
        let model: CreateFcmSubtraderRequest =
            serde_json::from_value(value.clone()).expect("CreateFCMSubtraderRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFCMSubtraderRequest", &output)
            .expect("CreateFCMSubtraderRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateFCMSubtraderRequest");
    }
    for value in [
        &fixtures["CreateFCMSubtraderResponse"],
        &full["CreateFCMSubtraderResponse"],
    ] {
        kalshi_client::typed::validate("CreateFCMSubtraderResponse", value)
            .expect("CreateFCMSubtraderResponse fixture");
        let model: CreateFcmSubtraderResponse =
            serde_json::from_value(value.clone()).expect("CreateFCMSubtraderResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFCMSubtraderResponse", &output)
            .expect("CreateFCMSubtraderResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateFCMSubtraderResponse");
    }
    for value in [
        &fixtures["CreateFCMPositionTransferRequest"],
        &full["CreateFCMPositionTransferRequest"],
    ] {
        kalshi_client::typed::validate("CreateFCMPositionTransferRequest", value)
            .expect("CreateFCMPositionTransferRequest fixture");
        let model: CreateFcmPositionTransferRequest =
            serde_json::from_value(value.clone()).expect("CreateFCMPositionTransferRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFCMPositionTransferRequest", &output)
            .expect("CreateFCMPositionTransferRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateFCMPositionTransferRequest");
    }
    for value in [
        &fixtures["CreateFCMPositionTransferResponse"],
        &full["CreateFCMPositionTransferResponse"],
    ] {
        kalshi_client::typed::validate("CreateFCMPositionTransferResponse", value)
            .expect("CreateFCMPositionTransferResponse fixture");
        let model: CreateFcmPositionTransferResponse = serde_json::from_value(value.clone())
            .expect("CreateFCMPositionTransferResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFCMPositionTransferResponse", &output)
            .expect("CreateFCMPositionTransferResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateFCMPositionTransferResponse");
    }
    for value in [
        &fixtures["GetFCMEventContractDailyCapResponse"],
        &full["GetFCMEventContractDailyCapResponse"],
    ] {
        kalshi_client::typed::validate("GetFCMEventContractDailyCapResponse", value)
            .expect("GetFCMEventContractDailyCapResponse fixture");
        let model: GetFcmEventContractDailyCapResponse = serde_json::from_value(value.clone())
            .expect("GetFCMEventContractDailyCapResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFCMEventContractDailyCapResponse", &output)
            .expect("GetFCMEventContractDailyCapResponse roundtrip");
        model_support::assert_presence(value, &output, "GetFCMEventContractDailyCapResponse");
    }
    for value in [
        &fixtures["UpdateFCMEventContractDailyCapRequest"],
        &full["UpdateFCMEventContractDailyCapRequest"],
    ] {
        kalshi_client::typed::validate("UpdateFCMEventContractDailyCapRequest", value)
            .expect("UpdateFCMEventContractDailyCapRequest fixture");
        let model: UpdateFcmEventContractDailyCapRequest = serde_json::from_value(value.clone())
            .expect("UpdateFCMEventContractDailyCapRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFCMEventContractDailyCapRequest", &output)
            .expect("UpdateFCMEventContractDailyCapRequest roundtrip");
        model_support::assert_presence(value, &output, "UpdateFCMEventContractDailyCapRequest");
    }
    for value in [
        &fixtures["GetFCMSubtraderBlockedCategoriesResponse"],
        &full["GetFCMSubtraderBlockedCategoriesResponse"],
    ] {
        kalshi_client::typed::validate("GetFCMSubtraderBlockedCategoriesResponse", value)
            .expect("GetFCMSubtraderBlockedCategoriesResponse fixture");
        let model: GetFcmSubtraderBlockedCategoriesResponse = serde_json::from_value(value.clone())
            .expect("GetFCMSubtraderBlockedCategoriesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFCMSubtraderBlockedCategoriesResponse", &output)
            .expect("GetFCMSubtraderBlockedCategoriesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetFCMSubtraderBlockedCategoriesResponse");
    }
    for value in [
        &fixtures["UpdateFCMSubtraderBlockedCategoriesRequest"],
        &full["UpdateFCMSubtraderBlockedCategoriesRequest"],
    ] {
        kalshi_client::typed::validate("UpdateFCMSubtraderBlockedCategoriesRequest", value)
            .expect("UpdateFCMSubtraderBlockedCategoriesRequest fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesRequest =
            serde_json::from_value(value.clone())
                .expect("UpdateFCMSubtraderBlockedCategoriesRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFCMSubtraderBlockedCategoriesRequest", &output)
            .expect("UpdateFCMSubtraderBlockedCategoriesRequest roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFCMSubtraderBlockedCategoriesRequest",
        );
    }
    for value in [
        &fixtures["UpdateFCMSubtraderBlockedCategoriesResponse"],
        &full["UpdateFCMSubtraderBlockedCategoriesResponse"],
    ] {
        kalshi_client::typed::validate("UpdateFCMSubtraderBlockedCategoriesResponse", value)
            .expect("UpdateFCMSubtraderBlockedCategoriesResponse fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesResponse =
            serde_json::from_value(value.clone())
                .expect("UpdateFCMSubtraderBlockedCategoriesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFCMSubtraderBlockedCategoriesResponse", &output)
            .expect("UpdateFCMSubtraderBlockedCategoriesResponse roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFCMSubtraderBlockedCategoriesResponse",
        );
    }
    for value in [&fixtures["EmptyResponse"], &full["EmptyResponse"]] {
        kalshi_client::typed::validate("EmptyResponse", value).expect("EmptyResponse fixture");
        let model: EmptyResponse =
            serde_json::from_value(value.clone()).expect("EmptyResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EmptyResponse", &output).expect("EmptyResponse roundtrip");
        model_support::assert_presence(value, &output, "EmptyResponse");
    }
    for value in [
        &fixtures["TargetBalanceAllocation"],
        &full["TargetBalanceAllocation"],
    ] {
        kalshi_client::typed::validate("TargetBalanceAllocation", value)
            .expect("TargetBalanceAllocation fixture");
        let model: TargetBalanceAllocation =
            serde_json::from_value(value.clone()).expect("TargetBalanceAllocation decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TargetBalanceAllocation", &output)
            .expect("TargetBalanceAllocation roundtrip");
        model_support::assert_presence(value, &output, "TargetBalanceAllocation");
    }
    for value in [
        &fixtures["TargetBalanceAllocationInput"],
        &full["TargetBalanceAllocationInput"],
    ] {
        kalshi_client::typed::validate("TargetBalanceAllocationInput", value)
            .expect("TargetBalanceAllocationInput fixture");
        let model: TargetBalanceAllocationInput =
            serde_json::from_value(value.clone()).expect("TargetBalanceAllocationInput decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TargetBalanceAllocationInput", &output)
            .expect("TargetBalanceAllocationInput roundtrip");
        model_support::assert_presence(value, &output, "TargetBalanceAllocationInput");
    }
    for value in [
        &fixtures["RestingMarginReservation"],
        &full["RestingMarginReservation"],
    ] {
        kalshi_client::typed::validate("RestingMarginReservation", value)
            .expect("RestingMarginReservation fixture");
        let model: RestingMarginReservation =
            serde_json::from_value(value.clone()).expect("RestingMarginReservation decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("RestingMarginReservation", &output)
            .expect("RestingMarginReservation roundtrip");
        model_support::assert_presence(value, &output, "RestingMarginReservation");
    }
    for value in [
        &fixtures["GetTargetBalanceAllocationResponse"],
        &full["GetTargetBalanceAllocationResponse"],
    ] {
        kalshi_client::typed::validate("GetTargetBalanceAllocationResponse", value)
            .expect("GetTargetBalanceAllocationResponse fixture");
        let model: GetTargetBalanceAllocationResponse = serde_json::from_value(value.clone())
            .expect("GetTargetBalanceAllocationResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTargetBalanceAllocationResponse", &output)
            .expect("GetTargetBalanceAllocationResponse roundtrip");
        model_support::assert_presence(value, &output, "GetTargetBalanceAllocationResponse");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationRequest"],
        &full["SetTargetBalanceAllocationRequest"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationRequest", value)
            .expect("SetTargetBalanceAllocationRequest fixture");
        let model: SetTargetBalanceAllocationRequest = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationRequest", &output)
            .expect("SetTargetBalanceAllocationRequest roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationRequest");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferRequest"],
        &full["IntraExchangeInstanceTransferRequest"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferRequest", value)
            .expect("IntraExchangeInstanceTransferRequest fixture");
        let model: IntraExchangeInstanceTransferRequest = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferRequest", &output)
            .expect("IntraExchangeInstanceTransferRequest roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferRequest");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferResponse"],
        &full["IntraExchangeInstanceTransferResponse"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferResponse", value)
            .expect("IntraExchangeInstanceTransferResponse fixture");
        let model: IntraExchangeInstanceTransferResponse = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferResponse", &output)
            .expect("IntraExchangeInstanceTransferResponse roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferResponse");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus"],
        &full["IntraExchangeInstanceTransferStatus"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus", value)
            .expect("IntraExchangeInstanceTransferStatus fixture");
        let model: IntraExchangeInstanceTransferStatus = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus", &output)
            .expect("IntraExchangeInstanceTransferStatus roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransfer"],
        &full["IntraExchangeInstanceTransfer"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransfer", value)
            .expect("IntraExchangeInstanceTransfer fixture");
        let model: IntraExchangeInstanceTransfer =
            serde_json::from_value(value.clone()).expect("IntraExchangeInstanceTransfer decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransfer", &output)
            .expect("IntraExchangeInstanceTransfer roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransfer");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersResponse"],
        &full["GetIntraExchangeInstanceTransfersResponse"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersResponse", value)
            .expect("GetIntraExchangeInstanceTransfersResponse fixture");
        let model: GetIntraExchangeInstanceTransfersResponse =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersResponse", &output)
            .expect("GetIntraExchangeInstanceTransfersResponse roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransfersResponse");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferResponse"],
        &full["GetIntraExchangeInstanceTransferResponse"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferResponse", value)
            .expect("GetIntraExchangeInstanceTransferResponse fixture");
        let model: GetIntraExchangeInstanceTransferResponse = serde_json::from_value(value.clone())
            .expect("GetIntraExchangeInstanceTransferResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferResponse", &output)
            .expect("GetIntraExchangeInstanceTransferResponse roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransferResponse");
    }
    for value in [&fixtures["OrderGroup"], &full["OrderGroup"]] {
        kalshi_client::typed::validate("OrderGroup", value).expect("OrderGroup fixture");
        let model: OrderGroup = serde_json::from_value(value.clone()).expect("OrderGroup decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("OrderGroup", &output).expect("OrderGroup roundtrip");
        model_support::assert_presence(value, &output, "OrderGroup");
    }
    for value in [
        &fixtures["GetOrderGroupsResponse"],
        &full["GetOrderGroupsResponse"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsResponse", value)
            .expect("GetOrderGroupsResponse fixture");
        let model: GetOrderGroupsResponse =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsResponse", &output)
            .expect("GetOrderGroupsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsResponse");
    }
    for value in [
        &fixtures["GetOrderGroupResponse"],
        &full["GetOrderGroupResponse"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupResponse", value)
            .expect("GetOrderGroupResponse fixture");
        let model: GetOrderGroupResponse =
            serde_json::from_value(value.clone()).expect("GetOrderGroupResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupResponse", &output)
            .expect("GetOrderGroupResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupResponse");
    }
    for value in [
        &fixtures["CreateOrderGroupRequest"],
        &full["CreateOrderGroupRequest"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupRequest", value)
            .expect("CreateOrderGroupRequest fixture");
        let model: CreateOrderGroupRequest =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupRequest", &output)
            .expect("CreateOrderGroupRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupRequest");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitRequest"],
        &full["UpdateOrderGroupLimitRequest"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitRequest", value)
            .expect("UpdateOrderGroupLimitRequest fixture");
        let model: UpdateOrderGroupLimitRequest =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitRequest", &output)
            .expect("UpdateOrderGroupLimitRequest roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitRequest");
    }
    for value in [
        &fixtures["CreateOrderGroupResponse"],
        &full["CreateOrderGroupResponse"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupResponse", value)
            .expect("CreateOrderGroupResponse fixture");
        let model: CreateOrderGroupResponse =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupResponse", &output)
            .expect("CreateOrderGroupResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupResponse");
    }
    for value in [
        &fixtures["GetCommunicationsIDResponse"],
        &full["GetCommunicationsIDResponse"],
    ] {
        kalshi_client::typed::validate("GetCommunicationsIDResponse", value)
            .expect("GetCommunicationsIDResponse fixture");
        let model: GetCommunicationsIdResponse =
            serde_json::from_value(value.clone()).expect("GetCommunicationsIDResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetCommunicationsIDResponse", &output)
            .expect("GetCommunicationsIDResponse roundtrip");
        model_support::assert_presence(value, &output, "GetCommunicationsIDResponse");
    }
    for value in [&fixtures["BlockTradeProposal"], &full["BlockTradeProposal"]] {
        kalshi_client::typed::validate("BlockTradeProposal", value)
            .expect("BlockTradeProposal fixture");
        let model: BlockTradeProposal =
            serde_json::from_value(value.clone()).expect("BlockTradeProposal decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BlockTradeProposal", &output)
            .expect("BlockTradeProposal roundtrip");
        model_support::assert_presence(value, &output, "BlockTradeProposal");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsResponse"],
        &full["GetBlockTradeProposalsResponse"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsResponse", value)
            .expect("GetBlockTradeProposalsResponse fixture");
        let model: GetBlockTradeProposalsResponse =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsResponse", &output)
            .expect("GetBlockTradeProposalsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsResponse");
    }
    for value in [
        &fixtures["ProposeBlockTradeRequest"],
        &full["ProposeBlockTradeRequest"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeRequest", value)
            .expect("ProposeBlockTradeRequest fixture");
        let model: ProposeBlockTradeRequest =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeRequest", &output)
            .expect("ProposeBlockTradeRequest roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeRequest");
    }
    for value in [
        &fixtures["ProposeBlockTradeResponse"],
        &full["ProposeBlockTradeResponse"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeResponse", value)
            .expect("ProposeBlockTradeResponse fixture");
        let model: ProposeBlockTradeResponse =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeResponse", &output)
            .expect("ProposeBlockTradeResponse roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeResponse");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalRequest"],
        &full["AcceptBlockTradeProposalRequest"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalRequest", value)
            .expect("AcceptBlockTradeProposalRequest fixture");
        let model: AcceptBlockTradeProposalRequest =
            serde_json::from_value(value.clone()).expect("AcceptBlockTradeProposalRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalRequest", &output)
            .expect("AcceptBlockTradeProposalRequest roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalRequest");
    }
    for value in [&fixtures["RFQ"], &full["RFQ"]] {
        kalshi_client::typed::validate("RFQ", value).expect("RFQ fixture");
        let model: Rfq = serde_json::from_value(value.clone()).expect("RFQ decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("RFQ", &output).expect("RFQ roundtrip");
        model_support::assert_presence(value, &output, "RFQ");
    }
    for value in [&fixtures["GetRFQsResponse"], &full["GetRFQsResponse"]] {
        kalshi_client::typed::validate("GetRFQsResponse", value).expect("GetRFQsResponse fixture");
        let model: GetRfQsResponse =
            serde_json::from_value(value.clone()).expect("GetRFQsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRFQsResponse", &output)
            .expect("GetRFQsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetRFQsResponse");
    }
    for value in [&fixtures["GetRFQResponse"], &full["GetRFQResponse"]] {
        kalshi_client::typed::validate("GetRFQResponse", value).expect("GetRFQResponse fixture");
        let model: GetRfqResponse =
            serde_json::from_value(value.clone()).expect("GetRFQResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRFQResponse", &output)
            .expect("GetRFQResponse roundtrip");
        model_support::assert_presence(value, &output, "GetRFQResponse");
    }
    for value in [&fixtures["CreateRFQRequest"], &full["CreateRFQRequest"]] {
        kalshi_client::typed::validate("CreateRFQRequest", value)
            .expect("CreateRFQRequest fixture");
        let model: CreateRfqRequest =
            serde_json::from_value(value.clone()).expect("CreateRFQRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRFQRequest", &output)
            .expect("CreateRFQRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateRFQRequest");
    }
    for value in [&fixtures["CreateRFQResponse"], &full["CreateRFQResponse"]] {
        kalshi_client::typed::validate("CreateRFQResponse", value)
            .expect("CreateRFQResponse fixture");
        let model: CreateRfqResponse =
            serde_json::from_value(value.clone()).expect("CreateRFQResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRFQResponse", &output)
            .expect("CreateRFQResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateRFQResponse");
    }
    for value in [&fixtures["Quote"], &full["Quote"]] {
        kalshi_client::typed::validate("Quote", value).expect("Quote fixture");
        let model: Quote = serde_json::from_value(value.clone()).expect("Quote decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Quote", &output).expect("Quote roundtrip");
        model_support::assert_presence(value, &output, "Quote");
    }
    for value in [&fixtures["GetQuotesResponse"], &full["GetQuotesResponse"]] {
        kalshi_client::typed::validate("GetQuotesResponse", value)
            .expect("GetQuotesResponse fixture");
        let model: GetQuotesResponse =
            serde_json::from_value(value.clone()).expect("GetQuotesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesResponse", &output)
            .expect("GetQuotesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesResponse");
    }
    for value in [&fixtures["GetQuoteResponse"], &full["GetQuoteResponse"]] {
        kalshi_client::typed::validate("GetQuoteResponse", value)
            .expect("GetQuoteResponse fixture");
        let model: GetQuoteResponse =
            serde_json::from_value(value.clone()).expect("GetQuoteResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteResponse", &output)
            .expect("GetQuoteResponse roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteResponse");
    }
    for value in [&fixtures["CreateQuoteRequest"], &full["CreateQuoteRequest"]] {
        kalshi_client::typed::validate("CreateQuoteRequest", value)
            .expect("CreateQuoteRequest fixture");
        let model: CreateQuoteRequest =
            serde_json::from_value(value.clone()).expect("CreateQuoteRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteRequest", &output)
            .expect("CreateQuoteRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteRequest");
    }
    for value in [
        &fixtures["CreateQuoteResponse"],
        &full["CreateQuoteResponse"],
    ] {
        kalshi_client::typed::validate("CreateQuoteResponse", value)
            .expect("CreateQuoteResponse fixture");
        let model: CreateQuoteResponse =
            serde_json::from_value(value.clone()).expect("CreateQuoteResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteResponse", &output)
            .expect("CreateQuoteResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteResponse");
    }
    for value in [&fixtures["AcceptQuoteRequest"], &full["AcceptQuoteRequest"]] {
        kalshi_client::typed::validate("AcceptQuoteRequest", value)
            .expect("AcceptQuoteRequest fixture");
        let model: AcceptQuoteRequest =
            serde_json::from_value(value.clone()).expect("AcceptQuoteRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteRequest", &output)
            .expect("AcceptQuoteRequest roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteRequest");
    }
    for value in [&fixtures["GetOrderResponse"], &full["GetOrderResponse"]] {
        kalshi_client::typed::validate("GetOrderResponse", value)
            .expect("GetOrderResponse fixture");
        let model: GetOrderResponse =
            serde_json::from_value(value.clone()).expect("GetOrderResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderResponse", &output)
            .expect("GetOrderResponse roundtrip");
        model_support::assert_presence(value, &output, "GetOrderResponse");
    }
    for value in [&fixtures["CreateOrderRequest"], &full["CreateOrderRequest"]] {
        kalshi_client::typed::validate("CreateOrderRequest", value)
            .expect("CreateOrderRequest fixture");
        let model: CreateOrderRequest =
            serde_json::from_value(value.clone()).expect("CreateOrderRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderRequest", &output)
            .expect("CreateOrderRequest roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderRequest");
    }
    for value in [
        &fixtures["CreateOrderResponse"],
        &full["CreateOrderResponse"],
    ] {
        kalshi_client::typed::validate("CreateOrderResponse", value)
            .expect("CreateOrderResponse fixture");
        let model: CreateOrderResponse =
            serde_json::from_value(value.clone()).expect("CreateOrderResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderResponse", &output)
            .expect("CreateOrderResponse roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderResponse");
    }
    for value in [
        &fixtures["BatchCreateOrdersRequest"],
        &full["BatchCreateOrdersRequest"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersRequest", value)
            .expect("BatchCreateOrdersRequest fixture");
        let model: BatchCreateOrdersRequest =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersRequest", &output)
            .expect("BatchCreateOrdersRequest roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersRequest");
    }
    for value in [
        &fixtures["BatchCreateOrdersResponse"],
        &full["BatchCreateOrdersResponse"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersResponse", value)
            .expect("BatchCreateOrdersResponse fixture");
        let model: BatchCreateOrdersResponse =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersResponse", &output)
            .expect("BatchCreateOrdersResponse roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersResponse");
    }
    for value in [
        &fixtures["BatchCreateOrdersIndividualResponse"],
        &full["BatchCreateOrdersIndividualResponse"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersIndividualResponse", value)
            .expect("BatchCreateOrdersIndividualResponse fixture");
        let model: BatchCreateOrdersIndividualResponse = serde_json::from_value(value.clone())
            .expect("BatchCreateOrdersIndividualResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersIndividualResponse", &output)
            .expect("BatchCreateOrdersIndividualResponse roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersIndividualResponse");
    }
    for value in [
        &fixtures["BatchCancelOrdersRequest"],
        &full["BatchCancelOrdersRequest"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersRequest", value)
            .expect("BatchCancelOrdersRequest fixture");
        let model: BatchCancelOrdersRequest =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersRequest", &output)
            .expect("BatchCancelOrdersRequest roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersRequest");
    }
    for value in [
        &fixtures["BatchCancelOrdersRequestOrder"],
        &full["BatchCancelOrdersRequestOrder"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersRequestOrder", value)
            .expect("BatchCancelOrdersRequestOrder fixture");
        let model: BatchCancelOrdersRequestOrder =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersRequestOrder decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersRequestOrder", &output)
            .expect("BatchCancelOrdersRequestOrder roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersRequestOrder");
    }
    for value in [
        &fixtures["BatchCancelOrdersResponse"],
        &full["BatchCancelOrdersResponse"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersResponse", value)
            .expect("BatchCancelOrdersResponse fixture");
        let model: BatchCancelOrdersResponse =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersResponse", &output)
            .expect("BatchCancelOrdersResponse roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersResponse");
    }
    for value in [
        &fixtures["BatchCancelOrdersIndividualResponse"],
        &full["BatchCancelOrdersIndividualResponse"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersIndividualResponse", value)
            .expect("BatchCancelOrdersIndividualResponse fixture");
        let model: BatchCancelOrdersIndividualResponse = serde_json::from_value(value.clone())
            .expect("BatchCancelOrdersIndividualResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersIndividualResponse", &output)
            .expect("BatchCancelOrdersIndividualResponse roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersIndividualResponse");
    }
    for value in [&fixtures["AmendOrderRequest"], &full["AmendOrderRequest"]] {
        kalshi_client::typed::validate("AmendOrderRequest", value)
            .expect("AmendOrderRequest fixture");
        let model: AmendOrderRequest =
            serde_json::from_value(value.clone()).expect("AmendOrderRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderRequest", &output)
            .expect("AmendOrderRequest roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderRequest");
    }
    for value in [&fixtures["AmendOrderResponse"], &full["AmendOrderResponse"]] {
        kalshi_client::typed::validate("AmendOrderResponse", value)
            .expect("AmendOrderResponse fixture");
        let model: AmendOrderResponse =
            serde_json::from_value(value.clone()).expect("AmendOrderResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderResponse", &output)
            .expect("AmendOrderResponse roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderResponse");
    }
    for value in [
        &fixtures["CreateOrderV2Request"],
        &full["CreateOrderV2Request"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Request", value)
            .expect("CreateOrderV2Request fixture");
        let model: CreateOrderV2Request =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Request decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Request", &output)
            .expect("CreateOrderV2Request roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Request");
    }
    for value in [
        &fixtures["CreateOrderV2Response"],
        &full["CreateOrderV2Response"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Response", value)
            .expect("CreateOrderV2Response fixture");
        let model: CreateOrderV2Response =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Response", &output)
            .expect("CreateOrderV2Response roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Response");
    }
    for value in [
        &fixtures["CancelOrderV2Response"],
        &full["CancelOrderV2Response"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Response", value)
            .expect("CancelOrderV2Response fixture");
        let model: CancelOrderV2Response =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Response", &output)
            .expect("CancelOrderV2Response roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Response");
    }
    for value in [
        &fixtures["DecreaseOrderV2Request"],
        &full["DecreaseOrderV2Request"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Request", value)
            .expect("DecreaseOrderV2Request fixture");
        let model: DecreaseOrderV2Request =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Request decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Request", &output)
            .expect("DecreaseOrderV2Request roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Request");
    }
    for value in [
        &fixtures["DecreaseOrderV2Response"],
        &full["DecreaseOrderV2Response"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Response", value)
            .expect("DecreaseOrderV2Response fixture");
        let model: DecreaseOrderV2Response =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Response", &output)
            .expect("DecreaseOrderV2Response roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Response");
    }
    for value in [
        &fixtures["AmendOrderV2Request"],
        &full["AmendOrderV2Request"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Request", value)
            .expect("AmendOrderV2Request fixture");
        let model: AmendOrderV2Request =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Request decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Request", &output)
            .expect("AmendOrderV2Request roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Request");
    }
    for value in [
        &fixtures["AmendOrderV2Response"],
        &full["AmendOrderV2Response"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Response", value)
            .expect("AmendOrderV2Response fixture");
        let model: AmendOrderV2Response =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Response", &output)
            .expect("AmendOrderV2Response roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Response");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Request"],
        &full["BatchCreateOrdersV2Request"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Request", value)
            .expect("BatchCreateOrdersV2Request fixture");
        let model: BatchCreateOrdersV2Request =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Request decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Request", &output)
            .expect("BatchCreateOrdersV2Request roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Request");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Response"],
        &full["BatchCreateOrdersV2Response"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Response", value)
            .expect("BatchCreateOrdersV2Response fixture");
        let model: BatchCreateOrdersV2Response =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Response", &output)
            .expect("BatchCreateOrdersV2Response roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Response");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Request"],
        &full["BatchCancelOrdersV2Request"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Request", value)
            .expect("BatchCancelOrdersV2Request fixture");
        let model: BatchCancelOrdersV2Request =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Request decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Request", &output)
            .expect("BatchCancelOrdersV2Request roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Request");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Response"],
        &full["BatchCancelOrdersV2Response"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Response", value)
            .expect("BatchCancelOrdersV2Response fixture");
        let model: BatchCancelOrdersV2Response =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Response decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Response", &output)
            .expect("BatchCancelOrdersV2Response roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Response");
    }
    for value in [&fixtures["AssociatedEvent"], &full["AssociatedEvent"]] {
        kalshi_client::typed::validate("AssociatedEvent", value).expect("AssociatedEvent fixture");
        let model: AssociatedEvent =
            serde_json::from_value(value.clone()).expect("AssociatedEvent decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AssociatedEvent", &output)
            .expect("AssociatedEvent roundtrip");
        model_support::assert_presence(value, &output, "AssociatedEvent");
    }
    for value in [
        &fixtures["MultivariateEventCollection"],
        &full["MultivariateEventCollection"],
    ] {
        kalshi_client::typed::validate("MultivariateEventCollection", value)
            .expect("MultivariateEventCollection fixture");
        let model: MultivariateEventCollection =
            serde_json::from_value(value.clone()).expect("MultivariateEventCollection decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MultivariateEventCollection", &output)
            .expect("MultivariateEventCollection roundtrip");
        model_support::assert_presence(value, &output, "MultivariateEventCollection");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionResponse"],
        &full["GetMultivariateEventCollectionResponse"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionResponse", value)
            .expect("GetMultivariateEventCollectionResponse fixture");
        let model: GetMultivariateEventCollectionResponse = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionResponse", &output)
            .expect("GetMultivariateEventCollectionResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionResponse");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionsResponse"],
        &full["GetMultivariateEventCollectionsResponse"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionsResponse", value)
            .expect("GetMultivariateEventCollectionsResponse fixture");
        let model: GetMultivariateEventCollectionsResponse = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionsResponse", &output)
            .expect("GetMultivariateEventCollectionsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionsResponse");
    }
    for value in [&fixtures["TickerPair"], &full["TickerPair"]] {
        kalshi_client::typed::validate("TickerPair", value).expect("TickerPair fixture");
        let model: TickerPair = serde_json::from_value(value.clone()).expect("TickerPair decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TickerPair", &output).expect("TickerPair roundtrip");
        model_support::assert_presence(value, &output, "TickerPair");
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionRequest"],
        &full["CreateMarketInMultivariateEventCollectionRequest"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionRequest", value)
            .expect("CreateMarketInMultivariateEventCollectionRequest fixture");
        let model: CreateMarketInMultivariateEventCollectionRequest =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionRequest decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionRequest", &output)
            .expect("CreateMarketInMultivariateEventCollectionRequest roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionRequest",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionResponse"],
        &full["CreateMarketInMultivariateEventCollectionResponse"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionResponse", value)
            .expect("CreateMarketInMultivariateEventCollectionResponse fixture");
        let model: CreateMarketInMultivariateEventCollectionResponse =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionResponse",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionResponse roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionResponse",
        );
    }
    for value in [
        &fixtures["PriceLevelDollarsCountFp"],
        &full["PriceLevelDollarsCountFp"],
    ] {
        kalshi_client::typed::validate("PriceLevelDollarsCountFp", value)
            .expect("PriceLevelDollarsCountFp fixture");
        let model: PriceLevelDollarsCountFp =
            serde_json::from_value(value.clone()).expect("PriceLevelDollarsCountFp decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PriceLevelDollarsCountFp", &output)
            .expect("PriceLevelDollarsCountFp roundtrip");
        model_support::assert_presence(value, &output, "PriceLevelDollarsCountFp");
    }
    for value in [&fixtures["OrderbookCountFp"], &full["OrderbookCountFp"]] {
        kalshi_client::typed::validate("OrderbookCountFp", value)
            .expect("OrderbookCountFp fixture");
        let model: OrderbookCountFp =
            serde_json::from_value(value.clone()).expect("OrderbookCountFp decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("OrderbookCountFp", &output)
            .expect("OrderbookCountFp roundtrip");
        model_support::assert_presence(value, &output, "OrderbookCountFp");
    }
    for value in [
        &fixtures["GetMarketOrderbooksResponse"],
        &full["GetMarketOrderbooksResponse"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksResponse", value)
            .expect("GetMarketOrderbooksResponse fixture");
        let model: GetMarketOrderbooksResponse =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksResponse", &output)
            .expect("GetMarketOrderbooksResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksResponse");
    }
    for value in [&fixtures["MarketOrderbookFp"], &full["MarketOrderbookFp"]] {
        kalshi_client::typed::validate("MarketOrderbookFp", value)
            .expect("MarketOrderbookFp fixture");
        let model: MarketOrderbookFp =
            serde_json::from_value(value.clone()).expect("MarketOrderbookFp decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketOrderbookFp", &output)
            .expect("MarketOrderbookFp roundtrip");
        model_support::assert_presence(value, &output, "MarketOrderbookFp");
    }
    for value in [
        &fixtures["GetMarketOrderbookResponse"],
        &full["GetMarketOrderbookResponse"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookResponse", value)
            .expect("GetMarketOrderbookResponse fixture");
        let model: GetMarketOrderbookResponse =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookResponse", &output)
            .expect("GetMarketOrderbookResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookResponse");
    }
    for value in [&fixtures["GetEventsResponse"], &full["GetEventsResponse"]] {
        kalshi_client::typed::validate("GetEventsResponse", value)
            .expect("GetEventsResponse fixture");
        let model: GetEventsResponse =
            serde_json::from_value(value.clone()).expect("GetEventsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventsResponse", &output)
            .expect("GetEventsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventsResponse");
    }
    for value in [
        &fixtures["GetMultivariateEventsResponse"],
        &full["GetMultivariateEventsResponse"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventsResponse", value)
            .expect("GetMultivariateEventsResponse fixture");
        let model: GetMultivariateEventsResponse =
            serde_json::from_value(value.clone()).expect("GetMultivariateEventsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventsResponse", &output)
            .expect("GetMultivariateEventsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventsResponse");
    }
    for value in [&fixtures["EventFeeChange"], &full["EventFeeChange"]] {
        kalshi_client::typed::validate("EventFeeChange", value).expect("EventFeeChange fixture");
        let model: EventFeeChange =
            serde_json::from_value(value.clone()).expect("EventFeeChange decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EventFeeChange", &output)
            .expect("EventFeeChange roundtrip");
        model_support::assert_presence(value, &output, "EventFeeChange");
    }
    for value in [
        &fixtures["GetEventFeeChangesResponse"],
        &full["GetEventFeeChangesResponse"],
    ] {
        kalshi_client::typed::validate("GetEventFeeChangesResponse", value)
            .expect("GetEventFeeChangesResponse fixture");
        let model: GetEventFeeChangesResponse =
            serde_json::from_value(value.clone()).expect("GetEventFeeChangesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventFeeChangesResponse", &output)
            .expect("GetEventFeeChangesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventFeeChangesResponse");
    }
    for value in [&fixtures["GetEventResponse"], &full["GetEventResponse"]] {
        kalshi_client::typed::validate("GetEventResponse", value)
            .expect("GetEventResponse fixture");
        let model: GetEventResponse =
            serde_json::from_value(value.clone()).expect("GetEventResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventResponse", &output)
            .expect("GetEventResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventResponse");
    }
    for value in [&fixtures["MarketMetadata"], &full["MarketMetadata"]] {
        kalshi_client::typed::validate("MarketMetadata", value).expect("MarketMetadata fixture");
        let model: MarketMetadata =
            serde_json::from_value(value.clone()).expect("MarketMetadata decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MarketMetadata", &output)
            .expect("MarketMetadata roundtrip");
        model_support::assert_presence(value, &output, "MarketMetadata");
    }
    for value in [
        &fixtures["GetEventMetadataResponse"],
        &full["GetEventMetadataResponse"],
    ] {
        kalshi_client::typed::validate("GetEventMetadataResponse", value)
            .expect("GetEventMetadataResponse fixture");
        let model: GetEventMetadataResponse =
            serde_json::from_value(value.clone()).expect("GetEventMetadataResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventMetadataResponse", &output)
            .expect("GetEventMetadataResponse roundtrip");
        model_support::assert_presence(value, &output, "GetEventMetadataResponse");
    }
    for value in [
        &fixtures["GetEventForecastPercentilesHistoryResponse"],
        &full["GetEventForecastPercentilesHistoryResponse"],
    ] {
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryResponse", value)
            .expect("GetEventForecastPercentilesHistoryResponse fixture");
        let model: GetEventForecastPercentilesHistoryResponse =
            serde_json::from_value(value.clone())
                .expect("GetEventForecastPercentilesHistoryResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryResponse", &output)
            .expect("GetEventForecastPercentilesHistoryResponse roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetEventForecastPercentilesHistoryResponse",
        );
    }
    for value in [
        &fixtures["ForecastPercentilesPoint"],
        &full["ForecastPercentilesPoint"],
    ] {
        kalshi_client::typed::validate("ForecastPercentilesPoint", value)
            .expect("ForecastPercentilesPoint fixture");
        let model: ForecastPercentilesPoint =
            serde_json::from_value(value.clone()).expect("ForecastPercentilesPoint decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ForecastPercentilesPoint", &output)
            .expect("ForecastPercentilesPoint roundtrip");
        model_support::assert_presence(value, &output, "ForecastPercentilesPoint");
    }
    for value in [&fixtures["PercentilePoint"], &full["PercentilePoint"]] {
        kalshi_client::typed::validate("PercentilePoint", value).expect("PercentilePoint fixture");
        let model: PercentilePoint =
            serde_json::from_value(value.clone()).expect("PercentilePoint decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PercentilePoint", &output)
            .expect("PercentilePoint roundtrip");
        model_support::assert_presence(value, &output, "PercentilePoint");
    }
    for value in [&fixtures["EventData"], &full["EventData"]] {
        kalshi_client::typed::validate("EventData", value).expect("EventData fixture");
        let model: EventData = serde_json::from_value(value.clone()).expect("EventData decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("EventData", &output).expect("EventData roundtrip");
        model_support::assert_presence(value, &output, "EventData");
    }
    for value in [&fixtures["Series"], &full["Series"]] {
        kalshi_client::typed::validate("Series", value).expect("Series fixture");
        let model: Series = serde_json::from_value(value.clone()).expect("Series decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Series", &output).expect("Series roundtrip");
        model_support::assert_presence(value, &output, "Series");
    }
    for value in [&fixtures["SeriesFeeChange"], &full["SeriesFeeChange"]] {
        kalshi_client::typed::validate("SeriesFeeChange", value).expect("SeriesFeeChange fixture");
        let model: SeriesFeeChange =
            serde_json::from_value(value.clone()).expect("SeriesFeeChange decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SeriesFeeChange", &output)
            .expect("SeriesFeeChange roundtrip");
        model_support::assert_presence(value, &output, "SeriesFeeChange");
    }
    for value in [&fixtures["GetSeriesResponse"], &full["GetSeriesResponse"]] {
        kalshi_client::typed::validate("GetSeriesResponse", value)
            .expect("GetSeriesResponse fixture");
        let model: GetSeriesResponse =
            serde_json::from_value(value.clone()).expect("GetSeriesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesResponse", &output)
            .expect("GetSeriesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesResponse");
    }
    for value in [
        &fixtures["GetSeriesListResponse"],
        &full["GetSeriesListResponse"],
    ] {
        kalshi_client::typed::validate("GetSeriesListResponse", value)
            .expect("GetSeriesListResponse fixture");
        let model: GetSeriesListResponse =
            serde_json::from_value(value.clone()).expect("GetSeriesListResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesListResponse", &output)
            .expect("GetSeriesListResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesListResponse");
    }
    for value in [
        &fixtures["GetSeriesFeeChangesResponse"],
        &full["GetSeriesFeeChangesResponse"],
    ] {
        kalshi_client::typed::validate("GetSeriesFeeChangesResponse", value)
            .expect("GetSeriesFeeChangesResponse fixture");
        let model: GetSeriesFeeChangesResponse =
            serde_json::from_value(value.clone()).expect("GetSeriesFeeChangesResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesFeeChangesResponse", &output)
            .expect("GetSeriesFeeChangesResponse roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesFeeChangesResponse");
    }
    for value in [&fixtures["SettlementSource"], &full["SettlementSource"]] {
        kalshi_client::typed::validate("SettlementSource", value)
            .expect("SettlementSource fixture");
        let model: SettlementSource =
            serde_json::from_value(value.clone()).expect("SettlementSource decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SettlementSource", &output)
            .expect("SettlementSource roundtrip");
        model_support::assert_presence(value, &output, "SettlementSource");
    }
    for value in [&fixtures["GetMarketsResponse"], &full["GetMarketsResponse"]] {
        kalshi_client::typed::validate("GetMarketsResponse", value)
            .expect("GetMarketsResponse fixture");
        let model: GetMarketsResponse =
            serde_json::from_value(value.clone()).expect("GetMarketsResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketsResponse", &output)
            .expect("GetMarketsResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketsResponse");
    }
    for value in [&fixtures["GetMarketResponse"], &full["GetMarketResponse"]] {
        kalshi_client::typed::validate("GetMarketResponse", value)
            .expect("GetMarketResponse fixture");
        let model: GetMarketResponse =
            serde_json::from_value(value.clone()).expect("GetMarketResponse decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketResponse", &output)
            .expect("GetMarketResponse roundtrip");
        model_support::assert_presence(value, &output, "GetMarketResponse");
    }
    for value in [&fixtures["MveSelectedLeg"], &full["MveSelectedLeg"]] {
        kalshi_client::typed::validate("MveSelectedLeg", value).expect("MveSelectedLeg fixture");
        let model: MveSelectedLeg =
            serde_json::from_value(value.clone()).expect("MveSelectedLeg decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("MveSelectedLeg", &output)
            .expect("MveSelectedLeg roundtrip");
        model_support::assert_presence(value, &output, "MveSelectedLeg");
    }
    for value in [&fixtures["PriceRange"], &full["PriceRange"]] {
        kalshi_client::typed::validate("PriceRange", value).expect("PriceRange fixture");
        let model: PriceRange = serde_json::from_value(value.clone()).expect("PriceRange decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("PriceRange", &output).expect("PriceRange roundtrip");
        model_support::assert_presence(value, &output, "PriceRange");
    }
    for value in [&fixtures["Market"], &full["Market"]] {
        kalshi_client::typed::validate("Market", value).expect("Market fixture");
        let model: Market = serde_json::from_value(value.clone()).expect("Market decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("Market", &output).expect("Market roundtrip");
        model_support::assert_presence(value, &output, "Market");
    }
    for value in [
        &fixtures["GetExchangeStatusStatus200"],
        &full["GetExchangeStatusStatus200"],
    ] {
        kalshi_client::typed::validate("GetExchangeStatusStatus200", value)
            .expect("GetExchangeStatusStatus200 fixture");
        let model: GetExchangeStatusStatus200 =
            serde_json::from_value(value.clone()).expect("GetExchangeStatusStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeStatusStatus200", &output)
            .expect("GetExchangeStatusStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeStatusStatus200");
    }
    for value in [
        &fixtures["GetExchangeStatusStatus500"],
        &full["GetExchangeStatusStatus500"],
    ] {
        kalshi_client::typed::validate("GetExchangeStatusStatus500", value)
            .expect("GetExchangeStatusStatus500 fixture");
        let model: GetExchangeStatusStatus500 =
            serde_json::from_value(value.clone()).expect("GetExchangeStatusStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeStatusStatus500", &output)
            .expect("GetExchangeStatusStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeStatusStatus500");
    }
    for value in [
        &fixtures["GetExchangeStatusStatus503"],
        &full["GetExchangeStatusStatus503"],
    ] {
        kalshi_client::typed::validate("GetExchangeStatusStatus503", value)
            .expect("GetExchangeStatusStatus503 fixture");
        let model: GetExchangeStatusStatus503 =
            serde_json::from_value(value.clone()).expect("GetExchangeStatusStatus503 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeStatusStatus503", &output)
            .expect("GetExchangeStatusStatus503 roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeStatusStatus503");
    }
    for value in [
        &fixtures["GetExchangeStatusStatus504"],
        &full["GetExchangeStatusStatus504"],
    ] {
        kalshi_client::typed::validate("GetExchangeStatusStatus504", value)
            .expect("GetExchangeStatusStatus504 fixture");
        let model: GetExchangeStatusStatus504 =
            serde_json::from_value(value.clone()).expect("GetExchangeStatusStatus504 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeStatusStatus504", &output)
            .expect("GetExchangeStatusStatus504 roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeStatusStatus504");
    }
    for value in [
        &fixtures["GetSeriesFeeChangesParameters"],
        &full["GetSeriesFeeChangesParameters"],
    ] {
        kalshi_client::typed::validate("GetSeriesFeeChangesParameters", value)
            .expect("GetSeriesFeeChangesParameters fixture");
        let model: GetSeriesFeeChangesParameters =
            serde_json::from_value(value.clone()).expect("GetSeriesFeeChangesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesFeeChangesParameters", &output)
            .expect("GetSeriesFeeChangesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesFeeChangesParameters");
    }
    for value in [
        &fixtures["GetSeriesFeeChangesStatus200"],
        &full["GetSeriesFeeChangesStatus200"],
    ] {
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus200", value)
            .expect("GetSeriesFeeChangesStatus200 fixture");
        let model: GetSeriesFeeChangesStatus200 =
            serde_json::from_value(value.clone()).expect("GetSeriesFeeChangesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus200", &output)
            .expect("GetSeriesFeeChangesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesFeeChangesStatus200");
    }
    for value in [
        &fixtures["GetSeriesFeeChangesStatus400"],
        &full["GetSeriesFeeChangesStatus400"],
    ] {
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus400", value)
            .expect("GetSeriesFeeChangesStatus400 fixture");
        let model: GetSeriesFeeChangesStatus400 =
            serde_json::from_value(value.clone()).expect("GetSeriesFeeChangesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus400", &output)
            .expect("GetSeriesFeeChangesStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesFeeChangesStatus400");
    }
    for value in [
        &fixtures["GetSeriesFeeChangesStatus500"],
        &full["GetSeriesFeeChangesStatus500"],
    ] {
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus500", value)
            .expect("GetSeriesFeeChangesStatus500 fixture");
        let model: GetSeriesFeeChangesStatus500 =
            serde_json::from_value(value.clone()).expect("GetSeriesFeeChangesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesFeeChangesStatus500", &output)
            .expect("GetSeriesFeeChangesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesFeeChangesStatus500");
    }
    for value in [
        &fixtures["GetExchangeScheduleStatus200"],
        &full["GetExchangeScheduleStatus200"],
    ] {
        kalshi_client::typed::validate("GetExchangeScheduleStatus200", value)
            .expect("GetExchangeScheduleStatus200 fixture");
        let model: GetExchangeScheduleStatus200 =
            serde_json::from_value(value.clone()).expect("GetExchangeScheduleStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetExchangeScheduleStatus200", &output)
            .expect("GetExchangeScheduleStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetExchangeScheduleStatus200");
    }
    for value in [
        &fixtures["GetUserDataTimestampStatus200"],
        &full["GetUserDataTimestampStatus200"],
    ] {
        kalshi_client::typed::validate("GetUserDataTimestampStatus200", value)
            .expect("GetUserDataTimestampStatus200 fixture");
        let model: GetUserDataTimestampStatus200 =
            serde_json::from_value(value.clone()).expect("GetUserDataTimestampStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetUserDataTimestampStatus200", &output)
            .expect("GetUserDataTimestampStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetUserDataTimestampStatus200");
    }
    for value in [
        &fixtures["GetMarketCandlesticksParameters"],
        &full["GetMarketCandlesticksParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksParameters", value)
            .expect("GetMarketCandlesticksParameters fixture");
        let model: GetMarketCandlesticksParameters =
            serde_json::from_value(value.clone()).expect("GetMarketCandlesticksParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksParameters", &output)
            .expect("GetMarketCandlesticksParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksParameters");
    }
    for value in [
        &fixtures["GetMarketCandlesticksStatus200"],
        &full["GetMarketCandlesticksStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksStatus200", value)
            .expect("GetMarketCandlesticksStatus200 fixture");
        let model: GetMarketCandlesticksStatus200 =
            serde_json::from_value(value.clone()).expect("GetMarketCandlesticksStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksStatus200", &output)
            .expect("GetMarketCandlesticksStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksStatus200");
    }
    for value in [
        &fixtures["GetTradesParameters"],
        &full["GetTradesParameters"],
    ] {
        kalshi_client::typed::validate("GetTradesParameters", value)
            .expect("GetTradesParameters fixture");
        let model: GetTradesParameters =
            serde_json::from_value(value.clone()).expect("GetTradesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesParameters", &output)
            .expect("GetTradesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetTradesParameters");
    }
    for value in [&fixtures["GetTradesStatus200"], &full["GetTradesStatus200"]] {
        kalshi_client::typed::validate("GetTradesStatus200", value)
            .expect("GetTradesStatus200 fixture");
        let model: GetTradesStatus200 =
            serde_json::from_value(value.clone()).expect("GetTradesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesStatus200", &output)
            .expect("GetTradesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetTradesStatus200");
    }
    for value in [
        &fixtures["GetMarketOrderbookParameters"],
        &full["GetMarketOrderbookParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookParameters", value)
            .expect("GetMarketOrderbookParameters fixture");
        let model: GetMarketOrderbookParameters =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookParameters", &output)
            .expect("GetMarketOrderbookParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookParameters");
    }
    for value in [
        &fixtures["GetMarketOrderbookStatus200"],
        &full["GetMarketOrderbookStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookStatus200", value)
            .expect("GetMarketOrderbookStatus200 fixture");
        let model: GetMarketOrderbookStatus200 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookStatus200", &output)
            .expect("GetMarketOrderbookStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookStatus200");
    }
    for value in [
        &fixtures["GetMarketOrderbookStatus401"],
        &full["GetMarketOrderbookStatus401"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookStatus401", value)
            .expect("GetMarketOrderbookStatus401 fixture");
        let model: GetMarketOrderbookStatus401 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookStatus401", &output)
            .expect("GetMarketOrderbookStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookStatus401");
    }
    for value in [
        &fixtures["GetMarketOrderbookStatus404"],
        &full["GetMarketOrderbookStatus404"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookStatus404", value)
            .expect("GetMarketOrderbookStatus404 fixture");
        let model: GetMarketOrderbookStatus404 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookStatus404", &output)
            .expect("GetMarketOrderbookStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookStatus404");
    }
    for value in [
        &fixtures["GetMarketOrderbookStatus500"],
        &full["GetMarketOrderbookStatus500"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbookStatus500", value)
            .expect("GetMarketOrderbookStatus500 fixture");
        let model: GetMarketOrderbookStatus500 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbookStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbookStatus500", &output)
            .expect("GetMarketOrderbookStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbookStatus500");
    }
    for value in [
        &fixtures["GetMarketOrderbooksParameters"],
        &full["GetMarketOrderbooksParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksParameters", value)
            .expect("GetMarketOrderbooksParameters fixture");
        let model: GetMarketOrderbooksParameters =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksParameters", &output)
            .expect("GetMarketOrderbooksParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksParameters");
    }
    for value in [
        &fixtures["GetMarketOrderbooksStatus200"],
        &full["GetMarketOrderbooksStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksStatus200", value)
            .expect("GetMarketOrderbooksStatus200 fixture");
        let model: GetMarketOrderbooksStatus200 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksStatus200", &output)
            .expect("GetMarketOrderbooksStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksStatus200");
    }
    for value in [
        &fixtures["GetMarketOrderbooksStatus400"],
        &full["GetMarketOrderbooksStatus400"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksStatus400", value)
            .expect("GetMarketOrderbooksStatus400 fixture");
        let model: GetMarketOrderbooksStatus400 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksStatus400", &output)
            .expect("GetMarketOrderbooksStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksStatus400");
    }
    for value in [
        &fixtures["GetMarketOrderbooksStatus401"],
        &full["GetMarketOrderbooksStatus401"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksStatus401", value)
            .expect("GetMarketOrderbooksStatus401 fixture");
        let model: GetMarketOrderbooksStatus401 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksStatus401", &output)
            .expect("GetMarketOrderbooksStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksStatus401");
    }
    for value in [
        &fixtures["GetMarketOrderbooksStatus500"],
        &full["GetMarketOrderbooksStatus500"],
    ] {
        kalshi_client::typed::validate("GetMarketOrderbooksStatus500", value)
            .expect("GetMarketOrderbooksStatus500 fixture");
        let model: GetMarketOrderbooksStatus500 =
            serde_json::from_value(value.clone()).expect("GetMarketOrderbooksStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketOrderbooksStatus500", &output)
            .expect("GetMarketOrderbooksStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketOrderbooksStatus500");
    }
    for value in [
        &fixtures["GetSeriesParameters"],
        &full["GetSeriesParameters"],
    ] {
        kalshi_client::typed::validate("GetSeriesParameters", value)
            .expect("GetSeriesParameters fixture");
        let model: GetSeriesParameters =
            serde_json::from_value(value.clone()).expect("GetSeriesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesParameters", &output)
            .expect("GetSeriesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesParameters");
    }
    for value in [&fixtures["GetSeriesStatus200"], &full["GetSeriesStatus200"]] {
        kalshi_client::typed::validate("GetSeriesStatus200", value)
            .expect("GetSeriesStatus200 fixture");
        let model: GetSeriesStatus200 =
            serde_json::from_value(value.clone()).expect("GetSeriesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesStatus200", &output)
            .expect("GetSeriesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesStatus200");
    }
    for value in [&fixtures["GetSeriesStatus400"], &full["GetSeriesStatus400"]] {
        kalshi_client::typed::validate("GetSeriesStatus400", value)
            .expect("GetSeriesStatus400 fixture");
        let model: GetSeriesStatus400 =
            serde_json::from_value(value.clone()).expect("GetSeriesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesStatus400", &output)
            .expect("GetSeriesStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesStatus400");
    }
    for value in [&fixtures["GetSeriesStatus500"], &full["GetSeriesStatus500"]] {
        kalshi_client::typed::validate("GetSeriesStatus500", value)
            .expect("GetSeriesStatus500 fixture");
        let model: GetSeriesStatus500 =
            serde_json::from_value(value.clone()).expect("GetSeriesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesStatus500", &output)
            .expect("GetSeriesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesStatus500");
    }
    for value in [
        &fixtures["GetSeriesListParameters"],
        &full["GetSeriesListParameters"],
    ] {
        kalshi_client::typed::validate("GetSeriesListParameters", value)
            .expect("GetSeriesListParameters fixture");
        let model: GetSeriesListParameters =
            serde_json::from_value(value.clone()).expect("GetSeriesListParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesListParameters", &output)
            .expect("GetSeriesListParameters roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesListParameters");
    }
    for value in [
        &fixtures["GetSeriesListStatus200"],
        &full["GetSeriesListStatus200"],
    ] {
        kalshi_client::typed::validate("GetSeriesListStatus200", value)
            .expect("GetSeriesListStatus200 fixture");
        let model: GetSeriesListStatus200 =
            serde_json::from_value(value.clone()).expect("GetSeriesListStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesListStatus200", &output)
            .expect("GetSeriesListStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesListStatus200");
    }
    for value in [
        &fixtures["GetSeriesListStatus400"],
        &full["GetSeriesListStatus400"],
    ] {
        kalshi_client::typed::validate("GetSeriesListStatus400", value)
            .expect("GetSeriesListStatus400 fixture");
        let model: GetSeriesListStatus400 =
            serde_json::from_value(value.clone()).expect("GetSeriesListStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesListStatus400", &output)
            .expect("GetSeriesListStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesListStatus400");
    }
    for value in [
        &fixtures["GetSeriesListStatus500"],
        &full["GetSeriesListStatus500"],
    ] {
        kalshi_client::typed::validate("GetSeriesListStatus500", value)
            .expect("GetSeriesListStatus500 fixture");
        let model: GetSeriesListStatus500 =
            serde_json::from_value(value.clone()).expect("GetSeriesListStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSeriesListStatus500", &output)
            .expect("GetSeriesListStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSeriesListStatus500");
    }
    for value in [
        &fixtures["GetMarketsParameters"],
        &full["GetMarketsParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketsParameters", value)
            .expect("GetMarketsParameters fixture");
        let model: GetMarketsParameters =
            serde_json::from_value(value.clone()).expect("GetMarketsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketsParameters", &output)
            .expect("GetMarketsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketsParameters");
    }
    for value in [
        &fixtures["GetMarketsStatus200"],
        &full["GetMarketsStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketsStatus200", value)
            .expect("GetMarketsStatus200 fixture");
        let model: GetMarketsStatus200 =
            serde_json::from_value(value.clone()).expect("GetMarketsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketsStatus200", &output)
            .expect("GetMarketsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketsStatus200");
    }
    for value in [
        &fixtures["GetMarketParameters"],
        &full["GetMarketParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketParameters", value)
            .expect("GetMarketParameters fixture");
        let model: GetMarketParameters =
            serde_json::from_value(value.clone()).expect("GetMarketParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketParameters", &output)
            .expect("GetMarketParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketParameters");
    }
    for value in [&fixtures["GetMarketStatus200"], &full["GetMarketStatus200"]] {
        kalshi_client::typed::validate("GetMarketStatus200", value)
            .expect("GetMarketStatus200 fixture");
        let model: GetMarketStatus200 =
            serde_json::from_value(value.clone()).expect("GetMarketStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketStatus200", &output)
            .expect("GetMarketStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketStatus200");
    }
    for value in [
        &fixtures["BatchGetMarketCandlesticksParameters"],
        &full["BatchGetMarketCandlesticksParameters"],
    ] {
        kalshi_client::typed::validate("BatchGetMarketCandlesticksParameters", value)
            .expect("BatchGetMarketCandlesticksParameters fixture");
        let model: BatchGetMarketCandlesticksParameters = serde_json::from_value(value.clone())
            .expect("BatchGetMarketCandlesticksParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchGetMarketCandlesticksParameters", &output)
            .expect("BatchGetMarketCandlesticksParameters roundtrip");
        model_support::assert_presence(value, &output, "BatchGetMarketCandlesticksParameters");
    }
    for value in [
        &fixtures["BatchGetMarketCandlesticksStatus200"],
        &full["BatchGetMarketCandlesticksStatus200"],
    ] {
        kalshi_client::typed::validate("BatchGetMarketCandlesticksStatus200", value)
            .expect("BatchGetMarketCandlesticksStatus200 fixture");
        let model: BatchGetMarketCandlesticksStatus200 = serde_json::from_value(value.clone())
            .expect("BatchGetMarketCandlesticksStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchGetMarketCandlesticksStatus200", &output)
            .expect("BatchGetMarketCandlesticksStatus200 roundtrip");
        model_support::assert_presence(value, &output, "BatchGetMarketCandlesticksStatus200");
    }
    for value in [
        &fixtures["GetMarketCandlesticksByEventParameters"],
        &full["GetMarketCandlesticksByEventParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksByEventParameters", value)
            .expect("GetMarketCandlesticksByEventParameters fixture");
        let model: GetMarketCandlesticksByEventParameters = serde_json::from_value(value.clone())
            .expect("GetMarketCandlesticksByEventParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksByEventParameters", &output)
            .expect("GetMarketCandlesticksByEventParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksByEventParameters");
    }
    for value in [
        &fixtures["GetMarketCandlesticksByEventStatus200"],
        &full["GetMarketCandlesticksByEventStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksByEventStatus200", value)
            .expect("GetMarketCandlesticksByEventStatus200 fixture");
        let model: GetMarketCandlesticksByEventStatus200 = serde_json::from_value(value.clone())
            .expect("GetMarketCandlesticksByEventStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksByEventStatus200", &output)
            .expect("GetMarketCandlesticksByEventStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksByEventStatus200");
    }
    for value in [
        &fixtures["GetEventsParameters"],
        &full["GetEventsParameters"],
    ] {
        kalshi_client::typed::validate("GetEventsParameters", value)
            .expect("GetEventsParameters fixture");
        let model: GetEventsParameters =
            serde_json::from_value(value.clone()).expect("GetEventsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventsParameters", &output)
            .expect("GetEventsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetEventsParameters");
    }
    for value in [&fixtures["GetEventsStatus200"], &full["GetEventsStatus200"]] {
        kalshi_client::typed::validate("GetEventsStatus200", value)
            .expect("GetEventsStatus200 fixture");
        let model: GetEventsStatus200 =
            serde_json::from_value(value.clone()).expect("GetEventsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventsStatus200", &output)
            .expect("GetEventsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetEventsStatus200");
    }
    for value in [
        &fixtures["GetMultivariateEventsParameters"],
        &full["GetMultivariateEventsParameters"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventsParameters", value)
            .expect("GetMultivariateEventsParameters fixture");
        let model: GetMultivariateEventsParameters =
            serde_json::from_value(value.clone()).expect("GetMultivariateEventsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventsParameters", &output)
            .expect("GetMultivariateEventsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventsParameters");
    }
    for value in [
        &fixtures["GetMultivariateEventsStatus200"],
        &full["GetMultivariateEventsStatus200"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventsStatus200", value)
            .expect("GetMultivariateEventsStatus200 fixture");
        let model: GetMultivariateEventsStatus200 =
            serde_json::from_value(value.clone()).expect("GetMultivariateEventsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventsStatus200", &output)
            .expect("GetMultivariateEventsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventsStatus200");
    }
    for value in [
        &fixtures["GetEventFeeChangesParameters"],
        &full["GetEventFeeChangesParameters"],
    ] {
        kalshi_client::typed::validate("GetEventFeeChangesParameters", value)
            .expect("GetEventFeeChangesParameters fixture");
        let model: GetEventFeeChangesParameters =
            serde_json::from_value(value.clone()).expect("GetEventFeeChangesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventFeeChangesParameters", &output)
            .expect("GetEventFeeChangesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetEventFeeChangesParameters");
    }
    for value in [
        &fixtures["GetEventFeeChangesStatus200"],
        &full["GetEventFeeChangesStatus200"],
    ] {
        kalshi_client::typed::validate("GetEventFeeChangesStatus200", value)
            .expect("GetEventFeeChangesStatus200 fixture");
        let model: GetEventFeeChangesStatus200 =
            serde_json::from_value(value.clone()).expect("GetEventFeeChangesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventFeeChangesStatus200", &output)
            .expect("GetEventFeeChangesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetEventFeeChangesStatus200");
    }
    for value in [
        &fixtures["GetEventFeeChangesStatus400"],
        &full["GetEventFeeChangesStatus400"],
    ] {
        kalshi_client::typed::validate("GetEventFeeChangesStatus400", value)
            .expect("GetEventFeeChangesStatus400 fixture");
        let model: GetEventFeeChangesStatus400 =
            serde_json::from_value(value.clone()).expect("GetEventFeeChangesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventFeeChangesStatus400", &output)
            .expect("GetEventFeeChangesStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetEventFeeChangesStatus400");
    }
    for value in [
        &fixtures["GetEventFeeChangesStatus500"],
        &full["GetEventFeeChangesStatus500"],
    ] {
        kalshi_client::typed::validate("GetEventFeeChangesStatus500", value)
            .expect("GetEventFeeChangesStatus500 fixture");
        let model: GetEventFeeChangesStatus500 =
            serde_json::from_value(value.clone()).expect("GetEventFeeChangesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventFeeChangesStatus500", &output)
            .expect("GetEventFeeChangesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetEventFeeChangesStatus500");
    }
    for value in [&fixtures["GetEventParameters"], &full["GetEventParameters"]] {
        kalshi_client::typed::validate("GetEventParameters", value)
            .expect("GetEventParameters fixture");
        let model: GetEventParameters =
            serde_json::from_value(value.clone()).expect("GetEventParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventParameters", &output)
            .expect("GetEventParameters roundtrip");
        model_support::assert_presence(value, &output, "GetEventParameters");
    }
    for value in [&fixtures["GetEventStatus200"], &full["GetEventStatus200"]] {
        kalshi_client::typed::validate("GetEventStatus200", value)
            .expect("GetEventStatus200 fixture");
        let model: GetEventStatus200 =
            serde_json::from_value(value.clone()).expect("GetEventStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventStatus200", &output)
            .expect("GetEventStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetEventStatus200");
    }
    for value in [
        &fixtures["GetEventMetadataParameters"],
        &full["GetEventMetadataParameters"],
    ] {
        kalshi_client::typed::validate("GetEventMetadataParameters", value)
            .expect("GetEventMetadataParameters fixture");
        let model: GetEventMetadataParameters =
            serde_json::from_value(value.clone()).expect("GetEventMetadataParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventMetadataParameters", &output)
            .expect("GetEventMetadataParameters roundtrip");
        model_support::assert_presence(value, &output, "GetEventMetadataParameters");
    }
    for value in [
        &fixtures["GetEventMetadataStatus200"],
        &full["GetEventMetadataStatus200"],
    ] {
        kalshi_client::typed::validate("GetEventMetadataStatus200", value)
            .expect("GetEventMetadataStatus200 fixture");
        let model: GetEventMetadataStatus200 =
            serde_json::from_value(value.clone()).expect("GetEventMetadataStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventMetadataStatus200", &output)
            .expect("GetEventMetadataStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetEventMetadataStatus200");
    }
    for value in [
        &fixtures["GetEventForecastPercentilesHistoryParameters"],
        &full["GetEventForecastPercentilesHistoryParameters"],
    ] {
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryParameters", value)
            .expect("GetEventForecastPercentilesHistoryParameters fixture");
        let model: GetEventForecastPercentilesHistoryParameters =
            serde_json::from_value(value.clone())
                .expect("GetEventForecastPercentilesHistoryParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryParameters", &output)
            .expect("GetEventForecastPercentilesHistoryParameters roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetEventForecastPercentilesHistoryParameters",
        );
    }
    for value in [
        &fixtures["GetEventForecastPercentilesHistoryStatus200"],
        &full["GetEventForecastPercentilesHistoryStatus200"],
    ] {
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryStatus200", value)
            .expect("GetEventForecastPercentilesHistoryStatus200 fixture");
        let model: GetEventForecastPercentilesHistoryStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetEventForecastPercentilesHistoryStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventForecastPercentilesHistoryStatus200", &output)
            .expect("GetEventForecastPercentilesHistoryStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetEventForecastPercentilesHistoryStatus200",
        );
    }
    for value in [
        &fixtures["GetOrdersParameters"],
        &full["GetOrdersParameters"],
    ] {
        kalshi_client::typed::validate("GetOrdersParameters", value)
            .expect("GetOrdersParameters fixture");
        let model: GetOrdersParameters =
            serde_json::from_value(value.clone()).expect("GetOrdersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersParameters", &output)
            .expect("GetOrdersParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersParameters");
    }
    for value in [&fixtures["GetOrdersStatus200"], &full["GetOrdersStatus200"]] {
        kalshi_client::typed::validate("GetOrdersStatus200", value)
            .expect("GetOrdersStatus200 fixture");
        let model: GetOrdersStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrdersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersStatus200", &output)
            .expect("GetOrdersStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersStatus200");
    }
    for value in [&fixtures["GetOrdersStatus400"], &full["GetOrdersStatus400"]] {
        kalshi_client::typed::validate("GetOrdersStatus400", value)
            .expect("GetOrdersStatus400 fixture");
        let model: GetOrdersStatus400 =
            serde_json::from_value(value.clone()).expect("GetOrdersStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersStatus400", &output)
            .expect("GetOrdersStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersStatus400");
    }
    for value in [&fixtures["GetOrdersStatus401"], &full["GetOrdersStatus401"]] {
        kalshi_client::typed::validate("GetOrdersStatus401", value)
            .expect("GetOrdersStatus401 fixture");
        let model: GetOrdersStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrdersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersStatus401", &output)
            .expect("GetOrdersStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersStatus401");
    }
    for value in [&fixtures["GetOrdersStatus500"], &full["GetOrdersStatus500"]] {
        kalshi_client::typed::validate("GetOrdersStatus500", value)
            .expect("GetOrdersStatus500 fixture");
        let model: GetOrdersStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrdersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrdersStatus500", &output)
            .expect("GetOrdersStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrdersStatus500");
    }
    for value in [&fixtures["GetOrderParameters"], &full["GetOrderParameters"]] {
        kalshi_client::typed::validate("GetOrderParameters", value)
            .expect("GetOrderParameters fixture");
        let model: GetOrderParameters =
            serde_json::from_value(value.clone()).expect("GetOrderParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderParameters", &output)
            .expect("GetOrderParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrderParameters");
    }
    for value in [&fixtures["GetOrderStatus200"], &full["GetOrderStatus200"]] {
        kalshi_client::typed::validate("GetOrderStatus200", value)
            .expect("GetOrderStatus200 fixture");
        let model: GetOrderStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrderStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderStatus200", &output)
            .expect("GetOrderStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderStatus200");
    }
    for value in [&fixtures["GetOrderStatus401"], &full["GetOrderStatus401"]] {
        kalshi_client::typed::validate("GetOrderStatus401", value)
            .expect("GetOrderStatus401 fixture");
        let model: GetOrderStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrderStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderStatus401", &output)
            .expect("GetOrderStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderStatus401");
    }
    for value in [&fixtures["GetOrderStatus404"], &full["GetOrderStatus404"]] {
        kalshi_client::typed::validate("GetOrderStatus404", value)
            .expect("GetOrderStatus404 fixture");
        let model: GetOrderStatus404 =
            serde_json::from_value(value.clone()).expect("GetOrderStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderStatus404", &output)
            .expect("GetOrderStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderStatus404");
    }
    for value in [&fixtures["GetOrderStatus500"], &full["GetOrderStatus500"]] {
        kalshi_client::typed::validate("GetOrderStatus500", value)
            .expect("GetOrderStatus500 fixture");
        let model: GetOrderStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrderStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderStatus500", &output)
            .expect("GetOrderStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderStatus500");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsParameters"],
        &full["GetOrderQueuePositionsParameters"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsParameters", value)
            .expect("GetOrderQueuePositionsParameters fixture");
        let model: GetOrderQueuePositionsParameters =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsParameters", &output)
            .expect("GetOrderQueuePositionsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsParameters");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsStatus200"],
        &full["GetOrderQueuePositionsStatus200"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus200", value)
            .expect("GetOrderQueuePositionsStatus200 fixture");
        let model: GetOrderQueuePositionsStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus200", &output)
            .expect("GetOrderQueuePositionsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsStatus200");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsStatus400"],
        &full["GetOrderQueuePositionsStatus400"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus400", value)
            .expect("GetOrderQueuePositionsStatus400 fixture");
        let model: GetOrderQueuePositionsStatus400 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus400", &output)
            .expect("GetOrderQueuePositionsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsStatus400");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsStatus401"],
        &full["GetOrderQueuePositionsStatus401"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus401", value)
            .expect("GetOrderQueuePositionsStatus401 fixture");
        let model: GetOrderQueuePositionsStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus401", &output)
            .expect("GetOrderQueuePositionsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsStatus401");
    }
    for value in [
        &fixtures["GetOrderQueuePositionsStatus500"],
        &full["GetOrderQueuePositionsStatus500"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus500", value)
            .expect("GetOrderQueuePositionsStatus500 fixture");
        let model: GetOrderQueuePositionsStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionsStatus500", &output)
            .expect("GetOrderQueuePositionsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionsStatus500");
    }
    for value in [
        &fixtures["GetOrderQueuePositionParameters"],
        &full["GetOrderQueuePositionParameters"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionParameters", value)
            .expect("GetOrderQueuePositionParameters fixture");
        let model: GetOrderQueuePositionParameters =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionParameters", &output)
            .expect("GetOrderQueuePositionParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionParameters");
    }
    for value in [
        &fixtures["GetOrderQueuePositionStatus200"],
        &full["GetOrderQueuePositionStatus200"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionStatus200", value)
            .expect("GetOrderQueuePositionStatus200 fixture");
        let model: GetOrderQueuePositionStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionStatus200", &output)
            .expect("GetOrderQueuePositionStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionStatus200");
    }
    for value in [
        &fixtures["GetOrderQueuePositionStatus401"],
        &full["GetOrderQueuePositionStatus401"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionStatus401", value)
            .expect("GetOrderQueuePositionStatus401 fixture");
        let model: GetOrderQueuePositionStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionStatus401", &output)
            .expect("GetOrderQueuePositionStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionStatus401");
    }
    for value in [
        &fixtures["GetOrderQueuePositionStatus404"],
        &full["GetOrderQueuePositionStatus404"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionStatus404", value)
            .expect("GetOrderQueuePositionStatus404 fixture");
        let model: GetOrderQueuePositionStatus404 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionStatus404", &output)
            .expect("GetOrderQueuePositionStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionStatus404");
    }
    for value in [
        &fixtures["GetOrderQueuePositionStatus500"],
        &full["GetOrderQueuePositionStatus500"],
    ] {
        kalshi_client::typed::validate("GetOrderQueuePositionStatus500", value)
            .expect("GetOrderQueuePositionStatus500 fixture");
        let model: GetOrderQueuePositionStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrderQueuePositionStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderQueuePositionStatus500", &output)
            .expect("GetOrderQueuePositionStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderQueuePositionStatus500");
    }
    for value in [&fixtures["CreateOrderV2Body"], &full["CreateOrderV2Body"]] {
        kalshi_client::typed::validate("CreateOrderV2Body", value)
            .expect("CreateOrderV2Body fixture");
        let model: CreateOrderV2Body =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Body decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Body", &output)
            .expect("CreateOrderV2Body roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Body");
    }
    for value in [
        &fixtures["CreateOrderV2Status201"],
        &full["CreateOrderV2Status201"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status201", value)
            .expect("CreateOrderV2Status201 fixture");
        let model: CreateOrderV2Status201 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status201", &output)
            .expect("CreateOrderV2Status201 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status201");
    }
    for value in [
        &fixtures["CreateOrderV2Status400"],
        &full["CreateOrderV2Status400"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status400", value)
            .expect("CreateOrderV2Status400 fixture");
        let model: CreateOrderV2Status400 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status400", &output)
            .expect("CreateOrderV2Status400 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status400");
    }
    for value in [
        &fixtures["CreateOrderV2Status401"],
        &full["CreateOrderV2Status401"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status401", value)
            .expect("CreateOrderV2Status401 fixture");
        let model: CreateOrderV2Status401 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status401", &output)
            .expect("CreateOrderV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status401");
    }
    for value in [
        &fixtures["CreateOrderV2Status403"],
        &full["CreateOrderV2Status403"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status403", value)
            .expect("CreateOrderV2Status403 fixture");
        let model: CreateOrderV2Status403 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status403", &output)
            .expect("CreateOrderV2Status403 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status403");
    }
    for value in [
        &fixtures["CreateOrderV2Status409"],
        &full["CreateOrderV2Status409"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status409", value)
            .expect("CreateOrderV2Status409 fixture");
        let model: CreateOrderV2Status409 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status409 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status409", &output)
            .expect("CreateOrderV2Status409 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status409");
    }
    for value in [
        &fixtures["CreateOrderV2Status429"],
        &full["CreateOrderV2Status429"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status429", value)
            .expect("CreateOrderV2Status429 fixture");
        let model: CreateOrderV2Status429 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status429 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status429", &output)
            .expect("CreateOrderV2Status429 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status429");
    }
    for value in [
        &fixtures["CreateOrderV2Status500"],
        &full["CreateOrderV2Status500"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status500", value)
            .expect("CreateOrderV2Status500 fixture");
        let model: CreateOrderV2Status500 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status500", &output)
            .expect("CreateOrderV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status500");
    }
    for value in [
        &fixtures["CreateOrderV2Status503"],
        &full["CreateOrderV2Status503"],
    ] {
        kalshi_client::typed::validate("CreateOrderV2Status503", value)
            .expect("CreateOrderV2Status503 fixture");
        let model: CreateOrderV2Status503 =
            serde_json::from_value(value.clone()).expect("CreateOrderV2Status503 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderV2Status503", &output)
            .expect("CreateOrderV2Status503 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderV2Status503");
    }
    for value in [
        &fixtures["CancelAllOrdersParameters"],
        &full["CancelAllOrdersParameters"],
    ] {
        kalshi_client::typed::validate("CancelAllOrdersParameters", value)
            .expect("CancelAllOrdersParameters fixture");
        let model: CancelAllOrdersParameters =
            serde_json::from_value(value.clone()).expect("CancelAllOrdersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelAllOrdersParameters", &output)
            .expect("CancelAllOrdersParameters roundtrip");
        model_support::assert_presence(value, &output, "CancelAllOrdersParameters");
    }
    for value in [
        &fixtures["CancelAllOrdersStatus401"],
        &full["CancelAllOrdersStatus401"],
    ] {
        kalshi_client::typed::validate("CancelAllOrdersStatus401", value)
            .expect("CancelAllOrdersStatus401 fixture");
        let model: CancelAllOrdersStatus401 =
            serde_json::from_value(value.clone()).expect("CancelAllOrdersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelAllOrdersStatus401", &output)
            .expect("CancelAllOrdersStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CancelAllOrdersStatus401");
    }
    for value in [
        &fixtures["CancelAllOrdersStatus429"],
        &full["CancelAllOrdersStatus429"],
    ] {
        kalshi_client::typed::validate("CancelAllOrdersStatus429", value)
            .expect("CancelAllOrdersStatus429 fixture");
        let model: CancelAllOrdersStatus429 =
            serde_json::from_value(value.clone()).expect("CancelAllOrdersStatus429 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelAllOrdersStatus429", &output)
            .expect("CancelAllOrdersStatus429 roundtrip");
        model_support::assert_presence(value, &output, "CancelAllOrdersStatus429");
    }
    for value in [
        &fixtures["CancelAllOrdersStatus500"],
        &full["CancelAllOrdersStatus500"],
    ] {
        kalshi_client::typed::validate("CancelAllOrdersStatus500", value)
            .expect("CancelAllOrdersStatus500 fixture");
        let model: CancelAllOrdersStatus500 =
            serde_json::from_value(value.clone()).expect("CancelAllOrdersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelAllOrdersStatus500", &output)
            .expect("CancelAllOrdersStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CancelAllOrdersStatus500");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Body"],
        &full["BatchCreateOrdersV2Body"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Body", value)
            .expect("BatchCreateOrdersV2Body fixture");
        let model: BatchCreateOrdersV2Body =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Body decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Body", &output)
            .expect("BatchCreateOrdersV2Body roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Body");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Status201"],
        &full["BatchCreateOrdersV2Status201"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Status201", value)
            .expect("BatchCreateOrdersV2Status201 fixture");
        let model: BatchCreateOrdersV2Status201 =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Status201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Status201", &output)
            .expect("BatchCreateOrdersV2Status201 roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Status201");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Status400"],
        &full["BatchCreateOrdersV2Status400"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Status400", value)
            .expect("BatchCreateOrdersV2Status400 fixture");
        let model: BatchCreateOrdersV2Status400 =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Status400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Status400", &output)
            .expect("BatchCreateOrdersV2Status400 roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Status400");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Status401"],
        &full["BatchCreateOrdersV2Status401"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Status401", value)
            .expect("BatchCreateOrdersV2Status401 fixture");
        let model: BatchCreateOrdersV2Status401 =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Status401", &output)
            .expect("BatchCreateOrdersV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Status401");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Status403"],
        &full["BatchCreateOrdersV2Status403"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Status403", value)
            .expect("BatchCreateOrdersV2Status403 fixture");
        let model: BatchCreateOrdersV2Status403 =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Status403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Status403", &output)
            .expect("BatchCreateOrdersV2Status403 roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Status403");
    }
    for value in [
        &fixtures["BatchCreateOrdersV2Status500"],
        &full["BatchCreateOrdersV2Status500"],
    ] {
        kalshi_client::typed::validate("BatchCreateOrdersV2Status500", value)
            .expect("BatchCreateOrdersV2Status500 fixture");
        let model: BatchCreateOrdersV2Status500 =
            serde_json::from_value(value.clone()).expect("BatchCreateOrdersV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCreateOrdersV2Status500", &output)
            .expect("BatchCreateOrdersV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "BatchCreateOrdersV2Status500");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Body"],
        &full["BatchCancelOrdersV2Body"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Body", value)
            .expect("BatchCancelOrdersV2Body fixture");
        let model: BatchCancelOrdersV2Body =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Body decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Body", &output)
            .expect("BatchCancelOrdersV2Body roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Body");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Status200"],
        &full["BatchCancelOrdersV2Status200"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Status200", value)
            .expect("BatchCancelOrdersV2Status200 fixture");
        let model: BatchCancelOrdersV2Status200 =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Status200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Status200", &output)
            .expect("BatchCancelOrdersV2Status200 roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Status200");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Status400"],
        &full["BatchCancelOrdersV2Status400"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Status400", value)
            .expect("BatchCancelOrdersV2Status400 fixture");
        let model: BatchCancelOrdersV2Status400 =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Status400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Status400", &output)
            .expect("BatchCancelOrdersV2Status400 roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Status400");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Status401"],
        &full["BatchCancelOrdersV2Status401"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Status401", value)
            .expect("BatchCancelOrdersV2Status401 fixture");
        let model: BatchCancelOrdersV2Status401 =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Status401", &output)
            .expect("BatchCancelOrdersV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Status401");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Status403"],
        &full["BatchCancelOrdersV2Status403"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Status403", value)
            .expect("BatchCancelOrdersV2Status403 fixture");
        let model: BatchCancelOrdersV2Status403 =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Status403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Status403", &output)
            .expect("BatchCancelOrdersV2Status403 roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Status403");
    }
    for value in [
        &fixtures["BatchCancelOrdersV2Status500"],
        &full["BatchCancelOrdersV2Status500"],
    ] {
        kalshi_client::typed::validate("BatchCancelOrdersV2Status500", value)
            .expect("BatchCancelOrdersV2Status500 fixture");
        let model: BatchCancelOrdersV2Status500 =
            serde_json::from_value(value.clone()).expect("BatchCancelOrdersV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("BatchCancelOrdersV2Status500", &output)
            .expect("BatchCancelOrdersV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "BatchCancelOrdersV2Status500");
    }
    for value in [
        &fixtures["CancelOrderV2Parameters"],
        &full["CancelOrderV2Parameters"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Parameters", value)
            .expect("CancelOrderV2Parameters fixture");
        let model: CancelOrderV2Parameters =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Parameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Parameters", &output)
            .expect("CancelOrderV2Parameters roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Parameters");
    }
    for value in [
        &fixtures["CancelOrderV2Status200"],
        &full["CancelOrderV2Status200"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Status200", value)
            .expect("CancelOrderV2Status200 fixture");
        let model: CancelOrderV2Status200 =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Status200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Status200", &output)
            .expect("CancelOrderV2Status200 roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Status200");
    }
    for value in [
        &fixtures["CancelOrderV2Status401"],
        &full["CancelOrderV2Status401"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Status401", value)
            .expect("CancelOrderV2Status401 fixture");
        let model: CancelOrderV2Status401 =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Status401", &output)
            .expect("CancelOrderV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Status401");
    }
    for value in [
        &fixtures["CancelOrderV2Status404"],
        &full["CancelOrderV2Status404"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Status404", value)
            .expect("CancelOrderV2Status404 fixture");
        let model: CancelOrderV2Status404 =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Status404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Status404", &output)
            .expect("CancelOrderV2Status404 roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Status404");
    }
    for value in [
        &fixtures["CancelOrderV2Status500"],
        &full["CancelOrderV2Status500"],
    ] {
        kalshi_client::typed::validate("CancelOrderV2Status500", value)
            .expect("CancelOrderV2Status500 fixture");
        let model: CancelOrderV2Status500 =
            serde_json::from_value(value.clone()).expect("CancelOrderV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CancelOrderV2Status500", &output)
            .expect("CancelOrderV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "CancelOrderV2Status500");
    }
    for value in [
        &fixtures["AmendOrderV2Parameters"],
        &full["AmendOrderV2Parameters"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Parameters", value)
            .expect("AmendOrderV2Parameters fixture");
        let model: AmendOrderV2Parameters =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Parameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Parameters", &output)
            .expect("AmendOrderV2Parameters roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Parameters");
    }
    for value in [&fixtures["AmendOrderV2Body"], &full["AmendOrderV2Body"]] {
        kalshi_client::typed::validate("AmendOrderV2Body", value)
            .expect("AmendOrderV2Body fixture");
        let model: AmendOrderV2Body =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Body decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Body", &output)
            .expect("AmendOrderV2Body roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Body");
    }
    for value in [
        &fixtures["AmendOrderV2Status200"],
        &full["AmendOrderV2Status200"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status200", value)
            .expect("AmendOrderV2Status200 fixture");
        let model: AmendOrderV2Status200 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status200", &output)
            .expect("AmendOrderV2Status200 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status200");
    }
    for value in [
        &fixtures["AmendOrderV2Status400"],
        &full["AmendOrderV2Status400"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status400", value)
            .expect("AmendOrderV2Status400 fixture");
        let model: AmendOrderV2Status400 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status400", &output)
            .expect("AmendOrderV2Status400 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status400");
    }
    for value in [
        &fixtures["AmendOrderV2Status401"],
        &full["AmendOrderV2Status401"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status401", value)
            .expect("AmendOrderV2Status401 fixture");
        let model: AmendOrderV2Status401 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status401", &output)
            .expect("AmendOrderV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status401");
    }
    for value in [
        &fixtures["AmendOrderV2Status403"],
        &full["AmendOrderV2Status403"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status403", value)
            .expect("AmendOrderV2Status403 fixture");
        let model: AmendOrderV2Status403 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status403", &output)
            .expect("AmendOrderV2Status403 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status403");
    }
    for value in [
        &fixtures["AmendOrderV2Status404"],
        &full["AmendOrderV2Status404"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status404", value)
            .expect("AmendOrderV2Status404 fixture");
        let model: AmendOrderV2Status404 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status404", &output)
            .expect("AmendOrderV2Status404 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status404");
    }
    for value in [
        &fixtures["AmendOrderV2Status500"],
        &full["AmendOrderV2Status500"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status500", value)
            .expect("AmendOrderV2Status500 fixture");
        let model: AmendOrderV2Status500 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status500", &output)
            .expect("AmendOrderV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status500");
    }
    for value in [
        &fixtures["AmendOrderV2Status503"],
        &full["AmendOrderV2Status503"],
    ] {
        kalshi_client::typed::validate("AmendOrderV2Status503", value)
            .expect("AmendOrderV2Status503 fixture");
        let model: AmendOrderV2Status503 =
            serde_json::from_value(value.clone()).expect("AmendOrderV2Status503 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AmendOrderV2Status503", &output)
            .expect("AmendOrderV2Status503 roundtrip");
        model_support::assert_presence(value, &output, "AmendOrderV2Status503");
    }
    for value in [
        &fixtures["DecreaseOrderV2Parameters"],
        &full["DecreaseOrderV2Parameters"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Parameters", value)
            .expect("DecreaseOrderV2Parameters fixture");
        let model: DecreaseOrderV2Parameters =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Parameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Parameters", &output)
            .expect("DecreaseOrderV2Parameters roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Parameters");
    }
    for value in [
        &fixtures["DecreaseOrderV2Body"],
        &full["DecreaseOrderV2Body"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Body", value)
            .expect("DecreaseOrderV2Body fixture");
        let model: DecreaseOrderV2Body =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Body decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Body", &output)
            .expect("DecreaseOrderV2Body roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Body");
    }
    for value in [
        &fixtures["DecreaseOrderV2Status200"],
        &full["DecreaseOrderV2Status200"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Status200", value)
            .expect("DecreaseOrderV2Status200 fixture");
        let model: DecreaseOrderV2Status200 =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Status200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Status200", &output)
            .expect("DecreaseOrderV2Status200 roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Status200");
    }
    for value in [
        &fixtures["DecreaseOrderV2Status400"],
        &full["DecreaseOrderV2Status400"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Status400", value)
            .expect("DecreaseOrderV2Status400 fixture");
        let model: DecreaseOrderV2Status400 =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Status400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Status400", &output)
            .expect("DecreaseOrderV2Status400 roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Status400");
    }
    for value in [
        &fixtures["DecreaseOrderV2Status401"],
        &full["DecreaseOrderV2Status401"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Status401", value)
            .expect("DecreaseOrderV2Status401 fixture");
        let model: DecreaseOrderV2Status401 =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Status401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Status401", &output)
            .expect("DecreaseOrderV2Status401 roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Status401");
    }
    for value in [
        &fixtures["DecreaseOrderV2Status404"],
        &full["DecreaseOrderV2Status404"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Status404", value)
            .expect("DecreaseOrderV2Status404 fixture");
        let model: DecreaseOrderV2Status404 =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Status404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Status404", &output)
            .expect("DecreaseOrderV2Status404 roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Status404");
    }
    for value in [
        &fixtures["DecreaseOrderV2Status500"],
        &full["DecreaseOrderV2Status500"],
    ] {
        kalshi_client::typed::validate("DecreaseOrderV2Status500", value)
            .expect("DecreaseOrderV2Status500 fixture");
        let model: DecreaseOrderV2Status500 =
            serde_json::from_value(value.clone()).expect("DecreaseOrderV2Status500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DecreaseOrderV2Status500", &output)
            .expect("DecreaseOrderV2Status500 roundtrip");
        model_support::assert_presence(value, &output, "DecreaseOrderV2Status500");
    }
    for value in [
        &fixtures["GetOrderGroupsParameters"],
        &full["GetOrderGroupsParameters"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsParameters", value)
            .expect("GetOrderGroupsParameters fixture");
        let model: GetOrderGroupsParameters =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsParameters", &output)
            .expect("GetOrderGroupsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsParameters");
    }
    for value in [
        &fixtures["GetOrderGroupsStatus200"],
        &full["GetOrderGroupsStatus200"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsStatus200", value)
            .expect("GetOrderGroupsStatus200 fixture");
        let model: GetOrderGroupsStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsStatus200", &output)
            .expect("GetOrderGroupsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsStatus200");
    }
    for value in [
        &fixtures["GetOrderGroupsStatus400"],
        &full["GetOrderGroupsStatus400"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsStatus400", value)
            .expect("GetOrderGroupsStatus400 fixture");
        let model: GetOrderGroupsStatus400 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsStatus400", &output)
            .expect("GetOrderGroupsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsStatus400");
    }
    for value in [
        &fixtures["GetOrderGroupsStatus401"],
        &full["GetOrderGroupsStatus401"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsStatus401", value)
            .expect("GetOrderGroupsStatus401 fixture");
        let model: GetOrderGroupsStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsStatus401", &output)
            .expect("GetOrderGroupsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsStatus401");
    }
    for value in [
        &fixtures["GetOrderGroupsStatus500"],
        &full["GetOrderGroupsStatus500"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupsStatus500", value)
            .expect("GetOrderGroupsStatus500 fixture");
        let model: GetOrderGroupsStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupsStatus500", &output)
            .expect("GetOrderGroupsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupsStatus500");
    }
    for value in [
        &fixtures["CreateOrderGroupBody"],
        &full["CreateOrderGroupBody"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupBody", value)
            .expect("CreateOrderGroupBody fixture");
        let model: CreateOrderGroupBody =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupBody", &output)
            .expect("CreateOrderGroupBody roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupBody");
    }
    for value in [
        &fixtures["CreateOrderGroupStatus201"],
        &full["CreateOrderGroupStatus201"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupStatus201", value)
            .expect("CreateOrderGroupStatus201 fixture");
        let model: CreateOrderGroupStatus201 =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupStatus201", &output)
            .expect("CreateOrderGroupStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupStatus201");
    }
    for value in [
        &fixtures["CreateOrderGroupStatus400"],
        &full["CreateOrderGroupStatus400"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupStatus400", value)
            .expect("CreateOrderGroupStatus400 fixture");
        let model: CreateOrderGroupStatus400 =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupStatus400", &output)
            .expect("CreateOrderGroupStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupStatus400");
    }
    for value in [
        &fixtures["CreateOrderGroupStatus401"],
        &full["CreateOrderGroupStatus401"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupStatus401", value)
            .expect("CreateOrderGroupStatus401 fixture");
        let model: CreateOrderGroupStatus401 =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupStatus401", &output)
            .expect("CreateOrderGroupStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupStatus401");
    }
    for value in [
        &fixtures["CreateOrderGroupStatus500"],
        &full["CreateOrderGroupStatus500"],
    ] {
        kalshi_client::typed::validate("CreateOrderGroupStatus500", value)
            .expect("CreateOrderGroupStatus500 fixture");
        let model: CreateOrderGroupStatus500 =
            serde_json::from_value(value.clone()).expect("CreateOrderGroupStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateOrderGroupStatus500", &output)
            .expect("CreateOrderGroupStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateOrderGroupStatus500");
    }
    for value in [
        &fixtures["GetOrderGroupParameters"],
        &full["GetOrderGroupParameters"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupParameters", value)
            .expect("GetOrderGroupParameters fixture");
        let model: GetOrderGroupParameters =
            serde_json::from_value(value.clone()).expect("GetOrderGroupParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupParameters", &output)
            .expect("GetOrderGroupParameters roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupParameters");
    }
    for value in [
        &fixtures["GetOrderGroupStatus200"],
        &full["GetOrderGroupStatus200"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupStatus200", value)
            .expect("GetOrderGroupStatus200 fixture");
        let model: GetOrderGroupStatus200 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupStatus200", &output)
            .expect("GetOrderGroupStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupStatus200");
    }
    for value in [
        &fixtures["GetOrderGroupStatus401"],
        &full["GetOrderGroupStatus401"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupStatus401", value)
            .expect("GetOrderGroupStatus401 fixture");
        let model: GetOrderGroupStatus401 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupStatus401", &output)
            .expect("GetOrderGroupStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupStatus401");
    }
    for value in [
        &fixtures["GetOrderGroupStatus404"],
        &full["GetOrderGroupStatus404"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupStatus404", value)
            .expect("GetOrderGroupStatus404 fixture");
        let model: GetOrderGroupStatus404 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupStatus404", &output)
            .expect("GetOrderGroupStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupStatus404");
    }
    for value in [
        &fixtures["GetOrderGroupStatus500"],
        &full["GetOrderGroupStatus500"],
    ] {
        kalshi_client::typed::validate("GetOrderGroupStatus500", value)
            .expect("GetOrderGroupStatus500 fixture");
        let model: GetOrderGroupStatus500 =
            serde_json::from_value(value.clone()).expect("GetOrderGroupStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetOrderGroupStatus500", &output)
            .expect("GetOrderGroupStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetOrderGroupStatus500");
    }
    for value in [
        &fixtures["DeleteOrderGroupParameters"],
        &full["DeleteOrderGroupParameters"],
    ] {
        kalshi_client::typed::validate("DeleteOrderGroupParameters", value)
            .expect("DeleteOrderGroupParameters fixture");
        let model: DeleteOrderGroupParameters =
            serde_json::from_value(value.clone()).expect("DeleteOrderGroupParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteOrderGroupParameters", &output)
            .expect("DeleteOrderGroupParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteOrderGroupParameters");
    }
    for value in [
        &fixtures["DeleteOrderGroupStatus200"],
        &full["DeleteOrderGroupStatus200"],
    ] {
        kalshi_client::typed::validate("DeleteOrderGroupStatus200", value)
            .expect("DeleteOrderGroupStatus200 fixture");
        let model: DeleteOrderGroupStatus200 =
            serde_json::from_value(value.clone()).expect("DeleteOrderGroupStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteOrderGroupStatus200", &output)
            .expect("DeleteOrderGroupStatus200 roundtrip");
        model_support::assert_presence(value, &output, "DeleteOrderGroupStatus200");
    }
    for value in [
        &fixtures["DeleteOrderGroupStatus401"],
        &full["DeleteOrderGroupStatus401"],
    ] {
        kalshi_client::typed::validate("DeleteOrderGroupStatus401", value)
            .expect("DeleteOrderGroupStatus401 fixture");
        let model: DeleteOrderGroupStatus401 =
            serde_json::from_value(value.clone()).expect("DeleteOrderGroupStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteOrderGroupStatus401", &output)
            .expect("DeleteOrderGroupStatus401 roundtrip");
        model_support::assert_presence(value, &output, "DeleteOrderGroupStatus401");
    }
    for value in [
        &fixtures["DeleteOrderGroupStatus404"],
        &full["DeleteOrderGroupStatus404"],
    ] {
        kalshi_client::typed::validate("DeleteOrderGroupStatus404", value)
            .expect("DeleteOrderGroupStatus404 fixture");
        let model: DeleteOrderGroupStatus404 =
            serde_json::from_value(value.clone()).expect("DeleteOrderGroupStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteOrderGroupStatus404", &output)
            .expect("DeleteOrderGroupStatus404 roundtrip");
        model_support::assert_presence(value, &output, "DeleteOrderGroupStatus404");
    }
    for value in [
        &fixtures["DeleteOrderGroupStatus500"],
        &full["DeleteOrderGroupStatus500"],
    ] {
        kalshi_client::typed::validate("DeleteOrderGroupStatus500", value)
            .expect("DeleteOrderGroupStatus500 fixture");
        let model: DeleteOrderGroupStatus500 =
            serde_json::from_value(value.clone()).expect("DeleteOrderGroupStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteOrderGroupStatus500", &output)
            .expect("DeleteOrderGroupStatus500 roundtrip");
        model_support::assert_presence(value, &output, "DeleteOrderGroupStatus500");
    }
    for value in [
        &fixtures["ResetOrderGroupParameters"],
        &full["ResetOrderGroupParameters"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupParameters", value)
            .expect("ResetOrderGroupParameters fixture");
        let model: ResetOrderGroupParameters =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupParameters", &output)
            .expect("ResetOrderGroupParameters roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupParameters");
    }
    for value in [
        &fixtures["ResetOrderGroupBody"],
        &full["ResetOrderGroupBody"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupBody", value)
            .expect("ResetOrderGroupBody fixture");
        let model: ResetOrderGroupBody =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupBody", &output)
            .expect("ResetOrderGroupBody roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupBody");
    }
    for value in [
        &fixtures["ResetOrderGroupStatus200"],
        &full["ResetOrderGroupStatus200"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupStatus200", value)
            .expect("ResetOrderGroupStatus200 fixture");
        let model: ResetOrderGroupStatus200 =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupStatus200", &output)
            .expect("ResetOrderGroupStatus200 roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupStatus200");
    }
    for value in [
        &fixtures["ResetOrderGroupStatus401"],
        &full["ResetOrderGroupStatus401"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupStatus401", value)
            .expect("ResetOrderGroupStatus401 fixture");
        let model: ResetOrderGroupStatus401 =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupStatus401", &output)
            .expect("ResetOrderGroupStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupStatus401");
    }
    for value in [
        &fixtures["ResetOrderGroupStatus404"],
        &full["ResetOrderGroupStatus404"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupStatus404", value)
            .expect("ResetOrderGroupStatus404 fixture");
        let model: ResetOrderGroupStatus404 =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupStatus404", &output)
            .expect("ResetOrderGroupStatus404 roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupStatus404");
    }
    for value in [
        &fixtures["ResetOrderGroupStatus500"],
        &full["ResetOrderGroupStatus500"],
    ] {
        kalshi_client::typed::validate("ResetOrderGroupStatus500", value)
            .expect("ResetOrderGroupStatus500 fixture");
        let model: ResetOrderGroupStatus500 =
            serde_json::from_value(value.clone()).expect("ResetOrderGroupStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ResetOrderGroupStatus500", &output)
            .expect("ResetOrderGroupStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ResetOrderGroupStatus500");
    }
    for value in [
        &fixtures["TriggerOrderGroupParameters"],
        &full["TriggerOrderGroupParameters"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupParameters", value)
            .expect("TriggerOrderGroupParameters fixture");
        let model: TriggerOrderGroupParameters =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupParameters", &output)
            .expect("TriggerOrderGroupParameters roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupParameters");
    }
    for value in [
        &fixtures["TriggerOrderGroupBody"],
        &full["TriggerOrderGroupBody"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupBody", value)
            .expect("TriggerOrderGroupBody fixture");
        let model: TriggerOrderGroupBody =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupBody", &output)
            .expect("TriggerOrderGroupBody roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupBody");
    }
    for value in [
        &fixtures["TriggerOrderGroupStatus200"],
        &full["TriggerOrderGroupStatus200"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupStatus200", value)
            .expect("TriggerOrderGroupStatus200 fixture");
        let model: TriggerOrderGroupStatus200 =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupStatus200", &output)
            .expect("TriggerOrderGroupStatus200 roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupStatus200");
    }
    for value in [
        &fixtures["TriggerOrderGroupStatus401"],
        &full["TriggerOrderGroupStatus401"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupStatus401", value)
            .expect("TriggerOrderGroupStatus401 fixture");
        let model: TriggerOrderGroupStatus401 =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupStatus401", &output)
            .expect("TriggerOrderGroupStatus401 roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupStatus401");
    }
    for value in [
        &fixtures["TriggerOrderGroupStatus404"],
        &full["TriggerOrderGroupStatus404"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupStatus404", value)
            .expect("TriggerOrderGroupStatus404 fixture");
        let model: TriggerOrderGroupStatus404 =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupStatus404", &output)
            .expect("TriggerOrderGroupStatus404 roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupStatus404");
    }
    for value in [
        &fixtures["TriggerOrderGroupStatus500"],
        &full["TriggerOrderGroupStatus500"],
    ] {
        kalshi_client::typed::validate("TriggerOrderGroupStatus500", value)
            .expect("TriggerOrderGroupStatus500 fixture");
        let model: TriggerOrderGroupStatus500 =
            serde_json::from_value(value.clone()).expect("TriggerOrderGroupStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("TriggerOrderGroupStatus500", &output)
            .expect("TriggerOrderGroupStatus500 roundtrip");
        model_support::assert_presence(value, &output, "TriggerOrderGroupStatus500");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitParameters"],
        &full["UpdateOrderGroupLimitParameters"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitParameters", value)
            .expect("UpdateOrderGroupLimitParameters fixture");
        let model: UpdateOrderGroupLimitParameters =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitParameters", &output)
            .expect("UpdateOrderGroupLimitParameters roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitParameters");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitBody"],
        &full["UpdateOrderGroupLimitBody"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitBody", value)
            .expect("UpdateOrderGroupLimitBody fixture");
        let model: UpdateOrderGroupLimitBody =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitBody", &output)
            .expect("UpdateOrderGroupLimitBody roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitBody");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitStatus200"],
        &full["UpdateOrderGroupLimitStatus200"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus200", value)
            .expect("UpdateOrderGroupLimitStatus200 fixture");
        let model: UpdateOrderGroupLimitStatus200 =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus200", &output)
            .expect("UpdateOrderGroupLimitStatus200 roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitStatus200");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitStatus400"],
        &full["UpdateOrderGroupLimitStatus400"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus400", value)
            .expect("UpdateOrderGroupLimitStatus400 fixture");
        let model: UpdateOrderGroupLimitStatus400 =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus400", &output)
            .expect("UpdateOrderGroupLimitStatus400 roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitStatus400");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitStatus401"],
        &full["UpdateOrderGroupLimitStatus401"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus401", value)
            .expect("UpdateOrderGroupLimitStatus401 fixture");
        let model: UpdateOrderGroupLimitStatus401 =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus401", &output)
            .expect("UpdateOrderGroupLimitStatus401 roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitStatus401");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitStatus404"],
        &full["UpdateOrderGroupLimitStatus404"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus404", value)
            .expect("UpdateOrderGroupLimitStatus404 fixture");
        let model: UpdateOrderGroupLimitStatus404 =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus404", &output)
            .expect("UpdateOrderGroupLimitStatus404 roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitStatus404");
    }
    for value in [
        &fixtures["UpdateOrderGroupLimitStatus500"],
        &full["UpdateOrderGroupLimitStatus500"],
    ] {
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus500", value)
            .expect("UpdateOrderGroupLimitStatus500 fixture");
        let model: UpdateOrderGroupLimitStatus500 =
            serde_json::from_value(value.clone()).expect("UpdateOrderGroupLimitStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateOrderGroupLimitStatus500", &output)
            .expect("UpdateOrderGroupLimitStatus500 roundtrip");
        model_support::assert_presence(value, &output, "UpdateOrderGroupLimitStatus500");
    }
    for value in [
        &fixtures["GetBalanceParameters"],
        &full["GetBalanceParameters"],
    ] {
        kalshi_client::typed::validate("GetBalanceParameters", value)
            .expect("GetBalanceParameters fixture");
        let model: GetBalanceParameters =
            serde_json::from_value(value.clone()).expect("GetBalanceParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBalanceParameters", &output)
            .expect("GetBalanceParameters roundtrip");
        model_support::assert_presence(value, &output, "GetBalanceParameters");
    }
    for value in [
        &fixtures["GetBalanceStatus200"],
        &full["GetBalanceStatus200"],
    ] {
        kalshi_client::typed::validate("GetBalanceStatus200", value)
            .expect("GetBalanceStatus200 fixture");
        let model: GetBalanceStatus200 =
            serde_json::from_value(value.clone()).expect("GetBalanceStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBalanceStatus200", &output)
            .expect("GetBalanceStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetBalanceStatus200");
    }
    for value in [
        &fixtures["GetBalanceStatus401"],
        &full["GetBalanceStatus401"],
    ] {
        kalshi_client::typed::validate("GetBalanceStatus401", value)
            .expect("GetBalanceStatus401 fixture");
        let model: GetBalanceStatus401 =
            serde_json::from_value(value.clone()).expect("GetBalanceStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBalanceStatus401", &output)
            .expect("GetBalanceStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetBalanceStatus401");
    }
    for value in [
        &fixtures["GetBalanceStatus500"],
        &full["GetBalanceStatus500"],
    ] {
        kalshi_client::typed::validate("GetBalanceStatus500", value)
            .expect("GetBalanceStatus500 fixture");
        let model: GetBalanceStatus500 =
            serde_json::from_value(value.clone()).expect("GetBalanceStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBalanceStatus500", &output)
            .expect("GetBalanceStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetBalanceStatus500");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferBody"],
        &full["IntraExchangeInstanceTransferBody"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferBody", value)
            .expect("IntraExchangeInstanceTransferBody fixture");
        let model: IntraExchangeInstanceTransferBody = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferBody", &output)
            .expect("IntraExchangeInstanceTransferBody roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferBody");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus200"],
        &full["IntraExchangeInstanceTransferStatus200"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus200", value)
            .expect("IntraExchangeInstanceTransferStatus200 fixture");
        let model: IntraExchangeInstanceTransferStatus200 = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus200", &output)
            .expect("IntraExchangeInstanceTransferStatus200 roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus200");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus400"],
        &full["IntraExchangeInstanceTransferStatus400"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus400", value)
            .expect("IntraExchangeInstanceTransferStatus400 fixture");
        let model: IntraExchangeInstanceTransferStatus400 = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus400", &output)
            .expect("IntraExchangeInstanceTransferStatus400 roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus400");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus401"],
        &full["IntraExchangeInstanceTransferStatus401"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus401", value)
            .expect("IntraExchangeInstanceTransferStatus401 fixture");
        let model: IntraExchangeInstanceTransferStatus401 = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus401", &output)
            .expect("IntraExchangeInstanceTransferStatus401 roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus401");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus403"],
        &full["IntraExchangeInstanceTransferStatus403"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus403", value)
            .expect("IntraExchangeInstanceTransferStatus403 fixture");
        let model: IntraExchangeInstanceTransferStatus403 = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus403", &output)
            .expect("IntraExchangeInstanceTransferStatus403 roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus403");
    }
    for value in [
        &fixtures["IntraExchangeInstanceTransferStatus500"],
        &full["IntraExchangeInstanceTransferStatus500"],
    ] {
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus500", value)
            .expect("IntraExchangeInstanceTransferStatus500 fixture");
        let model: IntraExchangeInstanceTransferStatus500 = serde_json::from_value(value.clone())
            .expect("IntraExchangeInstanceTransferStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("IntraExchangeInstanceTransferStatus500", &output)
            .expect("IntraExchangeInstanceTransferStatus500 roundtrip");
        model_support::assert_presence(value, &output, "IntraExchangeInstanceTransferStatus500");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersParameters"],
        &full["GetIntraExchangeInstanceTransfersParameters"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersParameters", value)
            .expect("GetIntraExchangeInstanceTransfersParameters fixture");
        let model: GetIntraExchangeInstanceTransfersParameters =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersParameters", &output)
            .expect("GetIntraExchangeInstanceTransfersParameters roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransfersParameters",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersStatus200"],
        &full["GetIntraExchangeInstanceTransfersStatus200"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus200", value)
            .expect("GetIntraExchangeInstanceTransfersStatus200 fixture");
        let model: GetIntraExchangeInstanceTransfersStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus200", &output)
            .expect("GetIntraExchangeInstanceTransfersStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransfersStatus200",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersStatus400"],
        &full["GetIntraExchangeInstanceTransfersStatus400"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus400", value)
            .expect("GetIntraExchangeInstanceTransfersStatus400 fixture");
        let model: GetIntraExchangeInstanceTransfersStatus400 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus400", &output)
            .expect("GetIntraExchangeInstanceTransfersStatus400 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransfersStatus400",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersStatus401"],
        &full["GetIntraExchangeInstanceTransfersStatus401"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus401", value)
            .expect("GetIntraExchangeInstanceTransfersStatus401 fixture");
        let model: GetIntraExchangeInstanceTransfersStatus401 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus401", &output)
            .expect("GetIntraExchangeInstanceTransfersStatus401 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransfersStatus401",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransfersStatus500"],
        &full["GetIntraExchangeInstanceTransfersStatus500"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus500", value)
            .expect("GetIntraExchangeInstanceTransfersStatus500 fixture");
        let model: GetIntraExchangeInstanceTransfersStatus500 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransfersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransfersStatus500", &output)
            .expect("GetIntraExchangeInstanceTransfersStatus500 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransfersStatus500",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferParameters"],
        &full["GetIntraExchangeInstanceTransferParameters"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferParameters", value)
            .expect("GetIntraExchangeInstanceTransferParameters fixture");
        let model: GetIntraExchangeInstanceTransferParameters =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransferParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferParameters", &output)
            .expect("GetIntraExchangeInstanceTransferParameters roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetIntraExchangeInstanceTransferParameters",
        );
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferStatus200"],
        &full["GetIntraExchangeInstanceTransferStatus200"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus200", value)
            .expect("GetIntraExchangeInstanceTransferStatus200 fixture");
        let model: GetIntraExchangeInstanceTransferStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransferStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus200", &output)
            .expect("GetIntraExchangeInstanceTransferStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransferStatus200");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferStatus401"],
        &full["GetIntraExchangeInstanceTransferStatus401"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus401", value)
            .expect("GetIntraExchangeInstanceTransferStatus401 fixture");
        let model: GetIntraExchangeInstanceTransferStatus401 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransferStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus401", &output)
            .expect("GetIntraExchangeInstanceTransferStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransferStatus401");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferStatus404"],
        &full["GetIntraExchangeInstanceTransferStatus404"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus404", value)
            .expect("GetIntraExchangeInstanceTransferStatus404 fixture");
        let model: GetIntraExchangeInstanceTransferStatus404 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransferStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus404", &output)
            .expect("GetIntraExchangeInstanceTransferStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransferStatus404");
    }
    for value in [
        &fixtures["GetIntraExchangeInstanceTransferStatus500"],
        &full["GetIntraExchangeInstanceTransferStatus500"],
    ] {
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus500", value)
            .expect("GetIntraExchangeInstanceTransferStatus500 fixture");
        let model: GetIntraExchangeInstanceTransferStatus500 =
            serde_json::from_value(value.clone())
                .expect("GetIntraExchangeInstanceTransferStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIntraExchangeInstanceTransferStatus500", &output)
            .expect("GetIntraExchangeInstanceTransferStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetIntraExchangeInstanceTransferStatus500");
    }
    for value in [
        &fixtures["CreateSubaccountBody"],
        &full["CreateSubaccountBody"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountBody", value)
            .expect("CreateSubaccountBody fixture");
        let model: CreateSubaccountBody =
            serde_json::from_value(value.clone()).expect("CreateSubaccountBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountBody", &output)
            .expect("CreateSubaccountBody roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountBody");
    }
    for value in [
        &fixtures["CreateSubaccountStatus201"],
        &full["CreateSubaccountStatus201"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountStatus201", value)
            .expect("CreateSubaccountStatus201 fixture");
        let model: CreateSubaccountStatus201 =
            serde_json::from_value(value.clone()).expect("CreateSubaccountStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountStatus201", &output)
            .expect("CreateSubaccountStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountStatus201");
    }
    for value in [
        &fixtures["CreateSubaccountStatus400"],
        &full["CreateSubaccountStatus400"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountStatus400", value)
            .expect("CreateSubaccountStatus400 fixture");
        let model: CreateSubaccountStatus400 =
            serde_json::from_value(value.clone()).expect("CreateSubaccountStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountStatus400", &output)
            .expect("CreateSubaccountStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountStatus400");
    }
    for value in [
        &fixtures["CreateSubaccountStatus401"],
        &full["CreateSubaccountStatus401"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountStatus401", value)
            .expect("CreateSubaccountStatus401 fixture");
        let model: CreateSubaccountStatus401 =
            serde_json::from_value(value.clone()).expect("CreateSubaccountStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountStatus401", &output)
            .expect("CreateSubaccountStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountStatus401");
    }
    for value in [
        &fixtures["CreateSubaccountStatus500"],
        &full["CreateSubaccountStatus500"],
    ] {
        kalshi_client::typed::validate("CreateSubaccountStatus500", value)
            .expect("CreateSubaccountStatus500 fixture");
        let model: CreateSubaccountStatus500 =
            serde_json::from_value(value.clone()).expect("CreateSubaccountStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateSubaccountStatus500", &output)
            .expect("CreateSubaccountStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateSubaccountStatus500");
    }
    for value in [
        &fixtures["ApplySubaccountTransferBody"],
        &full["ApplySubaccountTransferBody"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferBody", value)
            .expect("ApplySubaccountTransferBody fixture");
        let model: ApplySubaccountTransferBody =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferBody", &output)
            .expect("ApplySubaccountTransferBody roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferBody");
    }
    for value in [
        &fixtures["ApplySubaccountTransferStatus200"],
        &full["ApplySubaccountTransferStatus200"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferStatus200", value)
            .expect("ApplySubaccountTransferStatus200 fixture");
        let model: ApplySubaccountTransferStatus200 =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferStatus200", &output)
            .expect("ApplySubaccountTransferStatus200 roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferStatus200");
    }
    for value in [
        &fixtures["ApplySubaccountTransferStatus400"],
        &full["ApplySubaccountTransferStatus400"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferStatus400", value)
            .expect("ApplySubaccountTransferStatus400 fixture");
        let model: ApplySubaccountTransferStatus400 =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferStatus400", &output)
            .expect("ApplySubaccountTransferStatus400 roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferStatus400");
    }
    for value in [
        &fixtures["ApplySubaccountTransferStatus401"],
        &full["ApplySubaccountTransferStatus401"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferStatus401", value)
            .expect("ApplySubaccountTransferStatus401 fixture");
        let model: ApplySubaccountTransferStatus401 =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferStatus401", &output)
            .expect("ApplySubaccountTransferStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferStatus401");
    }
    for value in [
        &fixtures["ApplySubaccountTransferStatus500"],
        &full["ApplySubaccountTransferStatus500"],
    ] {
        kalshi_client::typed::validate("ApplySubaccountTransferStatus500", value)
            .expect("ApplySubaccountTransferStatus500 fixture");
        let model: ApplySubaccountTransferStatus500 =
            serde_json::from_value(value.clone()).expect("ApplySubaccountTransferStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ApplySubaccountTransferStatus500", &output)
            .expect("ApplySubaccountTransferStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ApplySubaccountTransferStatus500");
    }
    for value in [
        &fixtures["GetSubaccountBalancesStatus200"],
        &full["GetSubaccountBalancesStatus200"],
    ] {
        kalshi_client::typed::validate("GetSubaccountBalancesStatus200", value)
            .expect("GetSubaccountBalancesStatus200 fixture");
        let model: GetSubaccountBalancesStatus200 =
            serde_json::from_value(value.clone()).expect("GetSubaccountBalancesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountBalancesStatus200", &output)
            .expect("GetSubaccountBalancesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountBalancesStatus200");
    }
    for value in [
        &fixtures["GetSubaccountBalancesStatus401"],
        &full["GetSubaccountBalancesStatus401"],
    ] {
        kalshi_client::typed::validate("GetSubaccountBalancesStatus401", value)
            .expect("GetSubaccountBalancesStatus401 fixture");
        let model: GetSubaccountBalancesStatus401 =
            serde_json::from_value(value.clone()).expect("GetSubaccountBalancesStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountBalancesStatus401", &output)
            .expect("GetSubaccountBalancesStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountBalancesStatus401");
    }
    for value in [
        &fixtures["GetSubaccountBalancesStatus500"],
        &full["GetSubaccountBalancesStatus500"],
    ] {
        kalshi_client::typed::validate("GetSubaccountBalancesStatus500", value)
            .expect("GetSubaccountBalancesStatus500 fixture");
        let model: GetSubaccountBalancesStatus500 =
            serde_json::from_value(value.clone()).expect("GetSubaccountBalancesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountBalancesStatus500", &output)
            .expect("GetSubaccountBalancesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountBalancesStatus500");
    }
    for value in [
        &fixtures["GetSubaccountTransfersParameters"],
        &full["GetSubaccountTransfersParameters"],
    ] {
        kalshi_client::typed::validate("GetSubaccountTransfersParameters", value)
            .expect("GetSubaccountTransfersParameters fixture");
        let model: GetSubaccountTransfersParameters =
            serde_json::from_value(value.clone()).expect("GetSubaccountTransfersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountTransfersParameters", &output)
            .expect("GetSubaccountTransfersParameters roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountTransfersParameters");
    }
    for value in [
        &fixtures["GetSubaccountTransfersStatus200"],
        &full["GetSubaccountTransfersStatus200"],
    ] {
        kalshi_client::typed::validate("GetSubaccountTransfersStatus200", value)
            .expect("GetSubaccountTransfersStatus200 fixture");
        let model: GetSubaccountTransfersStatus200 =
            serde_json::from_value(value.clone()).expect("GetSubaccountTransfersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountTransfersStatus200", &output)
            .expect("GetSubaccountTransfersStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountTransfersStatus200");
    }
    for value in [
        &fixtures["GetSubaccountTransfersStatus401"],
        &full["GetSubaccountTransfersStatus401"],
    ] {
        kalshi_client::typed::validate("GetSubaccountTransfersStatus401", value)
            .expect("GetSubaccountTransfersStatus401 fixture");
        let model: GetSubaccountTransfersStatus401 =
            serde_json::from_value(value.clone()).expect("GetSubaccountTransfersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountTransfersStatus401", &output)
            .expect("GetSubaccountTransfersStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountTransfersStatus401");
    }
    for value in [
        &fixtures["GetSubaccountTransfersStatus500"],
        &full["GetSubaccountTransfersStatus500"],
    ] {
        kalshi_client::typed::validate("GetSubaccountTransfersStatus500", value)
            .expect("GetSubaccountTransfersStatus500 fixture");
        let model: GetSubaccountTransfersStatus500 =
            serde_json::from_value(value.clone()).expect("GetSubaccountTransfersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountTransfersStatus500", &output)
            .expect("GetSubaccountTransfersStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountTransfersStatus500");
    }
    for value in [
        &fixtures["GetSubaccountNettingStatus200"],
        &full["GetSubaccountNettingStatus200"],
    ] {
        kalshi_client::typed::validate("GetSubaccountNettingStatus200", value)
            .expect("GetSubaccountNettingStatus200 fixture");
        let model: GetSubaccountNettingStatus200 =
            serde_json::from_value(value.clone()).expect("GetSubaccountNettingStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountNettingStatus200", &output)
            .expect("GetSubaccountNettingStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountNettingStatus200");
    }
    for value in [
        &fixtures["GetSubaccountNettingStatus401"],
        &full["GetSubaccountNettingStatus401"],
    ] {
        kalshi_client::typed::validate("GetSubaccountNettingStatus401", value)
            .expect("GetSubaccountNettingStatus401 fixture");
        let model: GetSubaccountNettingStatus401 =
            serde_json::from_value(value.clone()).expect("GetSubaccountNettingStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountNettingStatus401", &output)
            .expect("GetSubaccountNettingStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountNettingStatus401");
    }
    for value in [
        &fixtures["GetSubaccountNettingStatus500"],
        &full["GetSubaccountNettingStatus500"],
    ] {
        kalshi_client::typed::validate("GetSubaccountNettingStatus500", value)
            .expect("GetSubaccountNettingStatus500 fixture");
        let model: GetSubaccountNettingStatus500 =
            serde_json::from_value(value.clone()).expect("GetSubaccountNettingStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSubaccountNettingStatus500", &output)
            .expect("GetSubaccountNettingStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSubaccountNettingStatus500");
    }
    for value in [
        &fixtures["UpdateSubaccountNettingBody"],
        &full["UpdateSubaccountNettingBody"],
    ] {
        kalshi_client::typed::validate("UpdateSubaccountNettingBody", value)
            .expect("UpdateSubaccountNettingBody fixture");
        let model: UpdateSubaccountNettingBody =
            serde_json::from_value(value.clone()).expect("UpdateSubaccountNettingBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateSubaccountNettingBody", &output)
            .expect("UpdateSubaccountNettingBody roundtrip");
        model_support::assert_presence(value, &output, "UpdateSubaccountNettingBody");
    }
    for value in [
        &fixtures["UpdateSubaccountNettingStatus400"],
        &full["UpdateSubaccountNettingStatus400"],
    ] {
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus400", value)
            .expect("UpdateSubaccountNettingStatus400 fixture");
        let model: UpdateSubaccountNettingStatus400 =
            serde_json::from_value(value.clone()).expect("UpdateSubaccountNettingStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus400", &output)
            .expect("UpdateSubaccountNettingStatus400 roundtrip");
        model_support::assert_presence(value, &output, "UpdateSubaccountNettingStatus400");
    }
    for value in [
        &fixtures["UpdateSubaccountNettingStatus401"],
        &full["UpdateSubaccountNettingStatus401"],
    ] {
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus401", value)
            .expect("UpdateSubaccountNettingStatus401 fixture");
        let model: UpdateSubaccountNettingStatus401 =
            serde_json::from_value(value.clone()).expect("UpdateSubaccountNettingStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus401", &output)
            .expect("UpdateSubaccountNettingStatus401 roundtrip");
        model_support::assert_presence(value, &output, "UpdateSubaccountNettingStatus401");
    }
    for value in [
        &fixtures["UpdateSubaccountNettingStatus500"],
        &full["UpdateSubaccountNettingStatus500"],
    ] {
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus500", value)
            .expect("UpdateSubaccountNettingStatus500 fixture");
        let model: UpdateSubaccountNettingStatus500 =
            serde_json::from_value(value.clone()).expect("UpdateSubaccountNettingStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateSubaccountNettingStatus500", &output)
            .expect("UpdateSubaccountNettingStatus500 roundtrip");
        model_support::assert_presence(value, &output, "UpdateSubaccountNettingStatus500");
    }
    for value in [
        &fixtures["GetPositionsParameters"],
        &full["GetPositionsParameters"],
    ] {
        kalshi_client::typed::validate("GetPositionsParameters", value)
            .expect("GetPositionsParameters fixture");
        let model: GetPositionsParameters =
            serde_json::from_value(value.clone()).expect("GetPositionsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsParameters", &output)
            .expect("GetPositionsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsParameters");
    }
    for value in [
        &fixtures["GetPositionsStatus200"],
        &full["GetPositionsStatus200"],
    ] {
        kalshi_client::typed::validate("GetPositionsStatus200", value)
            .expect("GetPositionsStatus200 fixture");
        let model: GetPositionsStatus200 =
            serde_json::from_value(value.clone()).expect("GetPositionsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsStatus200", &output)
            .expect("GetPositionsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsStatus200");
    }
    for value in [
        &fixtures["GetPositionsStatus400"],
        &full["GetPositionsStatus400"],
    ] {
        kalshi_client::typed::validate("GetPositionsStatus400", value)
            .expect("GetPositionsStatus400 fixture");
        let model: GetPositionsStatus400 =
            serde_json::from_value(value.clone()).expect("GetPositionsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsStatus400", &output)
            .expect("GetPositionsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsStatus400");
    }
    for value in [
        &fixtures["GetPositionsStatus401"],
        &full["GetPositionsStatus401"],
    ] {
        kalshi_client::typed::validate("GetPositionsStatus401", value)
            .expect("GetPositionsStatus401 fixture");
        let model: GetPositionsStatus401 =
            serde_json::from_value(value.clone()).expect("GetPositionsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsStatus401", &output)
            .expect("GetPositionsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsStatus401");
    }
    for value in [
        &fixtures["GetPositionsStatus500"],
        &full["GetPositionsStatus500"],
    ] {
        kalshi_client::typed::validate("GetPositionsStatus500", value)
            .expect("GetPositionsStatus500 fixture");
        let model: GetPositionsStatus500 =
            serde_json::from_value(value.clone()).expect("GetPositionsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPositionsStatus500", &output)
            .expect("GetPositionsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetPositionsStatus500");
    }
    for value in [
        &fixtures["GetSettlementsParameters"],
        &full["GetSettlementsParameters"],
    ] {
        kalshi_client::typed::validate("GetSettlementsParameters", value)
            .expect("GetSettlementsParameters fixture");
        let model: GetSettlementsParameters =
            serde_json::from_value(value.clone()).expect("GetSettlementsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsParameters", &output)
            .expect("GetSettlementsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsParameters");
    }
    for value in [
        &fixtures["GetSettlementsStatus200"],
        &full["GetSettlementsStatus200"],
    ] {
        kalshi_client::typed::validate("GetSettlementsStatus200", value)
            .expect("GetSettlementsStatus200 fixture");
        let model: GetSettlementsStatus200 =
            serde_json::from_value(value.clone()).expect("GetSettlementsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsStatus200", &output)
            .expect("GetSettlementsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsStatus200");
    }
    for value in [
        &fixtures["GetSettlementsStatus400"],
        &full["GetSettlementsStatus400"],
    ] {
        kalshi_client::typed::validate("GetSettlementsStatus400", value)
            .expect("GetSettlementsStatus400 fixture");
        let model: GetSettlementsStatus400 =
            serde_json::from_value(value.clone()).expect("GetSettlementsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsStatus400", &output)
            .expect("GetSettlementsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsStatus400");
    }
    for value in [
        &fixtures["GetSettlementsStatus401"],
        &full["GetSettlementsStatus401"],
    ] {
        kalshi_client::typed::validate("GetSettlementsStatus401", value)
            .expect("GetSettlementsStatus401 fixture");
        let model: GetSettlementsStatus401 =
            serde_json::from_value(value.clone()).expect("GetSettlementsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsStatus401", &output)
            .expect("GetSettlementsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsStatus401");
    }
    for value in [
        &fixtures["GetSettlementsStatus500"],
        &full["GetSettlementsStatus500"],
    ] {
        kalshi_client::typed::validate("GetSettlementsStatus500", value)
            .expect("GetSettlementsStatus500 fixture");
        let model: GetSettlementsStatus500 =
            serde_json::from_value(value.clone()).expect("GetSettlementsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetSettlementsStatus500", &output)
            .expect("GetSettlementsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetSettlementsStatus500");
    }
    for value in [
        &fixtures["GetDepositsParameters"],
        &full["GetDepositsParameters"],
    ] {
        kalshi_client::typed::validate("GetDepositsParameters", value)
            .expect("GetDepositsParameters fixture");
        let model: GetDepositsParameters =
            serde_json::from_value(value.clone()).expect("GetDepositsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsParameters", &output)
            .expect("GetDepositsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsParameters");
    }
    for value in [
        &fixtures["GetDepositsStatus200"],
        &full["GetDepositsStatus200"],
    ] {
        kalshi_client::typed::validate("GetDepositsStatus200", value)
            .expect("GetDepositsStatus200 fixture");
        let model: GetDepositsStatus200 =
            serde_json::from_value(value.clone()).expect("GetDepositsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsStatus200", &output)
            .expect("GetDepositsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsStatus200");
    }
    for value in [
        &fixtures["GetDepositsStatus400"],
        &full["GetDepositsStatus400"],
    ] {
        kalshi_client::typed::validate("GetDepositsStatus400", value)
            .expect("GetDepositsStatus400 fixture");
        let model: GetDepositsStatus400 =
            serde_json::from_value(value.clone()).expect("GetDepositsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsStatus400", &output)
            .expect("GetDepositsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsStatus400");
    }
    for value in [
        &fixtures["GetDepositsStatus401"],
        &full["GetDepositsStatus401"],
    ] {
        kalshi_client::typed::validate("GetDepositsStatus401", value)
            .expect("GetDepositsStatus401 fixture");
        let model: GetDepositsStatus401 =
            serde_json::from_value(value.clone()).expect("GetDepositsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsStatus401", &output)
            .expect("GetDepositsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsStatus401");
    }
    for value in [
        &fixtures["GetDepositsStatus500"],
        &full["GetDepositsStatus500"],
    ] {
        kalshi_client::typed::validate("GetDepositsStatus500", value)
            .expect("GetDepositsStatus500 fixture");
        let model: GetDepositsStatus500 =
            serde_json::from_value(value.clone()).expect("GetDepositsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetDepositsStatus500", &output)
            .expect("GetDepositsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetDepositsStatus500");
    }
    for value in [
        &fixtures["GetWithdrawalsParameters"],
        &full["GetWithdrawalsParameters"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsParameters", value)
            .expect("GetWithdrawalsParameters fixture");
        let model: GetWithdrawalsParameters =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsParameters", &output)
            .expect("GetWithdrawalsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsParameters");
    }
    for value in [
        &fixtures["GetWithdrawalsStatus200"],
        &full["GetWithdrawalsStatus200"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsStatus200", value)
            .expect("GetWithdrawalsStatus200 fixture");
        let model: GetWithdrawalsStatus200 =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsStatus200", &output)
            .expect("GetWithdrawalsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsStatus200");
    }
    for value in [
        &fixtures["GetWithdrawalsStatus400"],
        &full["GetWithdrawalsStatus400"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsStatus400", value)
            .expect("GetWithdrawalsStatus400 fixture");
        let model: GetWithdrawalsStatus400 =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsStatus400", &output)
            .expect("GetWithdrawalsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsStatus400");
    }
    for value in [
        &fixtures["GetWithdrawalsStatus401"],
        &full["GetWithdrawalsStatus401"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsStatus401", value)
            .expect("GetWithdrawalsStatus401 fixture");
        let model: GetWithdrawalsStatus401 =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsStatus401", &output)
            .expect("GetWithdrawalsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsStatus401");
    }
    for value in [
        &fixtures["GetWithdrawalsStatus500"],
        &full["GetWithdrawalsStatus500"],
    ] {
        kalshi_client::typed::validate("GetWithdrawalsStatus500", value)
            .expect("GetWithdrawalsStatus500 fixture");
        let model: GetWithdrawalsStatus500 =
            serde_json::from_value(value.clone()).expect("GetWithdrawalsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWithdrawalsStatus500", &output)
            .expect("GetWithdrawalsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetWithdrawalsStatus500");
    }
    for value in [
        &fixtures["GetPortfolioRestingOrderTotalValueStatus200"],
        &full["GetPortfolioRestingOrderTotalValueStatus200"],
    ] {
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus200", value)
            .expect("GetPortfolioRestingOrderTotalValueStatus200 fixture");
        let model: GetPortfolioRestingOrderTotalValueStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetPortfolioRestingOrderTotalValueStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus200", &output)
            .expect("GetPortfolioRestingOrderTotalValueStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetPortfolioRestingOrderTotalValueStatus200",
        );
    }
    for value in [
        &fixtures["GetPortfolioRestingOrderTotalValueStatus401"],
        &full["GetPortfolioRestingOrderTotalValueStatus401"],
    ] {
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus401", value)
            .expect("GetPortfolioRestingOrderTotalValueStatus401 fixture");
        let model: GetPortfolioRestingOrderTotalValueStatus401 =
            serde_json::from_value(value.clone())
                .expect("GetPortfolioRestingOrderTotalValueStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus401", &output)
            .expect("GetPortfolioRestingOrderTotalValueStatus401 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetPortfolioRestingOrderTotalValueStatus401",
        );
    }
    for value in [
        &fixtures["GetPortfolioRestingOrderTotalValueStatus500"],
        &full["GetPortfolioRestingOrderTotalValueStatus500"],
    ] {
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus500", value)
            .expect("GetPortfolioRestingOrderTotalValueStatus500 fixture");
        let model: GetPortfolioRestingOrderTotalValueStatus500 =
            serde_json::from_value(value.clone())
                .expect("GetPortfolioRestingOrderTotalValueStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetPortfolioRestingOrderTotalValueStatus500", &output)
            .expect("GetPortfolioRestingOrderTotalValueStatus500 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetPortfolioRestingOrderTotalValueStatus500",
        );
    }
    for value in [&fixtures["GetFillsParameters"], &full["GetFillsParameters"]] {
        kalshi_client::typed::validate("GetFillsParameters", value)
            .expect("GetFillsParameters fixture");
        let model: GetFillsParameters =
            serde_json::from_value(value.clone()).expect("GetFillsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsParameters", &output)
            .expect("GetFillsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFillsParameters");
    }
    for value in [&fixtures["GetFillsStatus200"], &full["GetFillsStatus200"]] {
        kalshi_client::typed::validate("GetFillsStatus200", value)
            .expect("GetFillsStatus200 fixture");
        let model: GetFillsStatus200 =
            serde_json::from_value(value.clone()).expect("GetFillsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsStatus200", &output)
            .expect("GetFillsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFillsStatus200");
    }
    for value in [
        &fixtures["GetCommunicationsIdStatus200"],
        &full["GetCommunicationsIdStatus200"],
    ] {
        kalshi_client::typed::validate("GetCommunicationsIdStatus200", value)
            .expect("GetCommunicationsIdStatus200 fixture");
        let model: GetCommunicationsIdStatus200 =
            serde_json::from_value(value.clone()).expect("GetCommunicationsIdStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetCommunicationsIdStatus200", &output)
            .expect("GetCommunicationsIdStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetCommunicationsIdStatus200");
    }
    for value in [
        &fixtures["GetCommunicationsIdStatus401"],
        &full["GetCommunicationsIdStatus401"],
    ] {
        kalshi_client::typed::validate("GetCommunicationsIdStatus401", value)
            .expect("GetCommunicationsIdStatus401 fixture");
        let model: GetCommunicationsIdStatus401 =
            serde_json::from_value(value.clone()).expect("GetCommunicationsIdStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetCommunicationsIdStatus401", &output)
            .expect("GetCommunicationsIdStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetCommunicationsIdStatus401");
    }
    for value in [
        &fixtures["GetCommunicationsIdStatus500"],
        &full["GetCommunicationsIdStatus500"],
    ] {
        kalshi_client::typed::validate("GetCommunicationsIdStatus500", value)
            .expect("GetCommunicationsIdStatus500 fixture");
        let model: GetCommunicationsIdStatus500 =
            serde_json::from_value(value.clone()).expect("GetCommunicationsIdStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetCommunicationsIdStatus500", &output)
            .expect("GetCommunicationsIdStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetCommunicationsIdStatus500");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsParameters"],
        &full["GetBlockTradeProposalsParameters"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsParameters", value)
            .expect("GetBlockTradeProposalsParameters fixture");
        let model: GetBlockTradeProposalsParameters =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsParameters", &output)
            .expect("GetBlockTradeProposalsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsParameters");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsStatus200"],
        &full["GetBlockTradeProposalsStatus200"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus200", value)
            .expect("GetBlockTradeProposalsStatus200 fixture");
        let model: GetBlockTradeProposalsStatus200 =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus200", &output)
            .expect("GetBlockTradeProposalsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsStatus200");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsStatus400"],
        &full["GetBlockTradeProposalsStatus400"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus400", value)
            .expect("GetBlockTradeProposalsStatus400 fixture");
        let model: GetBlockTradeProposalsStatus400 =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus400", &output)
            .expect("GetBlockTradeProposalsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsStatus400");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsStatus401"],
        &full["GetBlockTradeProposalsStatus401"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus401", value)
            .expect("GetBlockTradeProposalsStatus401 fixture");
        let model: GetBlockTradeProposalsStatus401 =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus401", &output)
            .expect("GetBlockTradeProposalsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsStatus401");
    }
    for value in [
        &fixtures["GetBlockTradeProposalsStatus500"],
        &full["GetBlockTradeProposalsStatus500"],
    ] {
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus500", value)
            .expect("GetBlockTradeProposalsStatus500 fixture");
        let model: GetBlockTradeProposalsStatus500 =
            serde_json::from_value(value.clone()).expect("GetBlockTradeProposalsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetBlockTradeProposalsStatus500", &output)
            .expect("GetBlockTradeProposalsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetBlockTradeProposalsStatus500");
    }
    for value in [
        &fixtures["ProposeBlockTradeBody"],
        &full["ProposeBlockTradeBody"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeBody", value)
            .expect("ProposeBlockTradeBody fixture");
        let model: ProposeBlockTradeBody =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeBody", &output)
            .expect("ProposeBlockTradeBody roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeBody");
    }
    for value in [
        &fixtures["ProposeBlockTradeStatus201"],
        &full["ProposeBlockTradeStatus201"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeStatus201", value)
            .expect("ProposeBlockTradeStatus201 fixture");
        let model: ProposeBlockTradeStatus201 =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeStatus201", &output)
            .expect("ProposeBlockTradeStatus201 roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeStatus201");
    }
    for value in [
        &fixtures["ProposeBlockTradeStatus400"],
        &full["ProposeBlockTradeStatus400"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeStatus400", value)
            .expect("ProposeBlockTradeStatus400 fixture");
        let model: ProposeBlockTradeStatus400 =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeStatus400", &output)
            .expect("ProposeBlockTradeStatus400 roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeStatus400");
    }
    for value in [
        &fixtures["ProposeBlockTradeStatus401"],
        &full["ProposeBlockTradeStatus401"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeStatus401", value)
            .expect("ProposeBlockTradeStatus401 fixture");
        let model: ProposeBlockTradeStatus401 =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeStatus401", &output)
            .expect("ProposeBlockTradeStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeStatus401");
    }
    for value in [
        &fixtures["ProposeBlockTradeStatus403"],
        &full["ProposeBlockTradeStatus403"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeStatus403", value)
            .expect("ProposeBlockTradeStatus403 fixture");
        let model: ProposeBlockTradeStatus403 =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeStatus403", &output)
            .expect("ProposeBlockTradeStatus403 roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeStatus403");
    }
    for value in [
        &fixtures["ProposeBlockTradeStatus500"],
        &full["ProposeBlockTradeStatus500"],
    ] {
        kalshi_client::typed::validate("ProposeBlockTradeStatus500", value)
            .expect("ProposeBlockTradeStatus500 fixture");
        let model: ProposeBlockTradeStatus500 =
            serde_json::from_value(value.clone()).expect("ProposeBlockTradeStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ProposeBlockTradeStatus500", &output)
            .expect("ProposeBlockTradeStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ProposeBlockTradeStatus500");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalParameters"],
        &full["AcceptBlockTradeProposalParameters"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalParameters", value)
            .expect("AcceptBlockTradeProposalParameters fixture");
        let model: AcceptBlockTradeProposalParameters = serde_json::from_value(value.clone())
            .expect("AcceptBlockTradeProposalParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalParameters", &output)
            .expect("AcceptBlockTradeProposalParameters roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalParameters");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalBody"],
        &full["AcceptBlockTradeProposalBody"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalBody", value)
            .expect("AcceptBlockTradeProposalBody fixture");
        let model: AcceptBlockTradeProposalBody =
            serde_json::from_value(value.clone()).expect("AcceptBlockTradeProposalBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalBody", &output)
            .expect("AcceptBlockTradeProposalBody roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalBody");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalStatus400"],
        &full["AcceptBlockTradeProposalStatus400"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus400", value)
            .expect("AcceptBlockTradeProposalStatus400 fixture");
        let model: AcceptBlockTradeProposalStatus400 = serde_json::from_value(value.clone())
            .expect("AcceptBlockTradeProposalStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus400", &output)
            .expect("AcceptBlockTradeProposalStatus400 roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalStatus400");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalStatus401"],
        &full["AcceptBlockTradeProposalStatus401"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus401", value)
            .expect("AcceptBlockTradeProposalStatus401 fixture");
        let model: AcceptBlockTradeProposalStatus401 = serde_json::from_value(value.clone())
            .expect("AcceptBlockTradeProposalStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus401", &output)
            .expect("AcceptBlockTradeProposalStatus401 roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalStatus401");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalStatus404"],
        &full["AcceptBlockTradeProposalStatus404"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus404", value)
            .expect("AcceptBlockTradeProposalStatus404 fixture");
        let model: AcceptBlockTradeProposalStatus404 = serde_json::from_value(value.clone())
            .expect("AcceptBlockTradeProposalStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus404", &output)
            .expect("AcceptBlockTradeProposalStatus404 roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalStatus404");
    }
    for value in [
        &fixtures["AcceptBlockTradeProposalStatus500"],
        &full["AcceptBlockTradeProposalStatus500"],
    ] {
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus500", value)
            .expect("AcceptBlockTradeProposalStatus500 fixture");
        let model: AcceptBlockTradeProposalStatus500 = serde_json::from_value(value.clone())
            .expect("AcceptBlockTradeProposalStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptBlockTradeProposalStatus500", &output)
            .expect("AcceptBlockTradeProposalStatus500 roundtrip");
        model_support::assert_presence(value, &output, "AcceptBlockTradeProposalStatus500");
    }
    for value in [&fixtures["GetRfQsParameters"], &full["GetRfQsParameters"]] {
        kalshi_client::typed::validate("GetRfQsParameters", value)
            .expect("GetRfQsParameters fixture");
        let model: GetRfQsParameters =
            serde_json::from_value(value.clone()).expect("GetRfQsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfQsParameters", &output)
            .expect("GetRfQsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetRfQsParameters");
    }
    for value in [&fixtures["GetRfQsStatus200"], &full["GetRfQsStatus200"]] {
        kalshi_client::typed::validate("GetRfQsStatus200", value)
            .expect("GetRfQsStatus200 fixture");
        let model: GetRfQsStatus200 =
            serde_json::from_value(value.clone()).expect("GetRfQsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfQsStatus200", &output)
            .expect("GetRfQsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetRfQsStatus200");
    }
    for value in [&fixtures["GetRfQsStatus400"], &full["GetRfQsStatus400"]] {
        kalshi_client::typed::validate("GetRfQsStatus400", value)
            .expect("GetRfQsStatus400 fixture");
        let model: GetRfQsStatus400 =
            serde_json::from_value(value.clone()).expect("GetRfQsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfQsStatus400", &output)
            .expect("GetRfQsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetRfQsStatus400");
    }
    for value in [&fixtures["GetRfQsStatus401"], &full["GetRfQsStatus401"]] {
        kalshi_client::typed::validate("GetRfQsStatus401", value)
            .expect("GetRfQsStatus401 fixture");
        let model: GetRfQsStatus401 =
            serde_json::from_value(value.clone()).expect("GetRfQsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfQsStatus401", &output)
            .expect("GetRfQsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetRfQsStatus401");
    }
    for value in [&fixtures["GetRfQsStatus500"], &full["GetRfQsStatus500"]] {
        kalshi_client::typed::validate("GetRfQsStatus500", value)
            .expect("GetRfQsStatus500 fixture");
        let model: GetRfQsStatus500 =
            serde_json::from_value(value.clone()).expect("GetRfQsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfQsStatus500", &output)
            .expect("GetRfQsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetRfQsStatus500");
    }
    for value in [&fixtures["CreateRfqBody"], &full["CreateRfqBody"]] {
        kalshi_client::typed::validate("CreateRfqBody", value).expect("CreateRfqBody fixture");
        let model: CreateRfqBody =
            serde_json::from_value(value.clone()).expect("CreateRfqBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqBody", &output).expect("CreateRfqBody roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqBody");
    }
    for value in [&fixtures["CreateRfqStatus201"], &full["CreateRfqStatus201"]] {
        kalshi_client::typed::validate("CreateRfqStatus201", value)
            .expect("CreateRfqStatus201 fixture");
        let model: CreateRfqStatus201 =
            serde_json::from_value(value.clone()).expect("CreateRfqStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqStatus201", &output)
            .expect("CreateRfqStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqStatus201");
    }
    for value in [&fixtures["CreateRfqStatus400"], &full["CreateRfqStatus400"]] {
        kalshi_client::typed::validate("CreateRfqStatus400", value)
            .expect("CreateRfqStatus400 fixture");
        let model: CreateRfqStatus400 =
            serde_json::from_value(value.clone()).expect("CreateRfqStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqStatus400", &output)
            .expect("CreateRfqStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqStatus400");
    }
    for value in [&fixtures["CreateRfqStatus401"], &full["CreateRfqStatus401"]] {
        kalshi_client::typed::validate("CreateRfqStatus401", value)
            .expect("CreateRfqStatus401 fixture");
        let model: CreateRfqStatus401 =
            serde_json::from_value(value.clone()).expect("CreateRfqStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqStatus401", &output)
            .expect("CreateRfqStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqStatus401");
    }
    for value in [&fixtures["CreateRfqStatus409"], &full["CreateRfqStatus409"]] {
        kalshi_client::typed::validate("CreateRfqStatus409", value)
            .expect("CreateRfqStatus409 fixture");
        let model: CreateRfqStatus409 =
            serde_json::from_value(value.clone()).expect("CreateRfqStatus409 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqStatus409", &output)
            .expect("CreateRfqStatus409 roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqStatus409");
    }
    for value in [&fixtures["CreateRfqStatus500"], &full["CreateRfqStatus500"]] {
        kalshi_client::typed::validate("CreateRfqStatus500", value)
            .expect("CreateRfqStatus500 fixture");
        let model: CreateRfqStatus500 =
            serde_json::from_value(value.clone()).expect("CreateRfqStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateRfqStatus500", &output)
            .expect("CreateRfqStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateRfqStatus500");
    }
    for value in [&fixtures["GetRfqParameters"], &full["GetRfqParameters"]] {
        kalshi_client::typed::validate("GetRfqParameters", value)
            .expect("GetRfqParameters fixture");
        let model: GetRfqParameters =
            serde_json::from_value(value.clone()).expect("GetRfqParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqParameters", &output)
            .expect("GetRfqParameters roundtrip");
        model_support::assert_presence(value, &output, "GetRfqParameters");
    }
    for value in [&fixtures["GetRfqStatus200"], &full["GetRfqStatus200"]] {
        kalshi_client::typed::validate("GetRfqStatus200", value).expect("GetRfqStatus200 fixture");
        let model: GetRfqStatus200 =
            serde_json::from_value(value.clone()).expect("GetRfqStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqStatus200", &output)
            .expect("GetRfqStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqStatus200");
    }
    for value in [&fixtures["GetRfqStatus400"], &full["GetRfqStatus400"]] {
        kalshi_client::typed::validate("GetRfqStatus400", value).expect("GetRfqStatus400 fixture");
        let model: GetRfqStatus400 =
            serde_json::from_value(value.clone()).expect("GetRfqStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqStatus400", &output)
            .expect("GetRfqStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqStatus400");
    }
    for value in [&fixtures["GetRfqStatus401"], &full["GetRfqStatus401"]] {
        kalshi_client::typed::validate("GetRfqStatus401", value).expect("GetRfqStatus401 fixture");
        let model: GetRfqStatus401 =
            serde_json::from_value(value.clone()).expect("GetRfqStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqStatus401", &output)
            .expect("GetRfqStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqStatus401");
    }
    for value in [&fixtures["GetRfqStatus404"], &full["GetRfqStatus404"]] {
        kalshi_client::typed::validate("GetRfqStatus404", value).expect("GetRfqStatus404 fixture");
        let model: GetRfqStatus404 =
            serde_json::from_value(value.clone()).expect("GetRfqStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqStatus404", &output)
            .expect("GetRfqStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqStatus404");
    }
    for value in [&fixtures["GetRfqStatus500"], &full["GetRfqStatus500"]] {
        kalshi_client::typed::validate("GetRfqStatus500", value).expect("GetRfqStatus500 fixture");
        let model: GetRfqStatus500 =
            serde_json::from_value(value.clone()).expect("GetRfqStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqStatus500", &output)
            .expect("GetRfqStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqStatus500");
    }
    for value in [
        &fixtures["DeleteRfqParameters"],
        &full["DeleteRfqParameters"],
    ] {
        kalshi_client::typed::validate("DeleteRfqParameters", value)
            .expect("DeleteRfqParameters fixture");
        let model: DeleteRfqParameters =
            serde_json::from_value(value.clone()).expect("DeleteRfqParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqParameters", &output)
            .expect("DeleteRfqParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqParameters");
    }
    for value in [&fixtures["DeleteRfqStatus400"], &full["DeleteRfqStatus400"]] {
        kalshi_client::typed::validate("DeleteRfqStatus400", value)
            .expect("DeleteRfqStatus400 fixture");
        let model: DeleteRfqStatus400 =
            serde_json::from_value(value.clone()).expect("DeleteRfqStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqStatus400", &output)
            .expect("DeleteRfqStatus400 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqStatus400");
    }
    for value in [&fixtures["DeleteRfqStatus401"], &full["DeleteRfqStatus401"]] {
        kalshi_client::typed::validate("DeleteRfqStatus401", value)
            .expect("DeleteRfqStatus401 fixture");
        let model: DeleteRfqStatus401 =
            serde_json::from_value(value.clone()).expect("DeleteRfqStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqStatus401", &output)
            .expect("DeleteRfqStatus401 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqStatus401");
    }
    for value in [&fixtures["DeleteRfqStatus404"], &full["DeleteRfqStatus404"]] {
        kalshi_client::typed::validate("DeleteRfqStatus404", value)
            .expect("DeleteRfqStatus404 fixture");
        let model: DeleteRfqStatus404 =
            serde_json::from_value(value.clone()).expect("DeleteRfqStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqStatus404", &output)
            .expect("DeleteRfqStatus404 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqStatus404");
    }
    for value in [&fixtures["DeleteRfqStatus500"], &full["DeleteRfqStatus500"]] {
        kalshi_client::typed::validate("DeleteRfqStatus500", value)
            .expect("DeleteRfqStatus500 fixture");
        let model: DeleteRfqStatus500 =
            serde_json::from_value(value.clone()).expect("DeleteRfqStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqStatus500", &output)
            .expect("DeleteRfqStatus500 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqStatus500");
    }
    for value in [
        &fixtures["GetRfqQuoteParameters"],
        &full["GetRfqQuoteParameters"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteParameters", value)
            .expect("GetRfqQuoteParameters fixture");
        let model: GetRfqQuoteParameters =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteParameters", &output)
            .expect("GetRfqQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteParameters");
    }
    for value in [
        &fixtures["GetRfqQuoteStatus200"],
        &full["GetRfqQuoteStatus200"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteStatus200", value)
            .expect("GetRfqQuoteStatus200 fixture");
        let model: GetRfqQuoteStatus200 =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteStatus200", &output)
            .expect("GetRfqQuoteStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteStatus200");
    }
    for value in [
        &fixtures["GetRfqQuoteStatus400"],
        &full["GetRfqQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteStatus400", value)
            .expect("GetRfqQuoteStatus400 fixture");
        let model: GetRfqQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteStatus400", &output)
            .expect("GetRfqQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteStatus400");
    }
    for value in [
        &fixtures["GetRfqQuoteStatus401"],
        &full["GetRfqQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteStatus401", value)
            .expect("GetRfqQuoteStatus401 fixture");
        let model: GetRfqQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteStatus401", &output)
            .expect("GetRfqQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteStatus401");
    }
    for value in [
        &fixtures["GetRfqQuoteStatus404"],
        &full["GetRfqQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteStatus404", value)
            .expect("GetRfqQuoteStatus404 fixture");
        let model: GetRfqQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteStatus404", &output)
            .expect("GetRfqQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteStatus404");
    }
    for value in [
        &fixtures["GetRfqQuoteStatus500"],
        &full["GetRfqQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("GetRfqQuoteStatus500", value)
            .expect("GetRfqQuoteStatus500 fixture");
        let model: GetRfqQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("GetRfqQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetRfqQuoteStatus500", &output)
            .expect("GetRfqQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetRfqQuoteStatus500");
    }
    for value in [
        &fixtures["DeleteRfqQuoteParameters"],
        &full["DeleteRfqQuoteParameters"],
    ] {
        kalshi_client::typed::validate("DeleteRfqQuoteParameters", value)
            .expect("DeleteRfqQuoteParameters fixture");
        let model: DeleteRfqQuoteParameters =
            serde_json::from_value(value.clone()).expect("DeleteRfqQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqQuoteParameters", &output)
            .expect("DeleteRfqQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqQuoteParameters");
    }
    for value in [
        &fixtures["DeleteRfqQuoteStatus400"],
        &full["DeleteRfqQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("DeleteRfqQuoteStatus400", value)
            .expect("DeleteRfqQuoteStatus400 fixture");
        let model: DeleteRfqQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("DeleteRfqQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqQuoteStatus400", &output)
            .expect("DeleteRfqQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqQuoteStatus400");
    }
    for value in [
        &fixtures["DeleteRfqQuoteStatus401"],
        &full["DeleteRfqQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("DeleteRfqQuoteStatus401", value)
            .expect("DeleteRfqQuoteStatus401 fixture");
        let model: DeleteRfqQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("DeleteRfqQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqQuoteStatus401", &output)
            .expect("DeleteRfqQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqQuoteStatus401");
    }
    for value in [
        &fixtures["DeleteRfqQuoteStatus404"],
        &full["DeleteRfqQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("DeleteRfqQuoteStatus404", value)
            .expect("DeleteRfqQuoteStatus404 fixture");
        let model: DeleteRfqQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("DeleteRfqQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqQuoteStatus404", &output)
            .expect("DeleteRfqQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqQuoteStatus404");
    }
    for value in [
        &fixtures["DeleteRfqQuoteStatus500"],
        &full["DeleteRfqQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("DeleteRfqQuoteStatus500", value)
            .expect("DeleteRfqQuoteStatus500 fixture");
        let model: DeleteRfqQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("DeleteRfqQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteRfqQuoteStatus500", &output)
            .expect("DeleteRfqQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "DeleteRfqQuoteStatus500");
    }
    for value in [
        &fixtures["AcceptRfqQuoteParameters"],
        &full["AcceptRfqQuoteParameters"],
    ] {
        kalshi_client::typed::validate("AcceptRfqQuoteParameters", value)
            .expect("AcceptRfqQuoteParameters fixture");
        let model: AcceptRfqQuoteParameters =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteParameters", &output)
            .expect("AcceptRfqQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteParameters");
    }
    for value in [&fixtures["AcceptRfqQuoteBody"], &full["AcceptRfqQuoteBody"]] {
        kalshi_client::typed::validate("AcceptRfqQuoteBody", value)
            .expect("AcceptRfqQuoteBody fixture");
        let model: AcceptRfqQuoteBody =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteBody", &output)
            .expect("AcceptRfqQuoteBody roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteBody");
    }
    for value in [
        &fixtures["AcceptRfqQuoteStatus400"],
        &full["AcceptRfqQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("AcceptRfqQuoteStatus400", value)
            .expect("AcceptRfqQuoteStatus400 fixture");
        let model: AcceptRfqQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteStatus400", &output)
            .expect("AcceptRfqQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteStatus400");
    }
    for value in [
        &fixtures["AcceptRfqQuoteStatus401"],
        &full["AcceptRfqQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("AcceptRfqQuoteStatus401", value)
            .expect("AcceptRfqQuoteStatus401 fixture");
        let model: AcceptRfqQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteStatus401", &output)
            .expect("AcceptRfqQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteStatus401");
    }
    for value in [
        &fixtures["AcceptRfqQuoteStatus404"],
        &full["AcceptRfqQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("AcceptRfqQuoteStatus404", value)
            .expect("AcceptRfqQuoteStatus404 fixture");
        let model: AcceptRfqQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteStatus404", &output)
            .expect("AcceptRfqQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteStatus404");
    }
    for value in [
        &fixtures["AcceptRfqQuoteStatus500"],
        &full["AcceptRfqQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("AcceptRfqQuoteStatus500", value)
            .expect("AcceptRfqQuoteStatus500 fixture");
        let model: AcceptRfqQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("AcceptRfqQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptRfqQuoteStatus500", &output)
            .expect("AcceptRfqQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "AcceptRfqQuoteStatus500");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteParameters"],
        &full["ConfirmRfqQuoteParameters"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteParameters", value)
            .expect("ConfirmRfqQuoteParameters fixture");
        let model: ConfirmRfqQuoteParameters =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteParameters", &output)
            .expect("ConfirmRfqQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteParameters");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteBody"],
        &full["ConfirmRfqQuoteBody"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteBody", value)
            .expect("ConfirmRfqQuoteBody fixture");
        let model: ConfirmRfqQuoteBody =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteBody", &output)
            .expect("ConfirmRfqQuoteBody roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteBody");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteStatus400"],
        &full["ConfirmRfqQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus400", value)
            .expect("ConfirmRfqQuoteStatus400 fixture");
        let model: ConfirmRfqQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus400", &output)
            .expect("ConfirmRfqQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteStatus400");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteStatus401"],
        &full["ConfirmRfqQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus401", value)
            .expect("ConfirmRfqQuoteStatus401 fixture");
        let model: ConfirmRfqQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus401", &output)
            .expect("ConfirmRfqQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteStatus401");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteStatus404"],
        &full["ConfirmRfqQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus404", value)
            .expect("ConfirmRfqQuoteStatus404 fixture");
        let model: ConfirmRfqQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus404", &output)
            .expect("ConfirmRfqQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteStatus404");
    }
    for value in [
        &fixtures["ConfirmRfqQuoteStatus500"],
        &full["ConfirmRfqQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus500", value)
            .expect("ConfirmRfqQuoteStatus500 fixture");
        let model: ConfirmRfqQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("ConfirmRfqQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmRfqQuoteStatus500", &output)
            .expect("ConfirmRfqQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmRfqQuoteStatus500");
    }
    for value in [
        &fixtures["GetQuotesParameters"],
        &full["GetQuotesParameters"],
    ] {
        kalshi_client::typed::validate("GetQuotesParameters", value)
            .expect("GetQuotesParameters fixture");
        let model: GetQuotesParameters =
            serde_json::from_value(value.clone()).expect("GetQuotesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesParameters", &output)
            .expect("GetQuotesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesParameters");
    }
    for value in [&fixtures["GetQuotesStatus200"], &full["GetQuotesStatus200"]] {
        kalshi_client::typed::validate("GetQuotesStatus200", value)
            .expect("GetQuotesStatus200 fixture");
        let model: GetQuotesStatus200 =
            serde_json::from_value(value.clone()).expect("GetQuotesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesStatus200", &output)
            .expect("GetQuotesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesStatus200");
    }
    for value in [&fixtures["GetQuotesStatus400"], &full["GetQuotesStatus400"]] {
        kalshi_client::typed::validate("GetQuotesStatus400", value)
            .expect("GetQuotesStatus400 fixture");
        let model: GetQuotesStatus400 =
            serde_json::from_value(value.clone()).expect("GetQuotesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesStatus400", &output)
            .expect("GetQuotesStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesStatus400");
    }
    for value in [&fixtures["GetQuotesStatus401"], &full["GetQuotesStatus401"]] {
        kalshi_client::typed::validate("GetQuotesStatus401", value)
            .expect("GetQuotesStatus401 fixture");
        let model: GetQuotesStatus401 =
            serde_json::from_value(value.clone()).expect("GetQuotesStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesStatus401", &output)
            .expect("GetQuotesStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesStatus401");
    }
    for value in [&fixtures["GetQuotesStatus500"], &full["GetQuotesStatus500"]] {
        kalshi_client::typed::validate("GetQuotesStatus500", value)
            .expect("GetQuotesStatus500 fixture");
        let model: GetQuotesStatus500 =
            serde_json::from_value(value.clone()).expect("GetQuotesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuotesStatus500", &output)
            .expect("GetQuotesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetQuotesStatus500");
    }
    for value in [&fixtures["CreateQuoteBody"], &full["CreateQuoteBody"]] {
        kalshi_client::typed::validate("CreateQuoteBody", value).expect("CreateQuoteBody fixture");
        let model: CreateQuoteBody =
            serde_json::from_value(value.clone()).expect("CreateQuoteBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteBody", &output)
            .expect("CreateQuoteBody roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteBody");
    }
    for value in [
        &fixtures["CreateQuoteStatus201"],
        &full["CreateQuoteStatus201"],
    ] {
        kalshi_client::typed::validate("CreateQuoteStatus201", value)
            .expect("CreateQuoteStatus201 fixture");
        let model: CreateQuoteStatus201 =
            serde_json::from_value(value.clone()).expect("CreateQuoteStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteStatus201", &output)
            .expect("CreateQuoteStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteStatus201");
    }
    for value in [
        &fixtures["CreateQuoteStatus400"],
        &full["CreateQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("CreateQuoteStatus400", value)
            .expect("CreateQuoteStatus400 fixture");
        let model: CreateQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("CreateQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteStatus400", &output)
            .expect("CreateQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteStatus400");
    }
    for value in [
        &fixtures["CreateQuoteStatus401"],
        &full["CreateQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("CreateQuoteStatus401", value)
            .expect("CreateQuoteStatus401 fixture");
        let model: CreateQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("CreateQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteStatus401", &output)
            .expect("CreateQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteStatus401");
    }
    for value in [
        &fixtures["CreateQuoteStatus500"],
        &full["CreateQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("CreateQuoteStatus500", value)
            .expect("CreateQuoteStatus500 fixture");
        let model: CreateQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("CreateQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateQuoteStatus500", &output)
            .expect("CreateQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateQuoteStatus500");
    }
    for value in [&fixtures["GetQuoteParameters"], &full["GetQuoteParameters"]] {
        kalshi_client::typed::validate("GetQuoteParameters", value)
            .expect("GetQuoteParameters fixture");
        let model: GetQuoteParameters =
            serde_json::from_value(value.clone()).expect("GetQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteParameters", &output)
            .expect("GetQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteParameters");
    }
    for value in [&fixtures["GetQuoteStatus200"], &full["GetQuoteStatus200"]] {
        kalshi_client::typed::validate("GetQuoteStatus200", value)
            .expect("GetQuoteStatus200 fixture");
        let model: GetQuoteStatus200 =
            serde_json::from_value(value.clone()).expect("GetQuoteStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteStatus200", &output)
            .expect("GetQuoteStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteStatus200");
    }
    for value in [&fixtures["GetQuoteStatus400"], &full["GetQuoteStatus400"]] {
        kalshi_client::typed::validate("GetQuoteStatus400", value)
            .expect("GetQuoteStatus400 fixture");
        let model: GetQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("GetQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteStatus400", &output)
            .expect("GetQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteStatus400");
    }
    for value in [&fixtures["GetQuoteStatus401"], &full["GetQuoteStatus401"]] {
        kalshi_client::typed::validate("GetQuoteStatus401", value)
            .expect("GetQuoteStatus401 fixture");
        let model: GetQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("GetQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteStatus401", &output)
            .expect("GetQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteStatus401");
    }
    for value in [&fixtures["GetQuoteStatus404"], &full["GetQuoteStatus404"]] {
        kalshi_client::typed::validate("GetQuoteStatus404", value)
            .expect("GetQuoteStatus404 fixture");
        let model: GetQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("GetQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteStatus404", &output)
            .expect("GetQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteStatus404");
    }
    for value in [&fixtures["GetQuoteStatus500"], &full["GetQuoteStatus500"]] {
        kalshi_client::typed::validate("GetQuoteStatus500", value)
            .expect("GetQuoteStatus500 fixture");
        let model: GetQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("GetQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetQuoteStatus500", &output)
            .expect("GetQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetQuoteStatus500");
    }
    for value in [
        &fixtures["DeleteQuoteParameters"],
        &full["DeleteQuoteParameters"],
    ] {
        kalshi_client::typed::validate("DeleteQuoteParameters", value)
            .expect("DeleteQuoteParameters fixture");
        let model: DeleteQuoteParameters =
            serde_json::from_value(value.clone()).expect("DeleteQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteQuoteParameters", &output)
            .expect("DeleteQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteQuoteParameters");
    }
    for value in [
        &fixtures["DeleteQuoteStatus400"],
        &full["DeleteQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("DeleteQuoteStatus400", value)
            .expect("DeleteQuoteStatus400 fixture");
        let model: DeleteQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("DeleteQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteQuoteStatus400", &output)
            .expect("DeleteQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "DeleteQuoteStatus400");
    }
    for value in [
        &fixtures["DeleteQuoteStatus401"],
        &full["DeleteQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("DeleteQuoteStatus401", value)
            .expect("DeleteQuoteStatus401 fixture");
        let model: DeleteQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("DeleteQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteQuoteStatus401", &output)
            .expect("DeleteQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "DeleteQuoteStatus401");
    }
    for value in [
        &fixtures["DeleteQuoteStatus404"],
        &full["DeleteQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("DeleteQuoteStatus404", value)
            .expect("DeleteQuoteStatus404 fixture");
        let model: DeleteQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("DeleteQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteQuoteStatus404", &output)
            .expect("DeleteQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "DeleteQuoteStatus404");
    }
    for value in [
        &fixtures["DeleteQuoteStatus500"],
        &full["DeleteQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("DeleteQuoteStatus500", value)
            .expect("DeleteQuoteStatus500 fixture");
        let model: DeleteQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("DeleteQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteQuoteStatus500", &output)
            .expect("DeleteQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "DeleteQuoteStatus500");
    }
    for value in [
        &fixtures["AcceptQuoteParameters"],
        &full["AcceptQuoteParameters"],
    ] {
        kalshi_client::typed::validate("AcceptQuoteParameters", value)
            .expect("AcceptQuoteParameters fixture");
        let model: AcceptQuoteParameters =
            serde_json::from_value(value.clone()).expect("AcceptQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteParameters", &output)
            .expect("AcceptQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteParameters");
    }
    for value in [&fixtures["AcceptQuoteBody"], &full["AcceptQuoteBody"]] {
        kalshi_client::typed::validate("AcceptQuoteBody", value).expect("AcceptQuoteBody fixture");
        let model: AcceptQuoteBody =
            serde_json::from_value(value.clone()).expect("AcceptQuoteBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteBody", &output)
            .expect("AcceptQuoteBody roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteBody");
    }
    for value in [
        &fixtures["AcceptQuoteStatus400"],
        &full["AcceptQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("AcceptQuoteStatus400", value)
            .expect("AcceptQuoteStatus400 fixture");
        let model: AcceptQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("AcceptQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteStatus400", &output)
            .expect("AcceptQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteStatus400");
    }
    for value in [
        &fixtures["AcceptQuoteStatus401"],
        &full["AcceptQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("AcceptQuoteStatus401", value)
            .expect("AcceptQuoteStatus401 fixture");
        let model: AcceptQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("AcceptQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteStatus401", &output)
            .expect("AcceptQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteStatus401");
    }
    for value in [
        &fixtures["AcceptQuoteStatus404"],
        &full["AcceptQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("AcceptQuoteStatus404", value)
            .expect("AcceptQuoteStatus404 fixture");
        let model: AcceptQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("AcceptQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteStatus404", &output)
            .expect("AcceptQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteStatus404");
    }
    for value in [
        &fixtures["AcceptQuoteStatus500"],
        &full["AcceptQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("AcceptQuoteStatus500", value)
            .expect("AcceptQuoteStatus500 fixture");
        let model: AcceptQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("AcceptQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("AcceptQuoteStatus500", &output)
            .expect("AcceptQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "AcceptQuoteStatus500");
    }
    for value in [
        &fixtures["ConfirmQuoteParameters"],
        &full["ConfirmQuoteParameters"],
    ] {
        kalshi_client::typed::validate("ConfirmQuoteParameters", value)
            .expect("ConfirmQuoteParameters fixture");
        let model: ConfirmQuoteParameters =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteParameters", &output)
            .expect("ConfirmQuoteParameters roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteParameters");
    }
    for value in [&fixtures["ConfirmQuoteBody"], &full["ConfirmQuoteBody"]] {
        kalshi_client::typed::validate("ConfirmQuoteBody", value)
            .expect("ConfirmQuoteBody fixture");
        let model: ConfirmQuoteBody =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteBody", &output)
            .expect("ConfirmQuoteBody roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteBody");
    }
    for value in [
        &fixtures["ConfirmQuoteStatus400"],
        &full["ConfirmQuoteStatus400"],
    ] {
        kalshi_client::typed::validate("ConfirmQuoteStatus400", value)
            .expect("ConfirmQuoteStatus400 fixture");
        let model: ConfirmQuoteStatus400 =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteStatus400", &output)
            .expect("ConfirmQuoteStatus400 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteStatus400");
    }
    for value in [
        &fixtures["ConfirmQuoteStatus401"],
        &full["ConfirmQuoteStatus401"],
    ] {
        kalshi_client::typed::validate("ConfirmQuoteStatus401", value)
            .expect("ConfirmQuoteStatus401 fixture");
        let model: ConfirmQuoteStatus401 =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteStatus401", &output)
            .expect("ConfirmQuoteStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteStatus401");
    }
    for value in [
        &fixtures["ConfirmQuoteStatus404"],
        &full["ConfirmQuoteStatus404"],
    ] {
        kalshi_client::typed::validate("ConfirmQuoteStatus404", value)
            .expect("ConfirmQuoteStatus404 fixture");
        let model: ConfirmQuoteStatus404 =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteStatus404", &output)
            .expect("ConfirmQuoteStatus404 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteStatus404");
    }
    for value in [
        &fixtures["ConfirmQuoteStatus500"],
        &full["ConfirmQuoteStatus500"],
    ] {
        kalshi_client::typed::validate("ConfirmQuoteStatus500", value)
            .expect("ConfirmQuoteStatus500 fixture");
        let model: ConfirmQuoteStatus500 =
            serde_json::from_value(value.clone()).expect("ConfirmQuoteStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ConfirmQuoteStatus500", &output)
            .expect("ConfirmQuoteStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ConfirmQuoteStatus500");
    }
    for value in [
        &fixtures["GetApiKeysParameters"],
        &full["GetApiKeysParameters"],
    ] {
        kalshi_client::typed::validate("GetApiKeysParameters", value)
            .expect("GetApiKeysParameters fixture");
        let model: GetApiKeysParameters =
            serde_json::from_value(value.clone()).expect("GetApiKeysParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetApiKeysParameters", &output)
            .expect("GetApiKeysParameters roundtrip");
        model_support::assert_presence(value, &output, "GetApiKeysParameters");
    }
    for value in [
        &fixtures["GetApiKeysStatus200"],
        &full["GetApiKeysStatus200"],
    ] {
        kalshi_client::typed::validate("GetApiKeysStatus200", value)
            .expect("GetApiKeysStatus200 fixture");
        let model: GetApiKeysStatus200 =
            serde_json::from_value(value.clone()).expect("GetApiKeysStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetApiKeysStatus200", &output)
            .expect("GetApiKeysStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetApiKeysStatus200");
    }
    for value in [&fixtures["CreateApiKeyBody"], &full["CreateApiKeyBody"]] {
        kalshi_client::typed::validate("CreateApiKeyBody", value)
            .expect("CreateApiKeyBody fixture");
        let model: CreateApiKeyBody =
            serde_json::from_value(value.clone()).expect("CreateApiKeyBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateApiKeyBody", &output)
            .expect("CreateApiKeyBody roundtrip");
        model_support::assert_presence(value, &output, "CreateApiKeyBody");
    }
    for value in [
        &fixtures["CreateApiKeyStatus201"],
        &full["CreateApiKeyStatus201"],
    ] {
        kalshi_client::typed::validate("CreateApiKeyStatus201", value)
            .expect("CreateApiKeyStatus201 fixture");
        let model: CreateApiKeyStatus201 =
            serde_json::from_value(value.clone()).expect("CreateApiKeyStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateApiKeyStatus201", &output)
            .expect("CreateApiKeyStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateApiKeyStatus201");
    }
    for value in [&fixtures["GenerateApiKeyBody"], &full["GenerateApiKeyBody"]] {
        kalshi_client::typed::validate("GenerateApiKeyBody", value)
            .expect("GenerateApiKeyBody fixture");
        let model: GenerateApiKeyBody =
            serde_json::from_value(value.clone()).expect("GenerateApiKeyBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GenerateApiKeyBody", &output)
            .expect("GenerateApiKeyBody roundtrip");
        model_support::assert_presence(value, &output, "GenerateApiKeyBody");
    }
    for value in [
        &fixtures["GenerateApiKeyStatus201"],
        &full["GenerateApiKeyStatus201"],
    ] {
        kalshi_client::typed::validate("GenerateApiKeyStatus201", value)
            .expect("GenerateApiKeyStatus201 fixture");
        let model: GenerateApiKeyStatus201 =
            serde_json::from_value(value.clone()).expect("GenerateApiKeyStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GenerateApiKeyStatus201", &output)
            .expect("GenerateApiKeyStatus201 roundtrip");
        model_support::assert_presence(value, &output, "GenerateApiKeyStatus201");
    }
    for value in [
        &fixtures["DeleteApiKeyParameters"],
        &full["DeleteApiKeyParameters"],
    ] {
        kalshi_client::typed::validate("DeleteApiKeyParameters", value)
            .expect("DeleteApiKeyParameters fixture");
        let model: DeleteApiKeyParameters =
            serde_json::from_value(value.clone()).expect("DeleteApiKeyParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteApiKeyParameters", &output)
            .expect("DeleteApiKeyParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteApiKeyParameters");
    }
    for value in [
        &fixtures["GetAccountApiLimitsStatus200"],
        &full["GetAccountApiLimitsStatus200"],
    ] {
        kalshi_client::typed::validate("GetAccountApiLimitsStatus200", value)
            .expect("GetAccountApiLimitsStatus200 fixture");
        let model: GetAccountApiLimitsStatus200 =
            serde_json::from_value(value.clone()).expect("GetAccountApiLimitsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountApiLimitsStatus200", &output)
            .expect("GetAccountApiLimitsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetAccountApiLimitsStatus200");
    }
    for value in [
        &fixtures["GetAccountApiUsageLevelVolumeProgressStatus200"],
        &full["GetAccountApiUsageLevelVolumeProgressStatus200"],
    ] {
        kalshi_client::typed::validate("GetAccountApiUsageLevelVolumeProgressStatus200", value)
            .expect("GetAccountApiUsageLevelVolumeProgressStatus200 fixture");
        let model: GetAccountApiUsageLevelVolumeProgressStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetAccountApiUsageLevelVolumeProgressStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountApiUsageLevelVolumeProgressStatus200", &output)
            .expect("GetAccountApiUsageLevelVolumeProgressStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetAccountApiUsageLevelVolumeProgressStatus200",
        );
    }
    for value in [
        &fixtures["GetAccountEndpointCostsStatus200"],
        &full["GetAccountEndpointCostsStatus200"],
    ] {
        kalshi_client::typed::validate("GetAccountEndpointCostsStatus200", value)
            .expect("GetAccountEndpointCostsStatus200 fixture");
        let model: GetAccountEndpointCostsStatus200 =
            serde_json::from_value(value.clone()).expect("GetAccountEndpointCostsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetAccountEndpointCostsStatus200", &output)
            .expect("GetAccountEndpointCostsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetAccountEndpointCostsStatus200");
    }
    for value in [
        &fixtures["GetTagsForSeriesCategoriesStatus200"],
        &full["GetTagsForSeriesCategoriesStatus200"],
    ] {
        kalshi_client::typed::validate("GetTagsForSeriesCategoriesStatus200", value)
            .expect("GetTagsForSeriesCategoriesStatus200 fixture");
        let model: GetTagsForSeriesCategoriesStatus200 = serde_json::from_value(value.clone())
            .expect("GetTagsForSeriesCategoriesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTagsForSeriesCategoriesStatus200", &output)
            .expect("GetTagsForSeriesCategoriesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetTagsForSeriesCategoriesStatus200");
    }
    for value in [
        &fixtures["GetFiltersForSportsStatus200"],
        &full["GetFiltersForSportsStatus200"],
    ] {
        kalshi_client::typed::validate("GetFiltersForSportsStatus200", value)
            .expect("GetFiltersForSportsStatus200 fixture");
        let model: GetFiltersForSportsStatus200 =
            serde_json::from_value(value.clone()).expect("GetFiltersForSportsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFiltersForSportsStatus200", &output)
            .expect("GetFiltersForSportsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFiltersForSportsStatus200");
    }
    for value in [
        &fixtures["GetLiveDataByMilestoneParameters"],
        &full["GetLiveDataByMilestoneParameters"],
    ] {
        kalshi_client::typed::validate("GetLiveDataByMilestoneParameters", value)
            .expect("GetLiveDataByMilestoneParameters fixture");
        let model: GetLiveDataByMilestoneParameters =
            serde_json::from_value(value.clone()).expect("GetLiveDataByMilestoneParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDataByMilestoneParameters", &output)
            .expect("GetLiveDataByMilestoneParameters roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDataByMilestoneParameters");
    }
    for value in [
        &fixtures["GetLiveDataByMilestoneStatus200"],
        &full["GetLiveDataByMilestoneStatus200"],
    ] {
        kalshi_client::typed::validate("GetLiveDataByMilestoneStatus200", value)
            .expect("GetLiveDataByMilestoneStatus200 fixture");
        let model: GetLiveDataByMilestoneStatus200 =
            serde_json::from_value(value.clone()).expect("GetLiveDataByMilestoneStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDataByMilestoneStatus200", &output)
            .expect("GetLiveDataByMilestoneStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDataByMilestoneStatus200");
    }
    for value in [
        &fixtures["GetLiveDataParameters"],
        &full["GetLiveDataParameters"],
    ] {
        kalshi_client::typed::validate("GetLiveDataParameters", value)
            .expect("GetLiveDataParameters fixture");
        let model: GetLiveDataParameters =
            serde_json::from_value(value.clone()).expect("GetLiveDataParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDataParameters", &output)
            .expect("GetLiveDataParameters roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDataParameters");
    }
    for value in [
        &fixtures["GetLiveDataStatus200"],
        &full["GetLiveDataStatus200"],
    ] {
        kalshi_client::typed::validate("GetLiveDataStatus200", value)
            .expect("GetLiveDataStatus200 fixture");
        let model: GetLiveDataStatus200 =
            serde_json::from_value(value.clone()).expect("GetLiveDataStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDataStatus200", &output)
            .expect("GetLiveDataStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDataStatus200");
    }
    for value in [
        &fixtures["GetLiveDatasParameters"],
        &full["GetLiveDatasParameters"],
    ] {
        kalshi_client::typed::validate("GetLiveDatasParameters", value)
            .expect("GetLiveDatasParameters fixture");
        let model: GetLiveDatasParameters =
            serde_json::from_value(value.clone()).expect("GetLiveDatasParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDatasParameters", &output)
            .expect("GetLiveDatasParameters roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDatasParameters");
    }
    for value in [
        &fixtures["GetLiveDatasStatus200"],
        &full["GetLiveDatasStatus200"],
    ] {
        kalshi_client::typed::validate("GetLiveDatasStatus200", value)
            .expect("GetLiveDatasStatus200 fixture");
        let model: GetLiveDatasStatus200 =
            serde_json::from_value(value.clone()).expect("GetLiveDatasStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetLiveDatasStatus200", &output)
            .expect("GetLiveDatasStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetLiveDatasStatus200");
    }
    for value in [
        &fixtures["GetGameStatsParameters"],
        &full["GetGameStatsParameters"],
    ] {
        kalshi_client::typed::validate("GetGameStatsParameters", value)
            .expect("GetGameStatsParameters fixture");
        let model: GetGameStatsParameters =
            serde_json::from_value(value.clone()).expect("GetGameStatsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetGameStatsParameters", &output)
            .expect("GetGameStatsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetGameStatsParameters");
    }
    for value in [
        &fixtures["GetGameStatsStatus200"],
        &full["GetGameStatsStatus200"],
    ] {
        kalshi_client::typed::validate("GetGameStatsStatus200", value)
            .expect("GetGameStatsStatus200 fixture");
        let model: GetGameStatsStatus200 =
            serde_json::from_value(value.clone()).expect("GetGameStatsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetGameStatsStatus200", &output)
            .expect("GetGameStatsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetGameStatsStatus200");
    }
    for value in [
        &fixtures["GetEventLiveDataParameters"],
        &full["GetEventLiveDataParameters"],
    ] {
        kalshi_client::typed::validate("GetEventLiveDataParameters", value)
            .expect("GetEventLiveDataParameters fixture");
        let model: GetEventLiveDataParameters =
            serde_json::from_value(value.clone()).expect("GetEventLiveDataParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventLiveDataParameters", &output)
            .expect("GetEventLiveDataParameters roundtrip");
        model_support::assert_presence(value, &output, "GetEventLiveDataParameters");
    }
    for value in [
        &fixtures["GetEventLiveDataStatus200"],
        &full["GetEventLiveDataStatus200"],
    ] {
        kalshi_client::typed::validate("GetEventLiveDataStatus200", value)
            .expect("GetEventLiveDataStatus200 fixture");
        let model: GetEventLiveDataStatus200 =
            serde_json::from_value(value.clone()).expect("GetEventLiveDataStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetEventLiveDataStatus200", &output)
            .expect("GetEventLiveDataStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetEventLiveDataStatus200");
    }
    for value in [
        &fixtures["GetWeatherIndexParameters"],
        &full["GetWeatherIndexParameters"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexParameters", value)
            .expect("GetWeatherIndexParameters fixture");
        let model: GetWeatherIndexParameters =
            serde_json::from_value(value.clone()).expect("GetWeatherIndexParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexParameters", &output)
            .expect("GetWeatherIndexParameters roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexParameters");
    }
    for value in [
        &fixtures["GetWeatherIndexStatus200"],
        &full["GetWeatherIndexStatus200"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexStatus200", value)
            .expect("GetWeatherIndexStatus200 fixture");
        let model: GetWeatherIndexStatus200 =
            serde_json::from_value(value.clone()).expect("GetWeatherIndexStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexStatus200", &output)
            .expect("GetWeatherIndexStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexStatus200");
    }
    for value in [
        &fixtures["GetWeatherIndexCalibrationsParameters"],
        &full["GetWeatherIndexCalibrationsParameters"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsParameters", value)
            .expect("GetWeatherIndexCalibrationsParameters fixture");
        let model: GetWeatherIndexCalibrationsParameters = serde_json::from_value(value.clone())
            .expect("GetWeatherIndexCalibrationsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsParameters", &output)
            .expect("GetWeatherIndexCalibrationsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexCalibrationsParameters");
    }
    for value in [
        &fixtures["GetWeatherIndexCalibrationsStatus200"],
        &full["GetWeatherIndexCalibrationsStatus200"],
    ] {
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsStatus200", value)
            .expect("GetWeatherIndexCalibrationsStatus200 fixture");
        let model: GetWeatherIndexCalibrationsStatus200 = serde_json::from_value(value.clone())
            .expect("GetWeatherIndexCalibrationsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetWeatherIndexCalibrationsStatus200", &output)
            .expect("GetWeatherIndexCalibrationsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetWeatherIndexCalibrationsStatus200");
    }
    for value in [
        &fixtures["GetStructuredTargetsParameters"],
        &full["GetStructuredTargetsParameters"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetsParameters", value)
            .expect("GetStructuredTargetsParameters fixture");
        let model: GetStructuredTargetsParameters =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetsParameters", &output)
            .expect("GetStructuredTargetsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetsParameters");
    }
    for value in [
        &fixtures["GetStructuredTargetsStatus200"],
        &full["GetStructuredTargetsStatus200"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetsStatus200", value)
            .expect("GetStructuredTargetsStatus200 fixture");
        let model: GetStructuredTargetsStatus200 =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetsStatus200", &output)
            .expect("GetStructuredTargetsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetsStatus200");
    }
    for value in [
        &fixtures["GetStructuredTargetParameters"],
        &full["GetStructuredTargetParameters"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetParameters", value)
            .expect("GetStructuredTargetParameters fixture");
        let model: GetStructuredTargetParameters =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetParameters", &output)
            .expect("GetStructuredTargetParameters roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetParameters");
    }
    for value in [
        &fixtures["GetStructuredTargetStatus200"],
        &full["GetStructuredTargetStatus200"],
    ] {
        kalshi_client::typed::validate("GetStructuredTargetStatus200", value)
            .expect("GetStructuredTargetStatus200 fixture");
        let model: GetStructuredTargetStatus200 =
            serde_json::from_value(value.clone()).expect("GetStructuredTargetStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetStructuredTargetStatus200", &output)
            .expect("GetStructuredTargetStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetStructuredTargetStatus200");
    }
    for value in [
        &fixtures["GetMilestoneParameters"],
        &full["GetMilestoneParameters"],
    ] {
        kalshi_client::typed::validate("GetMilestoneParameters", value)
            .expect("GetMilestoneParameters fixture");
        let model: GetMilestoneParameters =
            serde_json::from_value(value.clone()).expect("GetMilestoneParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestoneParameters", &output)
            .expect("GetMilestoneParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMilestoneParameters");
    }
    for value in [
        &fixtures["GetMilestoneStatus200"],
        &full["GetMilestoneStatus200"],
    ] {
        kalshi_client::typed::validate("GetMilestoneStatus200", value)
            .expect("GetMilestoneStatus200 fixture");
        let model: GetMilestoneStatus200 =
            serde_json::from_value(value.clone()).expect("GetMilestoneStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestoneStatus200", &output)
            .expect("GetMilestoneStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMilestoneStatus200");
    }
    for value in [
        &fixtures["GetMilestonesParameters"],
        &full["GetMilestonesParameters"],
    ] {
        kalshi_client::typed::validate("GetMilestonesParameters", value)
            .expect("GetMilestonesParameters fixture");
        let model: GetMilestonesParameters =
            serde_json::from_value(value.clone()).expect("GetMilestonesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestonesParameters", &output)
            .expect("GetMilestonesParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMilestonesParameters");
    }
    for value in [
        &fixtures["GetMilestonesStatus200"],
        &full["GetMilestonesStatus200"],
    ] {
        kalshi_client::typed::validate("GetMilestonesStatus200", value)
            .expect("GetMilestonesStatus200 fixture");
        let model: GetMilestonesStatus200 =
            serde_json::from_value(value.clone()).expect("GetMilestonesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMilestonesStatus200", &output)
            .expect("GetMilestonesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMilestonesStatus200");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionParameters"],
        &full["GetMultivariateEventCollectionParameters"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionParameters", value)
            .expect("GetMultivariateEventCollectionParameters fixture");
        let model: GetMultivariateEventCollectionParameters = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionParameters", &output)
            .expect("GetMultivariateEventCollectionParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionParameters");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionStatus200"],
        &full["GetMultivariateEventCollectionStatus200"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus200", value)
            .expect("GetMultivariateEventCollectionStatus200 fixture");
        let model: GetMultivariateEventCollectionStatus200 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus200", &output)
            .expect("GetMultivariateEventCollectionStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionStatus200");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionStatus400"],
        &full["GetMultivariateEventCollectionStatus400"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus400", value)
            .expect("GetMultivariateEventCollectionStatus400 fixture");
        let model: GetMultivariateEventCollectionStatus400 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus400", &output)
            .expect("GetMultivariateEventCollectionStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionStatus400");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionStatus404"],
        &full["GetMultivariateEventCollectionStatus404"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus404", value)
            .expect("GetMultivariateEventCollectionStatus404 fixture");
        let model: GetMultivariateEventCollectionStatus404 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus404", &output)
            .expect("GetMultivariateEventCollectionStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionStatus404");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionStatus500"],
        &full["GetMultivariateEventCollectionStatus500"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus500", value)
            .expect("GetMultivariateEventCollectionStatus500 fixture");
        let model: GetMultivariateEventCollectionStatus500 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionStatus500", &output)
            .expect("GetMultivariateEventCollectionStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionStatus500");
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionParameters"],
        &full["CreateMarketInMultivariateEventCollectionParameters"],
    ] {
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionParameters",
            value,
        )
        .expect("CreateMarketInMultivariateEventCollectionParameters fixture");
        let model: CreateMarketInMultivariateEventCollectionParameters =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionParameters",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionParameters roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionParameters",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionBody"],
        &full["CreateMarketInMultivariateEventCollectionBody"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionBody", value)
            .expect("CreateMarketInMultivariateEventCollectionBody fixture");
        let model: CreateMarketInMultivariateEventCollectionBody =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionBody", &output)
            .expect("CreateMarketInMultivariateEventCollectionBody roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionBody",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionStatus200"],
        &full["CreateMarketInMultivariateEventCollectionStatus200"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionStatus200", value)
            .expect("CreateMarketInMultivariateEventCollectionStatus200 fixture");
        let model: CreateMarketInMultivariateEventCollectionStatus200 =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus200",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionStatus200",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionStatus400"],
        &full["CreateMarketInMultivariateEventCollectionStatus400"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionStatus400", value)
            .expect("CreateMarketInMultivariateEventCollectionStatus400 fixture");
        let model: CreateMarketInMultivariateEventCollectionStatus400 =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus400",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionStatus400 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionStatus400",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionStatus401"],
        &full["CreateMarketInMultivariateEventCollectionStatus401"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionStatus401", value)
            .expect("CreateMarketInMultivariateEventCollectionStatus401 fixture");
        let model: CreateMarketInMultivariateEventCollectionStatus401 =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus401",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionStatus401 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionStatus401",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionStatus429"],
        &full["CreateMarketInMultivariateEventCollectionStatus429"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionStatus429", value)
            .expect("CreateMarketInMultivariateEventCollectionStatus429 fixture");
        let model: CreateMarketInMultivariateEventCollectionStatus429 =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionStatus429 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus429",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionStatus429 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionStatus429",
        );
    }
    for value in [
        &fixtures["CreateMarketInMultivariateEventCollectionStatus500"],
        &full["CreateMarketInMultivariateEventCollectionStatus500"],
    ] {
        kalshi_client::typed::validate("CreateMarketInMultivariateEventCollectionStatus500", value)
            .expect("CreateMarketInMultivariateEventCollectionStatus500 fixture");
        let model: CreateMarketInMultivariateEventCollectionStatus500 =
            serde_json::from_value(value.clone())
                .expect("CreateMarketInMultivariateEventCollectionStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate(
            "CreateMarketInMultivariateEventCollectionStatus500",
            &output,
        )
        .expect("CreateMarketInMultivariateEventCollectionStatus500 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "CreateMarketInMultivariateEventCollectionStatus500",
        );
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionsParameters"],
        &full["GetMultivariateEventCollectionsParameters"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionsParameters", value)
            .expect("GetMultivariateEventCollectionsParameters fixture");
        let model: GetMultivariateEventCollectionsParameters =
            serde_json::from_value(value.clone())
                .expect("GetMultivariateEventCollectionsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionsParameters", &output)
            .expect("GetMultivariateEventCollectionsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionsParameters");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionsStatus200"],
        &full["GetMultivariateEventCollectionsStatus200"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus200", value)
            .expect("GetMultivariateEventCollectionsStatus200 fixture");
        let model: GetMultivariateEventCollectionsStatus200 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus200", &output)
            .expect("GetMultivariateEventCollectionsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionsStatus200");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionsStatus400"],
        &full["GetMultivariateEventCollectionsStatus400"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus400", value)
            .expect("GetMultivariateEventCollectionsStatus400 fixture");
        let model: GetMultivariateEventCollectionsStatus400 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus400", &output)
            .expect("GetMultivariateEventCollectionsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionsStatus400");
    }
    for value in [
        &fixtures["GetMultivariateEventCollectionsStatus500"],
        &full["GetMultivariateEventCollectionsStatus500"],
    ] {
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus500", value)
            .expect("GetMultivariateEventCollectionsStatus500 fixture");
        let model: GetMultivariateEventCollectionsStatus500 = serde_json::from_value(value.clone())
            .expect("GetMultivariateEventCollectionsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMultivariateEventCollectionsStatus500", &output)
            .expect("GetMultivariateEventCollectionsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetMultivariateEventCollectionsStatus500");
    }
    for value in [
        &fixtures["GetIncentiveProgramsParameters"],
        &full["GetIncentiveProgramsParameters"],
    ] {
        kalshi_client::typed::validate("GetIncentiveProgramsParameters", value)
            .expect("GetIncentiveProgramsParameters fixture");
        let model: GetIncentiveProgramsParameters =
            serde_json::from_value(value.clone()).expect("GetIncentiveProgramsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIncentiveProgramsParameters", &output)
            .expect("GetIncentiveProgramsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetIncentiveProgramsParameters");
    }
    for value in [
        &fixtures["GetIncentiveProgramsStatus200"],
        &full["GetIncentiveProgramsStatus200"],
    ] {
        kalshi_client::typed::validate("GetIncentiveProgramsStatus200", value)
            .expect("GetIncentiveProgramsStatus200 fixture");
        let model: GetIncentiveProgramsStatus200 =
            serde_json::from_value(value.clone()).expect("GetIncentiveProgramsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIncentiveProgramsStatus200", &output)
            .expect("GetIncentiveProgramsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetIncentiveProgramsStatus200");
    }
    for value in [
        &fixtures["GetIncentiveProgramsStatus400"],
        &full["GetIncentiveProgramsStatus400"],
    ] {
        kalshi_client::typed::validate("GetIncentiveProgramsStatus400", value)
            .expect("GetIncentiveProgramsStatus400 fixture");
        let model: GetIncentiveProgramsStatus400 =
            serde_json::from_value(value.clone()).expect("GetIncentiveProgramsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIncentiveProgramsStatus400", &output)
            .expect("GetIncentiveProgramsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetIncentiveProgramsStatus400");
    }
    for value in [
        &fixtures["GetIncentiveProgramsStatus500"],
        &full["GetIncentiveProgramsStatus500"],
    ] {
        kalshi_client::typed::validate("GetIncentiveProgramsStatus500", value)
            .expect("GetIncentiveProgramsStatus500 fixture");
        let model: GetIncentiveProgramsStatus500 =
            serde_json::from_value(value.clone()).expect("GetIncentiveProgramsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetIncentiveProgramsStatus500", &output)
            .expect("GetIncentiveProgramsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetIncentiveProgramsStatus500");
    }
    for value in [
        &fixtures["GetFcmFillsParameters"],
        &full["GetFcmFillsParameters"],
    ] {
        kalshi_client::typed::validate("GetFcmFillsParameters", value)
            .expect("GetFcmFillsParameters fixture");
        let model: GetFcmFillsParameters =
            serde_json::from_value(value.clone()).expect("GetFcmFillsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmFillsParameters", &output)
            .expect("GetFcmFillsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFcmFillsParameters");
    }
    for value in [
        &fixtures["GetFcmFillsStatus200"],
        &full["GetFcmFillsStatus200"],
    ] {
        kalshi_client::typed::validate("GetFcmFillsStatus200", value)
            .expect("GetFcmFillsStatus200 fixture");
        let model: GetFcmFillsStatus200 =
            serde_json::from_value(value.clone()).expect("GetFcmFillsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmFillsStatus200", &output)
            .expect("GetFcmFillsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmFillsStatus200");
    }
    for value in [
        &fixtures["GetFcmOrdersParameters"],
        &full["GetFcmOrdersParameters"],
    ] {
        kalshi_client::typed::validate("GetFcmOrdersParameters", value)
            .expect("GetFcmOrdersParameters fixture");
        let model: GetFcmOrdersParameters =
            serde_json::from_value(value.clone()).expect("GetFcmOrdersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmOrdersParameters", &output)
            .expect("GetFcmOrdersParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFcmOrdersParameters");
    }
    for value in [
        &fixtures["GetFcmOrdersStatus200"],
        &full["GetFcmOrdersStatus200"],
    ] {
        kalshi_client::typed::validate("GetFcmOrdersStatus200", value)
            .expect("GetFcmOrdersStatus200 fixture");
        let model: GetFcmOrdersStatus200 =
            serde_json::from_value(value.clone()).expect("GetFcmOrdersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmOrdersStatus200", &output)
            .expect("GetFcmOrdersStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmOrdersStatus200");
    }
    for value in [
        &fixtures["ListFcmSubtradersStatus200"],
        &full["ListFcmSubtradersStatus200"],
    ] {
        kalshi_client::typed::validate("ListFcmSubtradersStatus200", value)
            .expect("ListFcmSubtradersStatus200 fixture");
        let model: ListFcmSubtradersStatus200 =
            serde_json::from_value(value.clone()).expect("ListFcmSubtradersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ListFcmSubtradersStatus200", &output)
            .expect("ListFcmSubtradersStatus200 roundtrip");
        model_support::assert_presence(value, &output, "ListFcmSubtradersStatus200");
    }
    for value in [
        &fixtures["ListFcmSubtradersStatus401"],
        &full["ListFcmSubtradersStatus401"],
    ] {
        kalshi_client::typed::validate("ListFcmSubtradersStatus401", value)
            .expect("ListFcmSubtradersStatus401 fixture");
        let model: ListFcmSubtradersStatus401 =
            serde_json::from_value(value.clone()).expect("ListFcmSubtradersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ListFcmSubtradersStatus401", &output)
            .expect("ListFcmSubtradersStatus401 roundtrip");
        model_support::assert_presence(value, &output, "ListFcmSubtradersStatus401");
    }
    for value in [
        &fixtures["ListFcmSubtradersStatus403"],
        &full["ListFcmSubtradersStatus403"],
    ] {
        kalshi_client::typed::validate("ListFcmSubtradersStatus403", value)
            .expect("ListFcmSubtradersStatus403 fixture");
        let model: ListFcmSubtradersStatus403 =
            serde_json::from_value(value.clone()).expect("ListFcmSubtradersStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ListFcmSubtradersStatus403", &output)
            .expect("ListFcmSubtradersStatus403 roundtrip");
        model_support::assert_presence(value, &output, "ListFcmSubtradersStatus403");
    }
    for value in [
        &fixtures["ListFcmSubtradersStatus500"],
        &full["ListFcmSubtradersStatus500"],
    ] {
        kalshi_client::typed::validate("ListFcmSubtradersStatus500", value)
            .expect("ListFcmSubtradersStatus500 fixture");
        let model: ListFcmSubtradersStatus500 =
            serde_json::from_value(value.clone()).expect("ListFcmSubtradersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("ListFcmSubtradersStatus500", &output)
            .expect("ListFcmSubtradersStatus500 roundtrip");
        model_support::assert_presence(value, &output, "ListFcmSubtradersStatus500");
    }
    for value in [
        &fixtures["CreateFcmSubtraderBody"],
        &full["CreateFcmSubtraderBody"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderBody", value)
            .expect("CreateFcmSubtraderBody fixture");
        let model: CreateFcmSubtraderBody =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderBody", &output)
            .expect("CreateFcmSubtraderBody roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderBody");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus201"],
        &full["CreateFcmSubtraderStatus201"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus201", value)
            .expect("CreateFcmSubtraderStatus201 fixture");
        let model: CreateFcmSubtraderStatus201 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus201", &output)
            .expect("CreateFcmSubtraderStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus201");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus400"],
        &full["CreateFcmSubtraderStatus400"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus400", value)
            .expect("CreateFcmSubtraderStatus400 fixture");
        let model: CreateFcmSubtraderStatus400 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus400", &output)
            .expect("CreateFcmSubtraderStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus400");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus401"],
        &full["CreateFcmSubtraderStatus401"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus401", value)
            .expect("CreateFcmSubtraderStatus401 fixture");
        let model: CreateFcmSubtraderStatus401 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus401", &output)
            .expect("CreateFcmSubtraderStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus401");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus403"],
        &full["CreateFcmSubtraderStatus403"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus403", value)
            .expect("CreateFcmSubtraderStatus403 fixture");
        let model: CreateFcmSubtraderStatus403 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus403", &output)
            .expect("CreateFcmSubtraderStatus403 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus403");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus409"],
        &full["CreateFcmSubtraderStatus409"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus409", value)
            .expect("CreateFcmSubtraderStatus409 fixture");
        let model: CreateFcmSubtraderStatus409 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus409 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus409", &output)
            .expect("CreateFcmSubtraderStatus409 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus409");
    }
    for value in [
        &fixtures["CreateFcmSubtraderStatus500"],
        &full["CreateFcmSubtraderStatus500"],
    ] {
        kalshi_client::typed::validate("CreateFcmSubtraderStatus500", value)
            .expect("CreateFcmSubtraderStatus500 fixture");
        let model: CreateFcmSubtraderStatus500 =
            serde_json::from_value(value.clone()).expect("CreateFcmSubtraderStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmSubtraderStatus500", &output)
            .expect("CreateFcmSubtraderStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmSubtraderStatus500");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapParameters"],
        &full["GetFcmEventContractDailyCapParameters"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapParameters", value)
            .expect("GetFcmEventContractDailyCapParameters fixture");
        let model: GetFcmEventContractDailyCapParameters = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapParameters", &output)
            .expect("GetFcmEventContractDailyCapParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapParameters");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus200"],
        &full["GetFcmEventContractDailyCapStatus200"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus200", value)
            .expect("GetFcmEventContractDailyCapStatus200 fixture");
        let model: GetFcmEventContractDailyCapStatus200 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus200", &output)
            .expect("GetFcmEventContractDailyCapStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus200");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus400"],
        &full["GetFcmEventContractDailyCapStatus400"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus400", value)
            .expect("GetFcmEventContractDailyCapStatus400 fixture");
        let model: GetFcmEventContractDailyCapStatus400 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus400", &output)
            .expect("GetFcmEventContractDailyCapStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus400");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus401"],
        &full["GetFcmEventContractDailyCapStatus401"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus401", value)
            .expect("GetFcmEventContractDailyCapStatus401 fixture");
        let model: GetFcmEventContractDailyCapStatus401 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus401", &output)
            .expect("GetFcmEventContractDailyCapStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus401");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus403"],
        &full["GetFcmEventContractDailyCapStatus403"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus403", value)
            .expect("GetFcmEventContractDailyCapStatus403 fixture");
        let model: GetFcmEventContractDailyCapStatus403 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus403", &output)
            .expect("GetFcmEventContractDailyCapStatus403 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus403");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus404"],
        &full["GetFcmEventContractDailyCapStatus404"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus404", value)
            .expect("GetFcmEventContractDailyCapStatus404 fixture");
        let model: GetFcmEventContractDailyCapStatus404 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus404", &output)
            .expect("GetFcmEventContractDailyCapStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus404");
    }
    for value in [
        &fixtures["GetFcmEventContractDailyCapStatus500"],
        &full["GetFcmEventContractDailyCapStatus500"],
    ] {
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus500", value)
            .expect("GetFcmEventContractDailyCapStatus500 fixture");
        let model: GetFcmEventContractDailyCapStatus500 = serde_json::from_value(value.clone())
            .expect("GetFcmEventContractDailyCapStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmEventContractDailyCapStatus500", &output)
            .expect("GetFcmEventContractDailyCapStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmEventContractDailyCapStatus500");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapBody"],
        &full["UpdateFcmEventContractDailyCapBody"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapBody", value)
            .expect("UpdateFcmEventContractDailyCapBody fixture");
        let model: UpdateFcmEventContractDailyCapBody = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapBody", &output)
            .expect("UpdateFcmEventContractDailyCapBody roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapBody");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus200"],
        &full["UpdateFcmEventContractDailyCapStatus200"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus200", value)
            .expect("UpdateFcmEventContractDailyCapStatus200 fixture");
        let model: UpdateFcmEventContractDailyCapStatus200 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus200", &output)
            .expect("UpdateFcmEventContractDailyCapStatus200 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus200");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus400"],
        &full["UpdateFcmEventContractDailyCapStatus400"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus400", value)
            .expect("UpdateFcmEventContractDailyCapStatus400 fixture");
        let model: UpdateFcmEventContractDailyCapStatus400 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus400", &output)
            .expect("UpdateFcmEventContractDailyCapStatus400 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus400");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus401"],
        &full["UpdateFcmEventContractDailyCapStatus401"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus401", value)
            .expect("UpdateFcmEventContractDailyCapStatus401 fixture");
        let model: UpdateFcmEventContractDailyCapStatus401 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus401", &output)
            .expect("UpdateFcmEventContractDailyCapStatus401 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus401");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus403"],
        &full["UpdateFcmEventContractDailyCapStatus403"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus403", value)
            .expect("UpdateFcmEventContractDailyCapStatus403 fixture");
        let model: UpdateFcmEventContractDailyCapStatus403 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus403", &output)
            .expect("UpdateFcmEventContractDailyCapStatus403 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus403");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus404"],
        &full["UpdateFcmEventContractDailyCapStatus404"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus404", value)
            .expect("UpdateFcmEventContractDailyCapStatus404 fixture");
        let model: UpdateFcmEventContractDailyCapStatus404 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus404", &output)
            .expect("UpdateFcmEventContractDailyCapStatus404 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus404");
    }
    for value in [
        &fixtures["UpdateFcmEventContractDailyCapStatus500"],
        &full["UpdateFcmEventContractDailyCapStatus500"],
    ] {
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus500", value)
            .expect("UpdateFcmEventContractDailyCapStatus500 fixture");
        let model: UpdateFcmEventContractDailyCapStatus500 = serde_json::from_value(value.clone())
            .expect("UpdateFcmEventContractDailyCapStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmEventContractDailyCapStatus500", &output)
            .expect("UpdateFcmEventContractDailyCapStatus500 roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmEventContractDailyCapStatus500");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapParameters"],
        &full["DeleteFcmEventContractDailyCapParameters"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapParameters", value)
            .expect("DeleteFcmEventContractDailyCapParameters fixture");
        let model: DeleteFcmEventContractDailyCapParameters = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapParameters", &output)
            .expect("DeleteFcmEventContractDailyCapParameters roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapParameters");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus200"],
        &full["DeleteFcmEventContractDailyCapStatus200"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus200", value)
            .expect("DeleteFcmEventContractDailyCapStatus200 fixture");
        let model: DeleteFcmEventContractDailyCapStatus200 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus200", &output)
            .expect("DeleteFcmEventContractDailyCapStatus200 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus200");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus400"],
        &full["DeleteFcmEventContractDailyCapStatus400"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus400", value)
            .expect("DeleteFcmEventContractDailyCapStatus400 fixture");
        let model: DeleteFcmEventContractDailyCapStatus400 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus400", &output)
            .expect("DeleteFcmEventContractDailyCapStatus400 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus400");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus401"],
        &full["DeleteFcmEventContractDailyCapStatus401"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus401", value)
            .expect("DeleteFcmEventContractDailyCapStatus401 fixture");
        let model: DeleteFcmEventContractDailyCapStatus401 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus401", &output)
            .expect("DeleteFcmEventContractDailyCapStatus401 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus401");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus403"],
        &full["DeleteFcmEventContractDailyCapStatus403"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus403", value)
            .expect("DeleteFcmEventContractDailyCapStatus403 fixture");
        let model: DeleteFcmEventContractDailyCapStatus403 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus403", &output)
            .expect("DeleteFcmEventContractDailyCapStatus403 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus403");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus404"],
        &full["DeleteFcmEventContractDailyCapStatus404"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus404", value)
            .expect("DeleteFcmEventContractDailyCapStatus404 fixture");
        let model: DeleteFcmEventContractDailyCapStatus404 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus404", &output)
            .expect("DeleteFcmEventContractDailyCapStatus404 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus404");
    }
    for value in [
        &fixtures["DeleteFcmEventContractDailyCapStatus500"],
        &full["DeleteFcmEventContractDailyCapStatus500"],
    ] {
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus500", value)
            .expect("DeleteFcmEventContractDailyCapStatus500 fixture");
        let model: DeleteFcmEventContractDailyCapStatus500 = serde_json::from_value(value.clone())
            .expect("DeleteFcmEventContractDailyCapStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("DeleteFcmEventContractDailyCapStatus500", &output)
            .expect("DeleteFcmEventContractDailyCapStatus500 roundtrip");
        model_support::assert_presence(value, &output, "DeleteFcmEventContractDailyCapStatus500");
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesParameters"],
        &full["GetFcmSubtraderBlockedCategoriesParameters"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesParameters", value)
            .expect("GetFcmSubtraderBlockedCategoriesParameters fixture");
        let model: GetFcmSubtraderBlockedCategoriesParameters =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesParameters", &output)
            .expect("GetFcmSubtraderBlockedCategoriesParameters roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "GetFcmSubtraderBlockedCategoriesParameters",
        );
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesStatus200"],
        &full["GetFcmSubtraderBlockedCategoriesStatus200"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus200", value)
            .expect("GetFcmSubtraderBlockedCategoriesStatus200 fixture");
        let model: GetFcmSubtraderBlockedCategoriesStatus200 =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus200", &output)
            .expect("GetFcmSubtraderBlockedCategoriesStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmSubtraderBlockedCategoriesStatus200");
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesStatus400"],
        &full["GetFcmSubtraderBlockedCategoriesStatus400"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus400", value)
            .expect("GetFcmSubtraderBlockedCategoriesStatus400 fixture");
        let model: GetFcmSubtraderBlockedCategoriesStatus400 =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus400", &output)
            .expect("GetFcmSubtraderBlockedCategoriesStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmSubtraderBlockedCategoriesStatus400");
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesStatus401"],
        &full["GetFcmSubtraderBlockedCategoriesStatus401"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus401", value)
            .expect("GetFcmSubtraderBlockedCategoriesStatus401 fixture");
        let model: GetFcmSubtraderBlockedCategoriesStatus401 =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus401", &output)
            .expect("GetFcmSubtraderBlockedCategoriesStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmSubtraderBlockedCategoriesStatus401");
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesStatus403"],
        &full["GetFcmSubtraderBlockedCategoriesStatus403"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus403", value)
            .expect("GetFcmSubtraderBlockedCategoriesStatus403 fixture");
        let model: GetFcmSubtraderBlockedCategoriesStatus403 =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus403", &output)
            .expect("GetFcmSubtraderBlockedCategoriesStatus403 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmSubtraderBlockedCategoriesStatus403");
    }
    for value in [
        &fixtures["GetFcmSubtraderBlockedCategoriesStatus500"],
        &full["GetFcmSubtraderBlockedCategoriesStatus500"],
    ] {
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus500", value)
            .expect("GetFcmSubtraderBlockedCategoriesStatus500 fixture");
        let model: GetFcmSubtraderBlockedCategoriesStatus500 =
            serde_json::from_value(value.clone())
                .expect("GetFcmSubtraderBlockedCategoriesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmSubtraderBlockedCategoriesStatus500", &output)
            .expect("GetFcmSubtraderBlockedCategoriesStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmSubtraderBlockedCategoriesStatus500");
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesBody"],
        &full["UpdateFcmSubtraderBlockedCategoriesBody"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesBody", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesBody fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesBody = serde_json::from_value(value.clone())
            .expect("UpdateFcmSubtraderBlockedCategoriesBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesBody", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesBody roundtrip");
        model_support::assert_presence(value, &output, "UpdateFcmSubtraderBlockedCategoriesBody");
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus200"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus200"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus200", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus200 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus200 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus200", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus200 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus200",
        );
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus400"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus400"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus400", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus400 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus400 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus400", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus400 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus400",
        );
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus401"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus401"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus401", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus401 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus401 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus401", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus401 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus401",
        );
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus403"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus403"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus403", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus403 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus403 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus403", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus403 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus403",
        );
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus404"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus404"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus404", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus404 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus404 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus404", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus404 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus404",
        );
    }
    for value in [
        &fixtures["UpdateFcmSubtraderBlockedCategoriesStatus500"],
        &full["UpdateFcmSubtraderBlockedCategoriesStatus500"],
    ] {
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus500", value)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus500 fixture");
        let model: UpdateFcmSubtraderBlockedCategoriesStatus500 =
            serde_json::from_value(value.clone())
                .expect("UpdateFcmSubtraderBlockedCategoriesStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("UpdateFcmSubtraderBlockedCategoriesStatus500", &output)
            .expect("UpdateFcmSubtraderBlockedCategoriesStatus500 roundtrip");
        model_support::assert_presence(
            value,
            &output,
            "UpdateFcmSubtraderBlockedCategoriesStatus500",
        );
    }
    for value in [
        &fixtures["GetTargetBalanceAllocationStatus200"],
        &full["GetTargetBalanceAllocationStatus200"],
    ] {
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus200", value)
            .expect("GetTargetBalanceAllocationStatus200 fixture");
        let model: GetTargetBalanceAllocationStatus200 = serde_json::from_value(value.clone())
            .expect("GetTargetBalanceAllocationStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus200", &output)
            .expect("GetTargetBalanceAllocationStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetTargetBalanceAllocationStatus200");
    }
    for value in [
        &fixtures["GetTargetBalanceAllocationStatus401"],
        &full["GetTargetBalanceAllocationStatus401"],
    ] {
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus401", value)
            .expect("GetTargetBalanceAllocationStatus401 fixture");
        let model: GetTargetBalanceAllocationStatus401 = serde_json::from_value(value.clone())
            .expect("GetTargetBalanceAllocationStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus401", &output)
            .expect("GetTargetBalanceAllocationStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetTargetBalanceAllocationStatus401");
    }
    for value in [
        &fixtures["GetTargetBalanceAllocationStatus403"],
        &full["GetTargetBalanceAllocationStatus403"],
    ] {
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus403", value)
            .expect("GetTargetBalanceAllocationStatus403 fixture");
        let model: GetTargetBalanceAllocationStatus403 = serde_json::from_value(value.clone())
            .expect("GetTargetBalanceAllocationStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus403", &output)
            .expect("GetTargetBalanceAllocationStatus403 roundtrip");
        model_support::assert_presence(value, &output, "GetTargetBalanceAllocationStatus403");
    }
    for value in [
        &fixtures["GetTargetBalanceAllocationStatus500"],
        &full["GetTargetBalanceAllocationStatus500"],
    ] {
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus500", value)
            .expect("GetTargetBalanceAllocationStatus500 fixture");
        let model: GetTargetBalanceAllocationStatus500 = serde_json::from_value(value.clone())
            .expect("GetTargetBalanceAllocationStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTargetBalanceAllocationStatus500", &output)
            .expect("GetTargetBalanceAllocationStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetTargetBalanceAllocationStatus500");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationBody"],
        &full["SetTargetBalanceAllocationBody"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationBody", value)
            .expect("SetTargetBalanceAllocationBody fixture");
        let model: SetTargetBalanceAllocationBody =
            serde_json::from_value(value.clone()).expect("SetTargetBalanceAllocationBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationBody", &output)
            .expect("SetTargetBalanceAllocationBody roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationBody");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationStatus200"],
        &full["SetTargetBalanceAllocationStatus200"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus200", value)
            .expect("SetTargetBalanceAllocationStatus200 fixture");
        let model: SetTargetBalanceAllocationStatus200 = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus200", &output)
            .expect("SetTargetBalanceAllocationStatus200 roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationStatus200");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationStatus400"],
        &full["SetTargetBalanceAllocationStatus400"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus400", value)
            .expect("SetTargetBalanceAllocationStatus400 fixture");
        let model: SetTargetBalanceAllocationStatus400 = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus400", &output)
            .expect("SetTargetBalanceAllocationStatus400 roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationStatus400");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationStatus401"],
        &full["SetTargetBalanceAllocationStatus401"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus401", value)
            .expect("SetTargetBalanceAllocationStatus401 fixture");
        let model: SetTargetBalanceAllocationStatus401 = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus401", &output)
            .expect("SetTargetBalanceAllocationStatus401 roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationStatus401");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationStatus403"],
        &full["SetTargetBalanceAllocationStatus403"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus403", value)
            .expect("SetTargetBalanceAllocationStatus403 fixture");
        let model: SetTargetBalanceAllocationStatus403 = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus403", &output)
            .expect("SetTargetBalanceAllocationStatus403 roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationStatus403");
    }
    for value in [
        &fixtures["SetTargetBalanceAllocationStatus500"],
        &full["SetTargetBalanceAllocationStatus500"],
    ] {
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus500", value)
            .expect("SetTargetBalanceAllocationStatus500 fixture");
        let model: SetTargetBalanceAllocationStatus500 = serde_json::from_value(value.clone())
            .expect("SetTargetBalanceAllocationStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("SetTargetBalanceAllocationStatus500", &output)
            .expect("SetTargetBalanceAllocationStatus500 roundtrip");
        model_support::assert_presence(value, &output, "SetTargetBalanceAllocationStatus500");
    }
    for value in [
        &fixtures["GetFcmPositionsParameters"],
        &full["GetFcmPositionsParameters"],
    ] {
        kalshi_client::typed::validate("GetFcmPositionsParameters", value)
            .expect("GetFcmPositionsParameters fixture");
        let model: GetFcmPositionsParameters =
            serde_json::from_value(value.clone()).expect("GetFcmPositionsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmPositionsParameters", &output)
            .expect("GetFcmPositionsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFcmPositionsParameters");
    }
    for value in [
        &fixtures["GetFcmPositionsStatus200"],
        &full["GetFcmPositionsStatus200"],
    ] {
        kalshi_client::typed::validate("GetFcmPositionsStatus200", value)
            .expect("GetFcmPositionsStatus200 fixture");
        let model: GetFcmPositionsStatus200 =
            serde_json::from_value(value.clone()).expect("GetFcmPositionsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFcmPositionsStatus200", &output)
            .expect("GetFcmPositionsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFcmPositionsStatus200");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferBody"],
        &full["CreateFcmPositionTransferBody"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferBody", value)
            .expect("CreateFcmPositionTransferBody fixture");
        let model: CreateFcmPositionTransferBody =
            serde_json::from_value(value.clone()).expect("CreateFcmPositionTransferBody decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferBody", &output)
            .expect("CreateFcmPositionTransferBody roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferBody");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus201"],
        &full["CreateFcmPositionTransferStatus201"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus201", value)
            .expect("CreateFcmPositionTransferStatus201 fixture");
        let model: CreateFcmPositionTransferStatus201 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus201 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus201", &output)
            .expect("CreateFcmPositionTransferStatus201 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus201");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus400"],
        &full["CreateFcmPositionTransferStatus400"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus400", value)
            .expect("CreateFcmPositionTransferStatus400 fixture");
        let model: CreateFcmPositionTransferStatus400 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus400", &output)
            .expect("CreateFcmPositionTransferStatus400 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus400");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus401"],
        &full["CreateFcmPositionTransferStatus401"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus401", value)
            .expect("CreateFcmPositionTransferStatus401 fixture");
        let model: CreateFcmPositionTransferStatus401 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus401", &output)
            .expect("CreateFcmPositionTransferStatus401 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus401");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus403"],
        &full["CreateFcmPositionTransferStatus403"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus403", value)
            .expect("CreateFcmPositionTransferStatus403 fixture");
        let model: CreateFcmPositionTransferStatus403 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus403 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus403", &output)
            .expect("CreateFcmPositionTransferStatus403 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus403");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus404"],
        &full["CreateFcmPositionTransferStatus404"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus404", value)
            .expect("CreateFcmPositionTransferStatus404 fixture");
        let model: CreateFcmPositionTransferStatus404 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus404", &output)
            .expect("CreateFcmPositionTransferStatus404 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus404");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus409"],
        &full["CreateFcmPositionTransferStatus409"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus409", value)
            .expect("CreateFcmPositionTransferStatus409 fixture");
        let model: CreateFcmPositionTransferStatus409 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus409 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus409", &output)
            .expect("CreateFcmPositionTransferStatus409 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus409");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus429"],
        &full["CreateFcmPositionTransferStatus429"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus429", value)
            .expect("CreateFcmPositionTransferStatus429 fixture");
        let model: CreateFcmPositionTransferStatus429 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus429 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus429", &output)
            .expect("CreateFcmPositionTransferStatus429 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus429");
    }
    for value in [
        &fixtures["CreateFcmPositionTransferStatus500"],
        &full["CreateFcmPositionTransferStatus500"],
    ] {
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus500", value)
            .expect("CreateFcmPositionTransferStatus500 fixture");
        let model: CreateFcmPositionTransferStatus500 = serde_json::from_value(value.clone())
            .expect("CreateFcmPositionTransferStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("CreateFcmPositionTransferStatus500", &output)
            .expect("CreateFcmPositionTransferStatus500 roundtrip");
        model_support::assert_presence(value, &output, "CreateFcmPositionTransferStatus500");
    }
    for value in [
        &fixtures["GetHistoricalCutoffStatus200"],
        &full["GetHistoricalCutoffStatus200"],
    ] {
        kalshi_client::typed::validate("GetHistoricalCutoffStatus200", value)
            .expect("GetHistoricalCutoffStatus200 fixture");
        let model: GetHistoricalCutoffStatus200 =
            serde_json::from_value(value.clone()).expect("GetHistoricalCutoffStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalCutoffStatus200", &output)
            .expect("GetHistoricalCutoffStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalCutoffStatus200");
    }
    for value in [
        &fixtures["GetMarketCandlesticksHistoricalParameters"],
        &full["GetMarketCandlesticksHistoricalParameters"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalParameters", value)
            .expect("GetMarketCandlesticksHistoricalParameters fixture");
        let model: GetMarketCandlesticksHistoricalParameters =
            serde_json::from_value(value.clone())
                .expect("GetMarketCandlesticksHistoricalParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalParameters", &output)
            .expect("GetMarketCandlesticksHistoricalParameters roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksHistoricalParameters");
    }
    for value in [
        &fixtures["GetMarketCandlesticksHistoricalStatus200"],
        &full["GetMarketCandlesticksHistoricalStatus200"],
    ] {
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalStatus200", value)
            .expect("GetMarketCandlesticksHistoricalStatus200 fixture");
        let model: GetMarketCandlesticksHistoricalStatus200 = serde_json::from_value(value.clone())
            .expect("GetMarketCandlesticksHistoricalStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetMarketCandlesticksHistoricalStatus200", &output)
            .expect("GetMarketCandlesticksHistoricalStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetMarketCandlesticksHistoricalStatus200");
    }
    for value in [
        &fixtures["GetFillsHistoricalParameters"],
        &full["GetFillsHistoricalParameters"],
    ] {
        kalshi_client::typed::validate("GetFillsHistoricalParameters", value)
            .expect("GetFillsHistoricalParameters fixture");
        let model: GetFillsHistoricalParameters =
            serde_json::from_value(value.clone()).expect("GetFillsHistoricalParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsHistoricalParameters", &output)
            .expect("GetFillsHistoricalParameters roundtrip");
        model_support::assert_presence(value, &output, "GetFillsHistoricalParameters");
    }
    for value in [
        &fixtures["GetFillsHistoricalStatus200"],
        &full["GetFillsHistoricalStatus200"],
    ] {
        kalshi_client::typed::validate("GetFillsHistoricalStatus200", value)
            .expect("GetFillsHistoricalStatus200 fixture");
        let model: GetFillsHistoricalStatus200 =
            serde_json::from_value(value.clone()).expect("GetFillsHistoricalStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsHistoricalStatus200", &output)
            .expect("GetFillsHistoricalStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetFillsHistoricalStatus200");
    }
    for value in [
        &fixtures["GetFillsHistoricalStatus404"],
        &full["GetFillsHistoricalStatus404"],
    ] {
        kalshi_client::typed::validate("GetFillsHistoricalStatus404", value)
            .expect("GetFillsHistoricalStatus404 fixture");
        let model: GetFillsHistoricalStatus404 =
            serde_json::from_value(value.clone()).expect("GetFillsHistoricalStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetFillsHistoricalStatus404", &output)
            .expect("GetFillsHistoricalStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetFillsHistoricalStatus404");
    }
    for value in [
        &fixtures["GetHistoricalOrdersParameters"],
        &full["GetHistoricalOrdersParameters"],
    ] {
        kalshi_client::typed::validate("GetHistoricalOrdersParameters", value)
            .expect("GetHistoricalOrdersParameters fixture");
        let model: GetHistoricalOrdersParameters =
            serde_json::from_value(value.clone()).expect("GetHistoricalOrdersParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalOrdersParameters", &output)
            .expect("GetHistoricalOrdersParameters roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalOrdersParameters");
    }
    for value in [
        &fixtures["GetHistoricalOrdersStatus200"],
        &full["GetHistoricalOrdersStatus200"],
    ] {
        kalshi_client::typed::validate("GetHistoricalOrdersStatus200", value)
            .expect("GetHistoricalOrdersStatus200 fixture");
        let model: GetHistoricalOrdersStatus200 =
            serde_json::from_value(value.clone()).expect("GetHistoricalOrdersStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalOrdersStatus200", &output)
            .expect("GetHistoricalOrdersStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalOrdersStatus200");
    }
    for value in [
        &fixtures["GetHistoricalOrdersStatus400"],
        &full["GetHistoricalOrdersStatus400"],
    ] {
        kalshi_client::typed::validate("GetHistoricalOrdersStatus400", value)
            .expect("GetHistoricalOrdersStatus400 fixture");
        let model: GetHistoricalOrdersStatus400 =
            serde_json::from_value(value.clone()).expect("GetHistoricalOrdersStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalOrdersStatus400", &output)
            .expect("GetHistoricalOrdersStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalOrdersStatus400");
    }
    for value in [
        &fixtures["GetHistoricalOrdersStatus401"],
        &full["GetHistoricalOrdersStatus401"],
    ] {
        kalshi_client::typed::validate("GetHistoricalOrdersStatus401", value)
            .expect("GetHistoricalOrdersStatus401 fixture");
        let model: GetHistoricalOrdersStatus401 =
            serde_json::from_value(value.clone()).expect("GetHistoricalOrdersStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalOrdersStatus401", &output)
            .expect("GetHistoricalOrdersStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalOrdersStatus401");
    }
    for value in [
        &fixtures["GetHistoricalOrdersStatus500"],
        &full["GetHistoricalOrdersStatus500"],
    ] {
        kalshi_client::typed::validate("GetHistoricalOrdersStatus500", value)
            .expect("GetHistoricalOrdersStatus500 fixture");
        let model: GetHistoricalOrdersStatus500 =
            serde_json::from_value(value.clone()).expect("GetHistoricalOrdersStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalOrdersStatus500", &output)
            .expect("GetHistoricalOrdersStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalOrdersStatus500");
    }
    for value in [
        &fixtures["GetHistoricalPositionsParameters"],
        &full["GetHistoricalPositionsParameters"],
    ] {
        kalshi_client::typed::validate("GetHistoricalPositionsParameters", value)
            .expect("GetHistoricalPositionsParameters fixture");
        let model: GetHistoricalPositionsParameters =
            serde_json::from_value(value.clone()).expect("GetHistoricalPositionsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalPositionsParameters", &output)
            .expect("GetHistoricalPositionsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalPositionsParameters");
    }
    for value in [
        &fixtures["GetHistoricalPositionsStatus200"],
        &full["GetHistoricalPositionsStatus200"],
    ] {
        kalshi_client::typed::validate("GetHistoricalPositionsStatus200", value)
            .expect("GetHistoricalPositionsStatus200 fixture");
        let model: GetHistoricalPositionsStatus200 =
            serde_json::from_value(value.clone()).expect("GetHistoricalPositionsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalPositionsStatus200", &output)
            .expect("GetHistoricalPositionsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalPositionsStatus200");
    }
    for value in [
        &fixtures["GetHistoricalPositionsStatus400"],
        &full["GetHistoricalPositionsStatus400"],
    ] {
        kalshi_client::typed::validate("GetHistoricalPositionsStatus400", value)
            .expect("GetHistoricalPositionsStatus400 fixture");
        let model: GetHistoricalPositionsStatus400 =
            serde_json::from_value(value.clone()).expect("GetHistoricalPositionsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalPositionsStatus400", &output)
            .expect("GetHistoricalPositionsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalPositionsStatus400");
    }
    for value in [
        &fixtures["GetHistoricalPositionsStatus401"],
        &full["GetHistoricalPositionsStatus401"],
    ] {
        kalshi_client::typed::validate("GetHistoricalPositionsStatus401", value)
            .expect("GetHistoricalPositionsStatus401 fixture");
        let model: GetHistoricalPositionsStatus401 =
            serde_json::from_value(value.clone()).expect("GetHistoricalPositionsStatus401 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalPositionsStatus401", &output)
            .expect("GetHistoricalPositionsStatus401 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalPositionsStatus401");
    }
    for value in [
        &fixtures["GetHistoricalPositionsStatus500"],
        &full["GetHistoricalPositionsStatus500"],
    ] {
        kalshi_client::typed::validate("GetHistoricalPositionsStatus500", value)
            .expect("GetHistoricalPositionsStatus500 fixture");
        let model: GetHistoricalPositionsStatus500 =
            serde_json::from_value(value.clone()).expect("GetHistoricalPositionsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalPositionsStatus500", &output)
            .expect("GetHistoricalPositionsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalPositionsStatus500");
    }
    for value in [
        &fixtures["GetTradesHistoricalParameters"],
        &full["GetTradesHistoricalParameters"],
    ] {
        kalshi_client::typed::validate("GetTradesHistoricalParameters", value)
            .expect("GetTradesHistoricalParameters fixture");
        let model: GetTradesHistoricalParameters =
            serde_json::from_value(value.clone()).expect("GetTradesHistoricalParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesHistoricalParameters", &output)
            .expect("GetTradesHistoricalParameters roundtrip");
        model_support::assert_presence(value, &output, "GetTradesHistoricalParameters");
    }
    for value in [
        &fixtures["GetTradesHistoricalStatus200"],
        &full["GetTradesHistoricalStatus200"],
    ] {
        kalshi_client::typed::validate("GetTradesHistoricalStatus200", value)
            .expect("GetTradesHistoricalStatus200 fixture");
        let model: GetTradesHistoricalStatus200 =
            serde_json::from_value(value.clone()).expect("GetTradesHistoricalStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesHistoricalStatus200", &output)
            .expect("GetTradesHistoricalStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetTradesHistoricalStatus200");
    }
    for value in [
        &fixtures["GetTradesHistoricalStatus400"],
        &full["GetTradesHistoricalStatus400"],
    ] {
        kalshi_client::typed::validate("GetTradesHistoricalStatus400", value)
            .expect("GetTradesHistoricalStatus400 fixture");
        let model: GetTradesHistoricalStatus400 =
            serde_json::from_value(value.clone()).expect("GetTradesHistoricalStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesHistoricalStatus400", &output)
            .expect("GetTradesHistoricalStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetTradesHistoricalStatus400");
    }
    for value in [
        &fixtures["GetTradesHistoricalStatus404"],
        &full["GetTradesHistoricalStatus404"],
    ] {
        kalshi_client::typed::validate("GetTradesHistoricalStatus404", value)
            .expect("GetTradesHistoricalStatus404 fixture");
        let model: GetTradesHistoricalStatus404 =
            serde_json::from_value(value.clone()).expect("GetTradesHistoricalStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesHistoricalStatus404", &output)
            .expect("GetTradesHistoricalStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetTradesHistoricalStatus404");
    }
    for value in [
        &fixtures["GetTradesHistoricalStatus500"],
        &full["GetTradesHistoricalStatus500"],
    ] {
        kalshi_client::typed::validate("GetTradesHistoricalStatus500", value)
            .expect("GetTradesHistoricalStatus500 fixture");
        let model: GetTradesHistoricalStatus500 =
            serde_json::from_value(value.clone()).expect("GetTradesHistoricalStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetTradesHistoricalStatus500", &output)
            .expect("GetTradesHistoricalStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetTradesHistoricalStatus500");
    }
    for value in [
        &fixtures["GetHistoricalMarketsParameters"],
        &full["GetHistoricalMarketsParameters"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketsParameters", value)
            .expect("GetHistoricalMarketsParameters fixture");
        let model: GetHistoricalMarketsParameters =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketsParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketsParameters", &output)
            .expect("GetHistoricalMarketsParameters roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketsParameters");
    }
    for value in [
        &fixtures["GetHistoricalMarketsStatus200"],
        &full["GetHistoricalMarketsStatus200"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketsStatus200", value)
            .expect("GetHistoricalMarketsStatus200 fixture");
        let model: GetHistoricalMarketsStatus200 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketsStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketsStatus200", &output)
            .expect("GetHistoricalMarketsStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketsStatus200");
    }
    for value in [
        &fixtures["GetHistoricalMarketsStatus400"],
        &full["GetHistoricalMarketsStatus400"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketsStatus400", value)
            .expect("GetHistoricalMarketsStatus400 fixture");
        let model: GetHistoricalMarketsStatus400 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketsStatus400 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketsStatus400", &output)
            .expect("GetHistoricalMarketsStatus400 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketsStatus400");
    }
    for value in [
        &fixtures["GetHistoricalMarketsStatus500"],
        &full["GetHistoricalMarketsStatus500"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketsStatus500", value)
            .expect("GetHistoricalMarketsStatus500 fixture");
        let model: GetHistoricalMarketsStatus500 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketsStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketsStatus500", &output)
            .expect("GetHistoricalMarketsStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketsStatus500");
    }
    for value in [
        &fixtures["GetHistoricalMarketParameters"],
        &full["GetHistoricalMarketParameters"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketParameters", value)
            .expect("GetHistoricalMarketParameters fixture");
        let model: GetHistoricalMarketParameters =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketParameters decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketParameters", &output)
            .expect("GetHistoricalMarketParameters roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketParameters");
    }
    for value in [
        &fixtures["GetHistoricalMarketStatus200"],
        &full["GetHistoricalMarketStatus200"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketStatus200", value)
            .expect("GetHistoricalMarketStatus200 fixture");
        let model: GetHistoricalMarketStatus200 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketStatus200 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketStatus200", &output)
            .expect("GetHistoricalMarketStatus200 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketStatus200");
    }
    for value in [
        &fixtures["GetHistoricalMarketStatus404"],
        &full["GetHistoricalMarketStatus404"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketStatus404", value)
            .expect("GetHistoricalMarketStatus404 fixture");
        let model: GetHistoricalMarketStatus404 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketStatus404 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketStatus404", &output)
            .expect("GetHistoricalMarketStatus404 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketStatus404");
    }
    for value in [
        &fixtures["GetHistoricalMarketStatus500"],
        &full["GetHistoricalMarketStatus500"],
    ] {
        kalshi_client::typed::validate("GetHistoricalMarketStatus500", value)
            .expect("GetHistoricalMarketStatus500 fixture");
        let model: GetHistoricalMarketStatus500 =
            serde_json::from_value(value.clone()).expect("GetHistoricalMarketStatus500 decode");
        let output = serde_json::to_value(model).unwrap();
        kalshi_client::typed::validate("GetHistoricalMarketStatus500", &output)
            .expect("GetHistoricalMarketStatus500 roundtrip");
        model_support::assert_presence(value, &output, "GetHistoricalMarketStatus500");
    }
}
#[test]
fn recursive_null_mutations_obey_the_source_schema() {
    let full: Value = serde_json::from_str(include_str!("typed-model-full-fixtures.json")).unwrap();
    let mut mutations = 0;
    mutations += model_support::assert_nulls::<FixedPointDollars>(
        "FixedPointDollars",
        &full["FixedPointDollars"],
    );
    mutations +=
        model_support::assert_nulls::<FixedPointCount>("FixedPointCount", &full["FixedPointCount"]);
    mutations +=
        model_support::assert_nulls::<ExchangeIndex>("ExchangeIndex", &full["ExchangeIndex"]);
    mutations += model_support::assert_nulls::<FeeType>("FeeType", &full["FeeType"]);
    mutations += model_support::assert_nulls::<GetMarketCandlesticksHistoricalResponse>(
        "GetMarketCandlesticksHistoricalResponse",
        &full["GetMarketCandlesticksHistoricalResponse"],
    );
    mutations += model_support::assert_nulls::<MarketCandlestickHistorical>(
        "MarketCandlestickHistorical",
        &full["MarketCandlestickHistorical"],
    );
    mutations += model_support::assert_nulls::<BidAskDistributionHistorical>(
        "BidAskDistributionHistorical",
        &full["BidAskDistributionHistorical"],
    );
    mutations += model_support::assert_nulls::<PriceDistributionHistorical>(
        "PriceDistributionHistorical",
        &full["PriceDistributionHistorical"],
    );
    mutations +=
        model_support::assert_nulls::<ErrorResponse>("ErrorResponse", &full["ErrorResponse"]);
    mutations += model_support::assert_nulls::<SelfTradePreventionType>(
        "SelfTradePreventionType",
        &full["SelfTradePreventionType"],
    );
    mutations += model_support::assert_nulls::<BookSide>("BookSide", &full["BookSide"]);
    mutations += model_support::assert_nulls::<OrderStatus>("OrderStatus", &full["OrderStatus"]);
    mutations += model_support::assert_nulls::<ExchangeInstance>(
        "ExchangeInstance",
        &full["ExchangeInstance"],
    );
    mutations += model_support::assert_nulls::<UserFilter>("UserFilter", &full["UserFilter"]);
    mutations += model_support::assert_nulls::<ApiKeyScope>("ApiKeyScope", &full["ApiKeyScope"]);
    mutations += model_support::assert_nulls::<ApiKeyType>("ApiKeyType", &full["ApiKeyType"]);
    mutations += model_support::assert_nulls::<ApiKey>("ApiKey", &full["ApiKey"]);
    mutations += model_support::assert_nulls::<GetApiKeysResponse>(
        "GetApiKeysResponse",
        &full["GetApiKeysResponse"],
    );
    mutations += model_support::assert_nulls::<CreateApiKeyRequest>(
        "CreateApiKeyRequest",
        &full["CreateApiKeyRequest"],
    );
    mutations += model_support::assert_nulls::<CreateApiKeyResponse>(
        "CreateApiKeyResponse",
        &full["CreateApiKeyResponse"],
    );
    mutations += model_support::assert_nulls::<GenerateApiKeyRequest>(
        "GenerateApiKeyRequest",
        &full["GenerateApiKeyRequest"],
    );
    mutations += model_support::assert_nulls::<GenerateApiKeyResponse>(
        "GenerateApiKeyResponse",
        &full["GenerateApiKeyResponse"],
    );
    mutations += model_support::assert_nulls::<GetTagsForSeriesCategoriesResponse>(
        "GetTagsForSeriesCategoriesResponse",
        &full["GetTagsForSeriesCategoriesResponse"],
    );
    mutations += model_support::assert_nulls::<ScopeList>("ScopeList", &full["ScopeList"]);
    mutations += model_support::assert_nulls::<SportFilterDetails>(
        "SportFilterDetails",
        &full["SportFilterDetails"],
    );
    mutations += model_support::assert_nulls::<GetFiltersBySportsResponse>(
        "GetFiltersBySportsResponse",
        &full["GetFiltersBySportsResponse"],
    );
    mutations += model_support::assert_nulls::<BucketLimit>("BucketLimit", &full["BucketLimit"]);
    mutations += model_support::assert_nulls::<GetAccountApiLimitsResponse>(
        "GetAccountApiLimitsResponse",
        &full["GetAccountApiLimitsResponse"],
    );
    mutations += model_support::assert_nulls::<ApiUsageLevelGrant>(
        "ApiUsageLevelGrant",
        &full["ApiUsageLevelGrant"],
    );
    mutations += model_support::assert_nulls::<GetAccountApiUsageLevelVolumeProgressResponse>(
        "GetAccountApiUsageLevelVolumeProgressResponse",
        &full["GetAccountApiUsageLevelVolumeProgressResponse"],
    );
    mutations += model_support::assert_nulls::<AccountApiUsageLevelVolumeProgress>(
        "AccountApiUsageLevelVolumeProgress",
        &full["AccountApiUsageLevelVolumeProgress"],
    );
    mutations += model_support::assert_nulls::<AccountApiUsageLevelVolumeGoal>(
        "AccountApiUsageLevelVolumeGoal",
        &full["AccountApiUsageLevelVolumeGoal"],
    );
    mutations += model_support::assert_nulls::<EndpointTokenCost>(
        "EndpointTokenCost",
        &full["EndpointTokenCost"],
    );
    mutations += model_support::assert_nulls::<GetAccountEndpointCostsResponse>(
        "GetAccountEndpointCostsResponse",
        &full["GetAccountEndpointCostsResponse"],
    );
    mutations +=
        model_support::assert_nulls::<ExchangeStatus>("ExchangeStatus", &full["ExchangeStatus"]);
    mutations += model_support::assert_nulls::<ExchangeIndexStatus>(
        "ExchangeIndexStatus",
        &full["ExchangeIndexStatus"],
    );
    mutations += model_support::assert_nulls::<GetExchangeScheduleResponse>(
        "GetExchangeScheduleResponse",
        &full["GetExchangeScheduleResponse"],
    );
    mutations += model_support::assert_nulls::<Schedule>("Schedule", &full["Schedule"]);
    mutations +=
        model_support::assert_nulls::<WeeklySchedule>("WeeklySchedule", &full["WeeklySchedule"]);
    mutations +=
        model_support::assert_nulls::<DailySchedule>("DailySchedule", &full["DailySchedule"]);
    mutations += model_support::assert_nulls::<MaintenanceWindow>(
        "MaintenanceWindow",
        &full["MaintenanceWindow"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalCutoffResponse>(
        "GetHistoricalCutoffResponse",
        &full["GetHistoricalCutoffResponse"],
    );
    mutations += model_support::assert_nulls::<GetUserDataTimestampResponse>(
        "GetUserDataTimestampResponse",
        &full["GetUserDataTimestampResponse"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksResponse>(
        "GetMarketCandlesticksResponse",
        &full["GetMarketCandlesticksResponse"],
    );
    mutations += model_support::assert_nulls::<GetEventCandlesticksResponse>(
        "GetEventCandlesticksResponse",
        &full["GetEventCandlesticksResponse"],
    );
    mutations += model_support::assert_nulls::<BatchGetMarketCandlesticksResponse>(
        "BatchGetMarketCandlesticksResponse",
        &full["BatchGetMarketCandlesticksResponse"],
    );
    mutations += model_support::assert_nulls::<MarketCandlesticksResponse>(
        "MarketCandlesticksResponse",
        &full["MarketCandlesticksResponse"],
    );
    mutations += model_support::assert_nulls::<MarketCandlestick>(
        "MarketCandlestick",
        &full["MarketCandlestick"],
    );
    mutations += model_support::assert_nulls::<BidAskDistribution>(
        "BidAskDistribution",
        &full["BidAskDistribution"],
    );
    mutations += model_support::assert_nulls::<PriceDistribution>(
        "PriceDistribution",
        &full["PriceDistribution"],
    );
    mutations += model_support::assert_nulls::<LiveData>("LiveData", &full["LiveData"]);
    mutations += model_support::assert_nulls::<GetLiveDataResponse>(
        "GetLiveDataResponse",
        &full["GetLiveDataResponse"],
    );
    mutations += model_support::assert_nulls::<GetLiveDatasResponse>(
        "GetLiveDatasResponse",
        &full["GetLiveDatasResponse"],
    );
    mutations +=
        model_support::assert_nulls::<EventLiveData>("EventLiveData", &full["EventLiveData"]);
    mutations += model_support::assert_nulls::<GetEventLiveDataResponse>(
        "GetEventLiveDataResponse",
        &full["GetEventLiveDataResponse"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexResponse>(
        "GetWeatherIndexResponse",
        &full["GetWeatherIndexResponse"],
    );
    mutations += model_support::assert_nulls::<WeatherIndexPoint>(
        "WeatherIndexPoint",
        &full["WeatherIndexPoint"],
    );
    mutations += model_support::assert_nulls::<WeatherIndexStationReading>(
        "WeatherIndexStationReading",
        &full["WeatherIndexStationReading"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexCalibrationsResponse>(
        "GetWeatherIndexCalibrationsResponse",
        &full["GetWeatherIndexCalibrationsResponse"],
    );
    mutations += model_support::assert_nulls::<WeatherIndexCalibration>(
        "WeatherIndexCalibration",
        &full["WeatherIndexCalibration"],
    );
    mutations += model_support::assert_nulls::<WeatherIndexCalibrationStation>(
        "WeatherIndexCalibrationStation",
        &full["WeatherIndexCalibrationStation"],
    );
    mutations += model_support::assert_nulls::<GetGameStatsResponse>(
        "GetGameStatsResponse",
        &full["GetGameStatsResponse"],
    );
    mutations += model_support::assert_nulls::<PlayByPlay>("PlayByPlay", &full["PlayByPlay"]);
    mutations +=
        model_support::assert_nulls::<IndexedBalance>("IndexedBalance", &full["IndexedBalance"]);
    mutations += model_support::assert_nulls::<GetBalanceResponse>(
        "GetBalanceResponse",
        &full["GetBalanceResponse"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountRequest>(
        "CreateSubaccountRequest",
        &full["CreateSubaccountRequest"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountResponse>(
        "CreateSubaccountResponse",
        &full["CreateSubaccountResponse"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferRequest>(
        "ApplySubaccountTransferRequest",
        &full["ApplySubaccountTransferRequest"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferResponse>(
        "ApplySubaccountTransferResponse",
        &full["ApplySubaccountTransferResponse"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountBalancesResponse>(
        "GetSubaccountBalancesResponse",
        &full["GetSubaccountBalancesResponse"],
    );
    mutations += model_support::assert_nulls::<SubaccountBalance>(
        "SubaccountBalance",
        &full["SubaccountBalance"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountTransfersResponse>(
        "GetSubaccountTransfersResponse",
        &full["GetSubaccountTransfersResponse"],
    );
    mutations += model_support::assert_nulls::<SubaccountTransfer>(
        "SubaccountTransfer",
        &full["SubaccountTransfer"],
    );
    mutations += model_support::assert_nulls::<UpdateSubaccountNettingRequest>(
        "UpdateSubaccountNettingRequest",
        &full["UpdateSubaccountNettingRequest"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountNettingResponse>(
        "GetSubaccountNettingResponse",
        &full["GetSubaccountNettingResponse"],
    );
    mutations += model_support::assert_nulls::<SubaccountNettingConfig>(
        "SubaccountNettingConfig",
        &full["SubaccountNettingConfig"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsResponse>(
        "GetSettlementsResponse",
        &full["GetSettlementsResponse"],
    );
    mutations += model_support::assert_nulls::<Settlement>("Settlement", &full["Settlement"]);
    mutations += model_support::assert_nulls::<GetPortfolioRestingOrderTotalValueResponse>(
        "GetPortfolioRestingOrderTotalValueResponse",
        &full["GetPortfolioRestingOrderTotalValueResponse"],
    );
    mutations += model_support::assert_nulls::<GetDepositsResponse>(
        "GetDepositsResponse",
        &full["GetDepositsResponse"],
    );
    mutations += model_support::assert_nulls::<Deposit>("Deposit", &full["Deposit"]);
    mutations += model_support::assert_nulls::<GetWithdrawalsResponse>(
        "GetWithdrawalsResponse",
        &full["GetWithdrawalsResponse"],
    );
    mutations += model_support::assert_nulls::<Withdrawal>("Withdrawal", &full["Withdrawal"]);
    mutations += model_support::assert_nulls::<Order>("Order", &full["Order"]);
    mutations += model_support::assert_nulls::<Milestone>("Milestone", &full["Milestone"]);
    mutations += model_support::assert_nulls::<GetMilestoneResponse>(
        "GetMilestoneResponse",
        &full["GetMilestoneResponse"],
    );
    mutations += model_support::assert_nulls::<GetMilestonesResponse>(
        "GetMilestonesResponse",
        &full["GetMilestonesResponse"],
    );
    mutations += model_support::assert_nulls::<GetOrdersResponse>(
        "GetOrdersResponse",
        &full["GetOrdersResponse"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionResponse>(
        "GetOrderQueuePositionResponse",
        &full["GetOrderQueuePositionResponse"],
    );
    mutations += model_support::assert_nulls::<OrderQueuePosition>(
        "OrderQueuePosition",
        &full["OrderQueuePosition"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsResponse>(
        "GetOrderQueuePositionsResponse",
        &full["GetOrderQueuePositionsResponse"],
    );
    mutations +=
        model_support::assert_nulls::<MarketPosition>("MarketPosition", &full["MarketPosition"]);
    mutations +=
        model_support::assert_nulls::<EventPosition>("EventPosition", &full["EventPosition"]);
    mutations += model_support::assert_nulls::<GetPositionsResponse>(
        "GetPositionsResponse",
        &full["GetPositionsResponse"],
    );
    mutations += model_support::assert_nulls::<Trade>("Trade", &full["Trade"]);
    mutations += model_support::assert_nulls::<GetIncentiveProgramsResponse>(
        "GetIncentiveProgramsResponse",
        &full["GetIncentiveProgramsResponse"],
    );
    mutations += model_support::assert_nulls::<IncentiveProgram>(
        "IncentiveProgram",
        &full["IncentiveProgram"],
    );
    mutations += model_support::assert_nulls::<GetTradesResponse>(
        "GetTradesResponse",
        &full["GetTradesResponse"],
    );
    mutations += model_support::assert_nulls::<OutcomeSide>("OutcomeSide", &full["OutcomeSide"]);
    mutations += model_support::assert_nulls::<Fill>("Fill", &full["Fill"]);
    mutations += model_support::assert_nulls::<FcmFill>("FcmFill", &full["FcmFill"]);
    mutations += model_support::assert_nulls::<GetFillsResponse>(
        "GetFillsResponse",
        &full["GetFillsResponse"],
    );
    mutations += model_support::assert_nulls::<GetFcmFillsResponse>(
        "GetFcmFillsResponse",
        &full["GetFcmFillsResponse"],
    );
    mutations += model_support::assert_nulls::<StructuredTarget>(
        "StructuredTarget",
        &full["StructuredTarget"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetsResponse>(
        "GetStructuredTargetsResponse",
        &full["GetStructuredTargetsResponse"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetResponse>(
        "GetStructuredTargetResponse",
        &full["GetStructuredTargetResponse"],
    );
    mutations += model_support::assert_nulls::<FcmSubtrader>("FCMSubtrader", &full["FCMSubtrader"]);
    mutations += model_support::assert_nulls::<ListFcmSubtradersResponse>(
        "ListFCMSubtradersResponse",
        &full["ListFCMSubtradersResponse"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderRequest>(
        "CreateFCMSubtraderRequest",
        &full["CreateFCMSubtraderRequest"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderResponse>(
        "CreateFCMSubtraderResponse",
        &full["CreateFCMSubtraderResponse"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferRequest>(
        "CreateFCMPositionTransferRequest",
        &full["CreateFCMPositionTransferRequest"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferResponse>(
        "CreateFCMPositionTransferResponse",
        &full["CreateFCMPositionTransferResponse"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapResponse>(
        "GetFCMEventContractDailyCapResponse",
        &full["GetFCMEventContractDailyCapResponse"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapRequest>(
        "UpdateFCMEventContractDailyCapRequest",
        &full["UpdateFCMEventContractDailyCapRequest"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesResponse>(
        "GetFCMSubtraderBlockedCategoriesResponse",
        &full["GetFCMSubtraderBlockedCategoriesResponse"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesRequest>(
        "UpdateFCMSubtraderBlockedCategoriesRequest",
        &full["UpdateFCMSubtraderBlockedCategoriesRequest"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesResponse>(
        "UpdateFCMSubtraderBlockedCategoriesResponse",
        &full["UpdateFCMSubtraderBlockedCategoriesResponse"],
    );
    mutations +=
        model_support::assert_nulls::<EmptyResponse>("EmptyResponse", &full["EmptyResponse"]);
    mutations += model_support::assert_nulls::<TargetBalanceAllocation>(
        "TargetBalanceAllocation",
        &full["TargetBalanceAllocation"],
    );
    mutations += model_support::assert_nulls::<TargetBalanceAllocationInput>(
        "TargetBalanceAllocationInput",
        &full["TargetBalanceAllocationInput"],
    );
    mutations += model_support::assert_nulls::<RestingMarginReservation>(
        "RestingMarginReservation",
        &full["RestingMarginReservation"],
    );
    mutations += model_support::assert_nulls::<GetTargetBalanceAllocationResponse>(
        "GetTargetBalanceAllocationResponse",
        &full["GetTargetBalanceAllocationResponse"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationRequest>(
        "SetTargetBalanceAllocationRequest",
        &full["SetTargetBalanceAllocationRequest"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferRequest>(
        "IntraExchangeInstanceTransferRequest",
        &full["IntraExchangeInstanceTransferRequest"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferResponse>(
        "IntraExchangeInstanceTransferResponse",
        &full["IntraExchangeInstanceTransferResponse"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus>(
        "IntraExchangeInstanceTransferStatus",
        &full["IntraExchangeInstanceTransferStatus"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransfer>(
        "IntraExchangeInstanceTransfer",
        &full["IntraExchangeInstanceTransfer"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersResponse>(
        "GetIntraExchangeInstanceTransfersResponse",
        &full["GetIntraExchangeInstanceTransfersResponse"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferResponse>(
        "GetIntraExchangeInstanceTransferResponse",
        &full["GetIntraExchangeInstanceTransferResponse"],
    );
    mutations += model_support::assert_nulls::<OrderGroup>("OrderGroup", &full["OrderGroup"]);
    mutations += model_support::assert_nulls::<GetOrderGroupsResponse>(
        "GetOrderGroupsResponse",
        &full["GetOrderGroupsResponse"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupResponse>(
        "GetOrderGroupResponse",
        &full["GetOrderGroupResponse"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupRequest>(
        "CreateOrderGroupRequest",
        &full["CreateOrderGroupRequest"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitRequest>(
        "UpdateOrderGroupLimitRequest",
        &full["UpdateOrderGroupLimitRequest"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupResponse>(
        "CreateOrderGroupResponse",
        &full["CreateOrderGroupResponse"],
    );
    mutations += model_support::assert_nulls::<GetCommunicationsIdResponse>(
        "GetCommunicationsIDResponse",
        &full["GetCommunicationsIDResponse"],
    );
    mutations += model_support::assert_nulls::<BlockTradeProposal>(
        "BlockTradeProposal",
        &full["BlockTradeProposal"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsResponse>(
        "GetBlockTradeProposalsResponse",
        &full["GetBlockTradeProposalsResponse"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeRequest>(
        "ProposeBlockTradeRequest",
        &full["ProposeBlockTradeRequest"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeResponse>(
        "ProposeBlockTradeResponse",
        &full["ProposeBlockTradeResponse"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalRequest>(
        "AcceptBlockTradeProposalRequest",
        &full["AcceptBlockTradeProposalRequest"],
    );
    mutations += model_support::assert_nulls::<Rfq>("RFQ", &full["RFQ"]);
    mutations +=
        model_support::assert_nulls::<GetRfQsResponse>("GetRFQsResponse", &full["GetRFQsResponse"]);
    mutations +=
        model_support::assert_nulls::<GetRfqResponse>("GetRFQResponse", &full["GetRFQResponse"]);
    mutations += model_support::assert_nulls::<CreateRfqRequest>(
        "CreateRFQRequest",
        &full["CreateRFQRequest"],
    );
    mutations += model_support::assert_nulls::<CreateRfqResponse>(
        "CreateRFQResponse",
        &full["CreateRFQResponse"],
    );
    mutations += model_support::assert_nulls::<Quote>("Quote", &full["Quote"]);
    mutations += model_support::assert_nulls::<GetQuotesResponse>(
        "GetQuotesResponse",
        &full["GetQuotesResponse"],
    );
    mutations += model_support::assert_nulls::<GetQuoteResponse>(
        "GetQuoteResponse",
        &full["GetQuoteResponse"],
    );
    mutations += model_support::assert_nulls::<CreateQuoteRequest>(
        "CreateQuoteRequest",
        &full["CreateQuoteRequest"],
    );
    mutations += model_support::assert_nulls::<CreateQuoteResponse>(
        "CreateQuoteResponse",
        &full["CreateQuoteResponse"],
    );
    mutations += model_support::assert_nulls::<AcceptQuoteRequest>(
        "AcceptQuoteRequest",
        &full["AcceptQuoteRequest"],
    );
    mutations += model_support::assert_nulls::<GetOrderResponse>(
        "GetOrderResponse",
        &full["GetOrderResponse"],
    );
    mutations += model_support::assert_nulls::<CreateOrderRequest>(
        "CreateOrderRequest",
        &full["CreateOrderRequest"],
    );
    mutations += model_support::assert_nulls::<CreateOrderResponse>(
        "CreateOrderResponse",
        &full["CreateOrderResponse"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersRequest>(
        "BatchCreateOrdersRequest",
        &full["BatchCreateOrdersRequest"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersResponse>(
        "BatchCreateOrdersResponse",
        &full["BatchCreateOrdersResponse"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersIndividualResponse>(
        "BatchCreateOrdersIndividualResponse",
        &full["BatchCreateOrdersIndividualResponse"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersRequest>(
        "BatchCancelOrdersRequest",
        &full["BatchCancelOrdersRequest"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersRequestOrder>(
        "BatchCancelOrdersRequestOrder",
        &full["BatchCancelOrdersRequestOrder"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersResponse>(
        "BatchCancelOrdersResponse",
        &full["BatchCancelOrdersResponse"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersIndividualResponse>(
        "BatchCancelOrdersIndividualResponse",
        &full["BatchCancelOrdersIndividualResponse"],
    );
    mutations += model_support::assert_nulls::<AmendOrderRequest>(
        "AmendOrderRequest",
        &full["AmendOrderRequest"],
    );
    mutations += model_support::assert_nulls::<AmendOrderResponse>(
        "AmendOrderResponse",
        &full["AmendOrderResponse"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Request>(
        "CreateOrderV2Request",
        &full["CreateOrderV2Request"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Response>(
        "CreateOrderV2Response",
        &full["CreateOrderV2Response"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Response>(
        "CancelOrderV2Response",
        &full["CancelOrderV2Response"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Request>(
        "DecreaseOrderV2Request",
        &full["DecreaseOrderV2Request"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Response>(
        "DecreaseOrderV2Response",
        &full["DecreaseOrderV2Response"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Request>(
        "AmendOrderV2Request",
        &full["AmendOrderV2Request"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Response>(
        "AmendOrderV2Response",
        &full["AmendOrderV2Response"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Request>(
        "BatchCreateOrdersV2Request",
        &full["BatchCreateOrdersV2Request"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Response>(
        "BatchCreateOrdersV2Response",
        &full["BatchCreateOrdersV2Response"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Request>(
        "BatchCancelOrdersV2Request",
        &full["BatchCancelOrdersV2Request"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Response>(
        "BatchCancelOrdersV2Response",
        &full["BatchCancelOrdersV2Response"],
    );
    mutations +=
        model_support::assert_nulls::<AssociatedEvent>("AssociatedEvent", &full["AssociatedEvent"]);
    mutations += model_support::assert_nulls::<MultivariateEventCollection>(
        "MultivariateEventCollection",
        &full["MultivariateEventCollection"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionResponse>(
        "GetMultivariateEventCollectionResponse",
        &full["GetMultivariateEventCollectionResponse"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionsResponse>(
        "GetMultivariateEventCollectionsResponse",
        &full["GetMultivariateEventCollectionsResponse"],
    );
    mutations += model_support::assert_nulls::<TickerPair>("TickerPair", &full["TickerPair"]);
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionRequest>(
        "CreateMarketInMultivariateEventCollectionRequest",
        &full["CreateMarketInMultivariateEventCollectionRequest"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionResponse>(
        "CreateMarketInMultivariateEventCollectionResponse",
        &full["CreateMarketInMultivariateEventCollectionResponse"],
    );
    mutations += model_support::assert_nulls::<PriceLevelDollarsCountFp>(
        "PriceLevelDollarsCountFp",
        &full["PriceLevelDollarsCountFp"],
    );
    mutations += model_support::assert_nulls::<OrderbookCountFp>(
        "OrderbookCountFp",
        &full["OrderbookCountFp"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksResponse>(
        "GetMarketOrderbooksResponse",
        &full["GetMarketOrderbooksResponse"],
    );
    mutations += model_support::assert_nulls::<MarketOrderbookFp>(
        "MarketOrderbookFp",
        &full["MarketOrderbookFp"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookResponse>(
        "GetMarketOrderbookResponse",
        &full["GetMarketOrderbookResponse"],
    );
    mutations += model_support::assert_nulls::<GetEventsResponse>(
        "GetEventsResponse",
        &full["GetEventsResponse"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventsResponse>(
        "GetMultivariateEventsResponse",
        &full["GetMultivariateEventsResponse"],
    );
    mutations +=
        model_support::assert_nulls::<EventFeeChange>("EventFeeChange", &full["EventFeeChange"]);
    mutations += model_support::assert_nulls::<GetEventFeeChangesResponse>(
        "GetEventFeeChangesResponse",
        &full["GetEventFeeChangesResponse"],
    );
    mutations += model_support::assert_nulls::<GetEventResponse>(
        "GetEventResponse",
        &full["GetEventResponse"],
    );
    mutations +=
        model_support::assert_nulls::<MarketMetadata>("MarketMetadata", &full["MarketMetadata"]);
    mutations += model_support::assert_nulls::<GetEventMetadataResponse>(
        "GetEventMetadataResponse",
        &full["GetEventMetadataResponse"],
    );
    mutations += model_support::assert_nulls::<GetEventForecastPercentilesHistoryResponse>(
        "GetEventForecastPercentilesHistoryResponse",
        &full["GetEventForecastPercentilesHistoryResponse"],
    );
    mutations += model_support::assert_nulls::<ForecastPercentilesPoint>(
        "ForecastPercentilesPoint",
        &full["ForecastPercentilesPoint"],
    );
    mutations +=
        model_support::assert_nulls::<PercentilePoint>("PercentilePoint", &full["PercentilePoint"]);
    mutations += model_support::assert_nulls::<EventData>("EventData", &full["EventData"]);
    mutations += model_support::assert_nulls::<Series>("Series", &full["Series"]);
    mutations +=
        model_support::assert_nulls::<SeriesFeeChange>("SeriesFeeChange", &full["SeriesFeeChange"]);
    mutations += model_support::assert_nulls::<GetSeriesResponse>(
        "GetSeriesResponse",
        &full["GetSeriesResponse"],
    );
    mutations += model_support::assert_nulls::<GetSeriesListResponse>(
        "GetSeriesListResponse",
        &full["GetSeriesListResponse"],
    );
    mutations += model_support::assert_nulls::<GetSeriesFeeChangesResponse>(
        "GetSeriesFeeChangesResponse",
        &full["GetSeriesFeeChangesResponse"],
    );
    mutations += model_support::assert_nulls::<SettlementSource>(
        "SettlementSource",
        &full["SettlementSource"],
    );
    mutations += model_support::assert_nulls::<GetMarketsResponse>(
        "GetMarketsResponse",
        &full["GetMarketsResponse"],
    );
    mutations += model_support::assert_nulls::<GetMarketResponse>(
        "GetMarketResponse",
        &full["GetMarketResponse"],
    );
    mutations +=
        model_support::assert_nulls::<MveSelectedLeg>("MveSelectedLeg", &full["MveSelectedLeg"]);
    mutations += model_support::assert_nulls::<PriceRange>("PriceRange", &full["PriceRange"]);
    mutations += model_support::assert_nulls::<Market>("Market", &full["Market"]);
    mutations += model_support::assert_nulls::<GetExchangeStatusStatus200>(
        "GetExchangeStatusStatus200",
        &full["GetExchangeStatusStatus200"],
    );
    mutations += model_support::assert_nulls::<GetExchangeStatusStatus500>(
        "GetExchangeStatusStatus500",
        &full["GetExchangeStatusStatus500"],
    );
    mutations += model_support::assert_nulls::<GetExchangeStatusStatus503>(
        "GetExchangeStatusStatus503",
        &full["GetExchangeStatusStatus503"],
    );
    mutations += model_support::assert_nulls::<GetExchangeStatusStatus504>(
        "GetExchangeStatusStatus504",
        &full["GetExchangeStatusStatus504"],
    );
    mutations += model_support::assert_nulls::<GetSeriesFeeChangesParameters>(
        "GetSeriesFeeChangesParameters",
        &full["GetSeriesFeeChangesParameters"],
    );
    mutations += model_support::assert_nulls::<GetSeriesFeeChangesStatus200>(
        "GetSeriesFeeChangesStatus200",
        &full["GetSeriesFeeChangesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSeriesFeeChangesStatus400>(
        "GetSeriesFeeChangesStatus400",
        &full["GetSeriesFeeChangesStatus400"],
    );
    mutations += model_support::assert_nulls::<GetSeriesFeeChangesStatus500>(
        "GetSeriesFeeChangesStatus500",
        &full["GetSeriesFeeChangesStatus500"],
    );
    mutations += model_support::assert_nulls::<GetExchangeScheduleStatus200>(
        "GetExchangeScheduleStatus200",
        &full["GetExchangeScheduleStatus200"],
    );
    mutations += model_support::assert_nulls::<GetUserDataTimestampStatus200>(
        "GetUserDataTimestampStatus200",
        &full["GetUserDataTimestampStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksParameters>(
        "GetMarketCandlesticksParameters",
        &full["GetMarketCandlesticksParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksStatus200>(
        "GetMarketCandlesticksStatus200",
        &full["GetMarketCandlesticksStatus200"],
    );
    mutations += model_support::assert_nulls::<GetTradesParameters>(
        "GetTradesParameters",
        &full["GetTradesParameters"],
    );
    mutations += model_support::assert_nulls::<GetTradesStatus200>(
        "GetTradesStatus200",
        &full["GetTradesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookParameters>(
        "GetMarketOrderbookParameters",
        &full["GetMarketOrderbookParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookStatus200>(
        "GetMarketOrderbookStatus200",
        &full["GetMarketOrderbookStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookStatus401>(
        "GetMarketOrderbookStatus401",
        &full["GetMarketOrderbookStatus401"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookStatus404>(
        "GetMarketOrderbookStatus404",
        &full["GetMarketOrderbookStatus404"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbookStatus500>(
        "GetMarketOrderbookStatus500",
        &full["GetMarketOrderbookStatus500"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksParameters>(
        "GetMarketOrderbooksParameters",
        &full["GetMarketOrderbooksParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksStatus200>(
        "GetMarketOrderbooksStatus200",
        &full["GetMarketOrderbooksStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksStatus400>(
        "GetMarketOrderbooksStatus400",
        &full["GetMarketOrderbooksStatus400"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksStatus401>(
        "GetMarketOrderbooksStatus401",
        &full["GetMarketOrderbooksStatus401"],
    );
    mutations += model_support::assert_nulls::<GetMarketOrderbooksStatus500>(
        "GetMarketOrderbooksStatus500",
        &full["GetMarketOrderbooksStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSeriesParameters>(
        "GetSeriesParameters",
        &full["GetSeriesParameters"],
    );
    mutations += model_support::assert_nulls::<GetSeriesStatus200>(
        "GetSeriesStatus200",
        &full["GetSeriesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSeriesStatus400>(
        "GetSeriesStatus400",
        &full["GetSeriesStatus400"],
    );
    mutations += model_support::assert_nulls::<GetSeriesStatus500>(
        "GetSeriesStatus500",
        &full["GetSeriesStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSeriesListParameters>(
        "GetSeriesListParameters",
        &full["GetSeriesListParameters"],
    );
    mutations += model_support::assert_nulls::<GetSeriesListStatus200>(
        "GetSeriesListStatus200",
        &full["GetSeriesListStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSeriesListStatus400>(
        "GetSeriesListStatus400",
        &full["GetSeriesListStatus400"],
    );
    mutations += model_support::assert_nulls::<GetSeriesListStatus500>(
        "GetSeriesListStatus500",
        &full["GetSeriesListStatus500"],
    );
    mutations += model_support::assert_nulls::<GetMarketsParameters>(
        "GetMarketsParameters",
        &full["GetMarketsParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketsStatus200>(
        "GetMarketsStatus200",
        &full["GetMarketsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketParameters>(
        "GetMarketParameters",
        &full["GetMarketParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketStatus200>(
        "GetMarketStatus200",
        &full["GetMarketStatus200"],
    );
    mutations += model_support::assert_nulls::<BatchGetMarketCandlesticksParameters>(
        "BatchGetMarketCandlesticksParameters",
        &full["BatchGetMarketCandlesticksParameters"],
    );
    mutations += model_support::assert_nulls::<BatchGetMarketCandlesticksStatus200>(
        "BatchGetMarketCandlesticksStatus200",
        &full["BatchGetMarketCandlesticksStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksByEventParameters>(
        "GetMarketCandlesticksByEventParameters",
        &full["GetMarketCandlesticksByEventParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksByEventStatus200>(
        "GetMarketCandlesticksByEventStatus200",
        &full["GetMarketCandlesticksByEventStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventsParameters>(
        "GetEventsParameters",
        &full["GetEventsParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventsStatus200>(
        "GetEventsStatus200",
        &full["GetEventsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventsParameters>(
        "GetMultivariateEventsParameters",
        &full["GetMultivariateEventsParameters"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventsStatus200>(
        "GetMultivariateEventsStatus200",
        &full["GetMultivariateEventsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventFeeChangesParameters>(
        "GetEventFeeChangesParameters",
        &full["GetEventFeeChangesParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventFeeChangesStatus200>(
        "GetEventFeeChangesStatus200",
        &full["GetEventFeeChangesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventFeeChangesStatus400>(
        "GetEventFeeChangesStatus400",
        &full["GetEventFeeChangesStatus400"],
    );
    mutations += model_support::assert_nulls::<GetEventFeeChangesStatus500>(
        "GetEventFeeChangesStatus500",
        &full["GetEventFeeChangesStatus500"],
    );
    mutations += model_support::assert_nulls::<GetEventParameters>(
        "GetEventParameters",
        &full["GetEventParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventStatus200>(
        "GetEventStatus200",
        &full["GetEventStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventMetadataParameters>(
        "GetEventMetadataParameters",
        &full["GetEventMetadataParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventMetadataStatus200>(
        "GetEventMetadataStatus200",
        &full["GetEventMetadataStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventForecastPercentilesHistoryParameters>(
        "GetEventForecastPercentilesHistoryParameters",
        &full["GetEventForecastPercentilesHistoryParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventForecastPercentilesHistoryStatus200>(
        "GetEventForecastPercentilesHistoryStatus200",
        &full["GetEventForecastPercentilesHistoryStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrdersParameters>(
        "GetOrdersParameters",
        &full["GetOrdersParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrdersStatus200>(
        "GetOrdersStatus200",
        &full["GetOrdersStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrdersStatus400>(
        "GetOrdersStatus400",
        &full["GetOrdersStatus400"],
    );
    mutations += model_support::assert_nulls::<GetOrdersStatus401>(
        "GetOrdersStatus401",
        &full["GetOrdersStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrdersStatus500>(
        "GetOrdersStatus500",
        &full["GetOrdersStatus500"],
    );
    mutations += model_support::assert_nulls::<GetOrderParameters>(
        "GetOrderParameters",
        &full["GetOrderParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrderStatus200>(
        "GetOrderStatus200",
        &full["GetOrderStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrderStatus401>(
        "GetOrderStatus401",
        &full["GetOrderStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrderStatus404>(
        "GetOrderStatus404",
        &full["GetOrderStatus404"],
    );
    mutations += model_support::assert_nulls::<GetOrderStatus500>(
        "GetOrderStatus500",
        &full["GetOrderStatus500"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsParameters>(
        "GetOrderQueuePositionsParameters",
        &full["GetOrderQueuePositionsParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsStatus200>(
        "GetOrderQueuePositionsStatus200",
        &full["GetOrderQueuePositionsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsStatus400>(
        "GetOrderQueuePositionsStatus400",
        &full["GetOrderQueuePositionsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsStatus401>(
        "GetOrderQueuePositionsStatus401",
        &full["GetOrderQueuePositionsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionsStatus500>(
        "GetOrderQueuePositionsStatus500",
        &full["GetOrderQueuePositionsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionParameters>(
        "GetOrderQueuePositionParameters",
        &full["GetOrderQueuePositionParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionStatus200>(
        "GetOrderQueuePositionStatus200",
        &full["GetOrderQueuePositionStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionStatus401>(
        "GetOrderQueuePositionStatus401",
        &full["GetOrderQueuePositionStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionStatus404>(
        "GetOrderQueuePositionStatus404",
        &full["GetOrderQueuePositionStatus404"],
    );
    mutations += model_support::assert_nulls::<GetOrderQueuePositionStatus500>(
        "GetOrderQueuePositionStatus500",
        &full["GetOrderQueuePositionStatus500"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Body>(
        "CreateOrderV2Body",
        &full["CreateOrderV2Body"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status201>(
        "CreateOrderV2Status201",
        &full["CreateOrderV2Status201"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status400>(
        "CreateOrderV2Status400",
        &full["CreateOrderV2Status400"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status401>(
        "CreateOrderV2Status401",
        &full["CreateOrderV2Status401"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status403>(
        "CreateOrderV2Status403",
        &full["CreateOrderV2Status403"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status409>(
        "CreateOrderV2Status409",
        &full["CreateOrderV2Status409"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status429>(
        "CreateOrderV2Status429",
        &full["CreateOrderV2Status429"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status500>(
        "CreateOrderV2Status500",
        &full["CreateOrderV2Status500"],
    );
    mutations += model_support::assert_nulls::<CreateOrderV2Status503>(
        "CreateOrderV2Status503",
        &full["CreateOrderV2Status503"],
    );
    mutations += model_support::assert_nulls::<CancelAllOrdersParameters>(
        "CancelAllOrdersParameters",
        &full["CancelAllOrdersParameters"],
    );
    mutations += model_support::assert_nulls::<CancelAllOrdersStatus401>(
        "CancelAllOrdersStatus401",
        &full["CancelAllOrdersStatus401"],
    );
    mutations += model_support::assert_nulls::<CancelAllOrdersStatus429>(
        "CancelAllOrdersStatus429",
        &full["CancelAllOrdersStatus429"],
    );
    mutations += model_support::assert_nulls::<CancelAllOrdersStatus500>(
        "CancelAllOrdersStatus500",
        &full["CancelAllOrdersStatus500"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Body>(
        "BatchCreateOrdersV2Body",
        &full["BatchCreateOrdersV2Body"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Status201>(
        "BatchCreateOrdersV2Status201",
        &full["BatchCreateOrdersV2Status201"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Status400>(
        "BatchCreateOrdersV2Status400",
        &full["BatchCreateOrdersV2Status400"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Status401>(
        "BatchCreateOrdersV2Status401",
        &full["BatchCreateOrdersV2Status401"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Status403>(
        "BatchCreateOrdersV2Status403",
        &full["BatchCreateOrdersV2Status403"],
    );
    mutations += model_support::assert_nulls::<BatchCreateOrdersV2Status500>(
        "BatchCreateOrdersV2Status500",
        &full["BatchCreateOrdersV2Status500"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Body>(
        "BatchCancelOrdersV2Body",
        &full["BatchCancelOrdersV2Body"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Status200>(
        "BatchCancelOrdersV2Status200",
        &full["BatchCancelOrdersV2Status200"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Status400>(
        "BatchCancelOrdersV2Status400",
        &full["BatchCancelOrdersV2Status400"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Status401>(
        "BatchCancelOrdersV2Status401",
        &full["BatchCancelOrdersV2Status401"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Status403>(
        "BatchCancelOrdersV2Status403",
        &full["BatchCancelOrdersV2Status403"],
    );
    mutations += model_support::assert_nulls::<BatchCancelOrdersV2Status500>(
        "BatchCancelOrdersV2Status500",
        &full["BatchCancelOrdersV2Status500"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Parameters>(
        "CancelOrderV2Parameters",
        &full["CancelOrderV2Parameters"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Status200>(
        "CancelOrderV2Status200",
        &full["CancelOrderV2Status200"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Status401>(
        "CancelOrderV2Status401",
        &full["CancelOrderV2Status401"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Status404>(
        "CancelOrderV2Status404",
        &full["CancelOrderV2Status404"],
    );
    mutations += model_support::assert_nulls::<CancelOrderV2Status500>(
        "CancelOrderV2Status500",
        &full["CancelOrderV2Status500"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Parameters>(
        "AmendOrderV2Parameters",
        &full["AmendOrderV2Parameters"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Body>(
        "AmendOrderV2Body",
        &full["AmendOrderV2Body"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status200>(
        "AmendOrderV2Status200",
        &full["AmendOrderV2Status200"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status400>(
        "AmendOrderV2Status400",
        &full["AmendOrderV2Status400"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status401>(
        "AmendOrderV2Status401",
        &full["AmendOrderV2Status401"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status403>(
        "AmendOrderV2Status403",
        &full["AmendOrderV2Status403"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status404>(
        "AmendOrderV2Status404",
        &full["AmendOrderV2Status404"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status500>(
        "AmendOrderV2Status500",
        &full["AmendOrderV2Status500"],
    );
    mutations += model_support::assert_nulls::<AmendOrderV2Status503>(
        "AmendOrderV2Status503",
        &full["AmendOrderV2Status503"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Parameters>(
        "DecreaseOrderV2Parameters",
        &full["DecreaseOrderV2Parameters"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Body>(
        "DecreaseOrderV2Body",
        &full["DecreaseOrderV2Body"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Status200>(
        "DecreaseOrderV2Status200",
        &full["DecreaseOrderV2Status200"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Status400>(
        "DecreaseOrderV2Status400",
        &full["DecreaseOrderV2Status400"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Status401>(
        "DecreaseOrderV2Status401",
        &full["DecreaseOrderV2Status401"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Status404>(
        "DecreaseOrderV2Status404",
        &full["DecreaseOrderV2Status404"],
    );
    mutations += model_support::assert_nulls::<DecreaseOrderV2Status500>(
        "DecreaseOrderV2Status500",
        &full["DecreaseOrderV2Status500"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupsParameters>(
        "GetOrderGroupsParameters",
        &full["GetOrderGroupsParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupsStatus200>(
        "GetOrderGroupsStatus200",
        &full["GetOrderGroupsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupsStatus400>(
        "GetOrderGroupsStatus400",
        &full["GetOrderGroupsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupsStatus401>(
        "GetOrderGroupsStatus401",
        &full["GetOrderGroupsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupsStatus500>(
        "GetOrderGroupsStatus500",
        &full["GetOrderGroupsStatus500"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupBody>(
        "CreateOrderGroupBody",
        &full["CreateOrderGroupBody"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupStatus201>(
        "CreateOrderGroupStatus201",
        &full["CreateOrderGroupStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupStatus400>(
        "CreateOrderGroupStatus400",
        &full["CreateOrderGroupStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupStatus401>(
        "CreateOrderGroupStatus401",
        &full["CreateOrderGroupStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateOrderGroupStatus500>(
        "CreateOrderGroupStatus500",
        &full["CreateOrderGroupStatus500"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupParameters>(
        "GetOrderGroupParameters",
        &full["GetOrderGroupParameters"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupStatus200>(
        "GetOrderGroupStatus200",
        &full["GetOrderGroupStatus200"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupStatus401>(
        "GetOrderGroupStatus401",
        &full["GetOrderGroupStatus401"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupStatus404>(
        "GetOrderGroupStatus404",
        &full["GetOrderGroupStatus404"],
    );
    mutations += model_support::assert_nulls::<GetOrderGroupStatus500>(
        "GetOrderGroupStatus500",
        &full["GetOrderGroupStatus500"],
    );
    mutations += model_support::assert_nulls::<DeleteOrderGroupParameters>(
        "DeleteOrderGroupParameters",
        &full["DeleteOrderGroupParameters"],
    );
    mutations += model_support::assert_nulls::<DeleteOrderGroupStatus200>(
        "DeleteOrderGroupStatus200",
        &full["DeleteOrderGroupStatus200"],
    );
    mutations += model_support::assert_nulls::<DeleteOrderGroupStatus401>(
        "DeleteOrderGroupStatus401",
        &full["DeleteOrderGroupStatus401"],
    );
    mutations += model_support::assert_nulls::<DeleteOrderGroupStatus404>(
        "DeleteOrderGroupStatus404",
        &full["DeleteOrderGroupStatus404"],
    );
    mutations += model_support::assert_nulls::<DeleteOrderGroupStatus500>(
        "DeleteOrderGroupStatus500",
        &full["DeleteOrderGroupStatus500"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupParameters>(
        "ResetOrderGroupParameters",
        &full["ResetOrderGroupParameters"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupBody>(
        "ResetOrderGroupBody",
        &full["ResetOrderGroupBody"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupStatus200>(
        "ResetOrderGroupStatus200",
        &full["ResetOrderGroupStatus200"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupStatus401>(
        "ResetOrderGroupStatus401",
        &full["ResetOrderGroupStatus401"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupStatus404>(
        "ResetOrderGroupStatus404",
        &full["ResetOrderGroupStatus404"],
    );
    mutations += model_support::assert_nulls::<ResetOrderGroupStatus500>(
        "ResetOrderGroupStatus500",
        &full["ResetOrderGroupStatus500"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupParameters>(
        "TriggerOrderGroupParameters",
        &full["TriggerOrderGroupParameters"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupBody>(
        "TriggerOrderGroupBody",
        &full["TriggerOrderGroupBody"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupStatus200>(
        "TriggerOrderGroupStatus200",
        &full["TriggerOrderGroupStatus200"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupStatus401>(
        "TriggerOrderGroupStatus401",
        &full["TriggerOrderGroupStatus401"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupStatus404>(
        "TriggerOrderGroupStatus404",
        &full["TriggerOrderGroupStatus404"],
    );
    mutations += model_support::assert_nulls::<TriggerOrderGroupStatus500>(
        "TriggerOrderGroupStatus500",
        &full["TriggerOrderGroupStatus500"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitParameters>(
        "UpdateOrderGroupLimitParameters",
        &full["UpdateOrderGroupLimitParameters"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitBody>(
        "UpdateOrderGroupLimitBody",
        &full["UpdateOrderGroupLimitBody"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitStatus200>(
        "UpdateOrderGroupLimitStatus200",
        &full["UpdateOrderGroupLimitStatus200"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitStatus400>(
        "UpdateOrderGroupLimitStatus400",
        &full["UpdateOrderGroupLimitStatus400"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitStatus401>(
        "UpdateOrderGroupLimitStatus401",
        &full["UpdateOrderGroupLimitStatus401"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitStatus404>(
        "UpdateOrderGroupLimitStatus404",
        &full["UpdateOrderGroupLimitStatus404"],
    );
    mutations += model_support::assert_nulls::<UpdateOrderGroupLimitStatus500>(
        "UpdateOrderGroupLimitStatus500",
        &full["UpdateOrderGroupLimitStatus500"],
    );
    mutations += model_support::assert_nulls::<GetBalanceParameters>(
        "GetBalanceParameters",
        &full["GetBalanceParameters"],
    );
    mutations += model_support::assert_nulls::<GetBalanceStatus200>(
        "GetBalanceStatus200",
        &full["GetBalanceStatus200"],
    );
    mutations += model_support::assert_nulls::<GetBalanceStatus401>(
        "GetBalanceStatus401",
        &full["GetBalanceStatus401"],
    );
    mutations += model_support::assert_nulls::<GetBalanceStatus500>(
        "GetBalanceStatus500",
        &full["GetBalanceStatus500"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferBody>(
        "IntraExchangeInstanceTransferBody",
        &full["IntraExchangeInstanceTransferBody"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus200>(
        "IntraExchangeInstanceTransferStatus200",
        &full["IntraExchangeInstanceTransferStatus200"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus400>(
        "IntraExchangeInstanceTransferStatus400",
        &full["IntraExchangeInstanceTransferStatus400"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus401>(
        "IntraExchangeInstanceTransferStatus401",
        &full["IntraExchangeInstanceTransferStatus401"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus403>(
        "IntraExchangeInstanceTransferStatus403",
        &full["IntraExchangeInstanceTransferStatus403"],
    );
    mutations += model_support::assert_nulls::<IntraExchangeInstanceTransferStatus500>(
        "IntraExchangeInstanceTransferStatus500",
        &full["IntraExchangeInstanceTransferStatus500"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersParameters>(
        "GetIntraExchangeInstanceTransfersParameters",
        &full["GetIntraExchangeInstanceTransfersParameters"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersStatus200>(
        "GetIntraExchangeInstanceTransfersStatus200",
        &full["GetIntraExchangeInstanceTransfersStatus200"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersStatus400>(
        "GetIntraExchangeInstanceTransfersStatus400",
        &full["GetIntraExchangeInstanceTransfersStatus400"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersStatus401>(
        "GetIntraExchangeInstanceTransfersStatus401",
        &full["GetIntraExchangeInstanceTransfersStatus401"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransfersStatus500>(
        "GetIntraExchangeInstanceTransfersStatus500",
        &full["GetIntraExchangeInstanceTransfersStatus500"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferParameters>(
        "GetIntraExchangeInstanceTransferParameters",
        &full["GetIntraExchangeInstanceTransferParameters"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferStatus200>(
        "GetIntraExchangeInstanceTransferStatus200",
        &full["GetIntraExchangeInstanceTransferStatus200"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferStatus401>(
        "GetIntraExchangeInstanceTransferStatus401",
        &full["GetIntraExchangeInstanceTransferStatus401"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferStatus404>(
        "GetIntraExchangeInstanceTransferStatus404",
        &full["GetIntraExchangeInstanceTransferStatus404"],
    );
    mutations += model_support::assert_nulls::<GetIntraExchangeInstanceTransferStatus500>(
        "GetIntraExchangeInstanceTransferStatus500",
        &full["GetIntraExchangeInstanceTransferStatus500"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountBody>(
        "CreateSubaccountBody",
        &full["CreateSubaccountBody"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountStatus201>(
        "CreateSubaccountStatus201",
        &full["CreateSubaccountStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountStatus400>(
        "CreateSubaccountStatus400",
        &full["CreateSubaccountStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountStatus401>(
        "CreateSubaccountStatus401",
        &full["CreateSubaccountStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateSubaccountStatus500>(
        "CreateSubaccountStatus500",
        &full["CreateSubaccountStatus500"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferBody>(
        "ApplySubaccountTransferBody",
        &full["ApplySubaccountTransferBody"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferStatus200>(
        "ApplySubaccountTransferStatus200",
        &full["ApplySubaccountTransferStatus200"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferStatus400>(
        "ApplySubaccountTransferStatus400",
        &full["ApplySubaccountTransferStatus400"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferStatus401>(
        "ApplySubaccountTransferStatus401",
        &full["ApplySubaccountTransferStatus401"],
    );
    mutations += model_support::assert_nulls::<ApplySubaccountTransferStatus500>(
        "ApplySubaccountTransferStatus500",
        &full["ApplySubaccountTransferStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountBalancesStatus200>(
        "GetSubaccountBalancesStatus200",
        &full["GetSubaccountBalancesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountBalancesStatus401>(
        "GetSubaccountBalancesStatus401",
        &full["GetSubaccountBalancesStatus401"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountBalancesStatus500>(
        "GetSubaccountBalancesStatus500",
        &full["GetSubaccountBalancesStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountTransfersParameters>(
        "GetSubaccountTransfersParameters",
        &full["GetSubaccountTransfersParameters"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountTransfersStatus200>(
        "GetSubaccountTransfersStatus200",
        &full["GetSubaccountTransfersStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountTransfersStatus401>(
        "GetSubaccountTransfersStatus401",
        &full["GetSubaccountTransfersStatus401"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountTransfersStatus500>(
        "GetSubaccountTransfersStatus500",
        &full["GetSubaccountTransfersStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountNettingStatus200>(
        "GetSubaccountNettingStatus200",
        &full["GetSubaccountNettingStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountNettingStatus401>(
        "GetSubaccountNettingStatus401",
        &full["GetSubaccountNettingStatus401"],
    );
    mutations += model_support::assert_nulls::<GetSubaccountNettingStatus500>(
        "GetSubaccountNettingStatus500",
        &full["GetSubaccountNettingStatus500"],
    );
    mutations += model_support::assert_nulls::<UpdateSubaccountNettingBody>(
        "UpdateSubaccountNettingBody",
        &full["UpdateSubaccountNettingBody"],
    );
    mutations += model_support::assert_nulls::<UpdateSubaccountNettingStatus400>(
        "UpdateSubaccountNettingStatus400",
        &full["UpdateSubaccountNettingStatus400"],
    );
    mutations += model_support::assert_nulls::<UpdateSubaccountNettingStatus401>(
        "UpdateSubaccountNettingStatus401",
        &full["UpdateSubaccountNettingStatus401"],
    );
    mutations += model_support::assert_nulls::<UpdateSubaccountNettingStatus500>(
        "UpdateSubaccountNettingStatus500",
        &full["UpdateSubaccountNettingStatus500"],
    );
    mutations += model_support::assert_nulls::<GetPositionsParameters>(
        "GetPositionsParameters",
        &full["GetPositionsParameters"],
    );
    mutations += model_support::assert_nulls::<GetPositionsStatus200>(
        "GetPositionsStatus200",
        &full["GetPositionsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetPositionsStatus400>(
        "GetPositionsStatus400",
        &full["GetPositionsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetPositionsStatus401>(
        "GetPositionsStatus401",
        &full["GetPositionsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetPositionsStatus500>(
        "GetPositionsStatus500",
        &full["GetPositionsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsParameters>(
        "GetSettlementsParameters",
        &full["GetSettlementsParameters"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsStatus200>(
        "GetSettlementsStatus200",
        &full["GetSettlementsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsStatus400>(
        "GetSettlementsStatus400",
        &full["GetSettlementsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsStatus401>(
        "GetSettlementsStatus401",
        &full["GetSettlementsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetSettlementsStatus500>(
        "GetSettlementsStatus500",
        &full["GetSettlementsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetDepositsParameters>(
        "GetDepositsParameters",
        &full["GetDepositsParameters"],
    );
    mutations += model_support::assert_nulls::<GetDepositsStatus200>(
        "GetDepositsStatus200",
        &full["GetDepositsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetDepositsStatus400>(
        "GetDepositsStatus400",
        &full["GetDepositsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetDepositsStatus401>(
        "GetDepositsStatus401",
        &full["GetDepositsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetDepositsStatus500>(
        "GetDepositsStatus500",
        &full["GetDepositsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetWithdrawalsParameters>(
        "GetWithdrawalsParameters",
        &full["GetWithdrawalsParameters"],
    );
    mutations += model_support::assert_nulls::<GetWithdrawalsStatus200>(
        "GetWithdrawalsStatus200",
        &full["GetWithdrawalsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetWithdrawalsStatus400>(
        "GetWithdrawalsStatus400",
        &full["GetWithdrawalsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetWithdrawalsStatus401>(
        "GetWithdrawalsStatus401",
        &full["GetWithdrawalsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetWithdrawalsStatus500>(
        "GetWithdrawalsStatus500",
        &full["GetWithdrawalsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetPortfolioRestingOrderTotalValueStatus200>(
        "GetPortfolioRestingOrderTotalValueStatus200",
        &full["GetPortfolioRestingOrderTotalValueStatus200"],
    );
    mutations += model_support::assert_nulls::<GetPortfolioRestingOrderTotalValueStatus401>(
        "GetPortfolioRestingOrderTotalValueStatus401",
        &full["GetPortfolioRestingOrderTotalValueStatus401"],
    );
    mutations += model_support::assert_nulls::<GetPortfolioRestingOrderTotalValueStatus500>(
        "GetPortfolioRestingOrderTotalValueStatus500",
        &full["GetPortfolioRestingOrderTotalValueStatus500"],
    );
    mutations += model_support::assert_nulls::<GetFillsParameters>(
        "GetFillsParameters",
        &full["GetFillsParameters"],
    );
    mutations += model_support::assert_nulls::<GetFillsStatus200>(
        "GetFillsStatus200",
        &full["GetFillsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetCommunicationsIdStatus200>(
        "GetCommunicationsIdStatus200",
        &full["GetCommunicationsIdStatus200"],
    );
    mutations += model_support::assert_nulls::<GetCommunicationsIdStatus401>(
        "GetCommunicationsIdStatus401",
        &full["GetCommunicationsIdStatus401"],
    );
    mutations += model_support::assert_nulls::<GetCommunicationsIdStatus500>(
        "GetCommunicationsIdStatus500",
        &full["GetCommunicationsIdStatus500"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsParameters>(
        "GetBlockTradeProposalsParameters",
        &full["GetBlockTradeProposalsParameters"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsStatus200>(
        "GetBlockTradeProposalsStatus200",
        &full["GetBlockTradeProposalsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsStatus400>(
        "GetBlockTradeProposalsStatus400",
        &full["GetBlockTradeProposalsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsStatus401>(
        "GetBlockTradeProposalsStatus401",
        &full["GetBlockTradeProposalsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetBlockTradeProposalsStatus500>(
        "GetBlockTradeProposalsStatus500",
        &full["GetBlockTradeProposalsStatus500"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeBody>(
        "ProposeBlockTradeBody",
        &full["ProposeBlockTradeBody"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeStatus201>(
        "ProposeBlockTradeStatus201",
        &full["ProposeBlockTradeStatus201"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeStatus400>(
        "ProposeBlockTradeStatus400",
        &full["ProposeBlockTradeStatus400"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeStatus401>(
        "ProposeBlockTradeStatus401",
        &full["ProposeBlockTradeStatus401"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeStatus403>(
        "ProposeBlockTradeStatus403",
        &full["ProposeBlockTradeStatus403"],
    );
    mutations += model_support::assert_nulls::<ProposeBlockTradeStatus500>(
        "ProposeBlockTradeStatus500",
        &full["ProposeBlockTradeStatus500"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalParameters>(
        "AcceptBlockTradeProposalParameters",
        &full["AcceptBlockTradeProposalParameters"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalBody>(
        "AcceptBlockTradeProposalBody",
        &full["AcceptBlockTradeProposalBody"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalStatus400>(
        "AcceptBlockTradeProposalStatus400",
        &full["AcceptBlockTradeProposalStatus400"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalStatus401>(
        "AcceptBlockTradeProposalStatus401",
        &full["AcceptBlockTradeProposalStatus401"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalStatus404>(
        "AcceptBlockTradeProposalStatus404",
        &full["AcceptBlockTradeProposalStatus404"],
    );
    mutations += model_support::assert_nulls::<AcceptBlockTradeProposalStatus500>(
        "AcceptBlockTradeProposalStatus500",
        &full["AcceptBlockTradeProposalStatus500"],
    );
    mutations += model_support::assert_nulls::<GetRfQsParameters>(
        "GetRfQsParameters",
        &full["GetRfQsParameters"],
    );
    mutations += model_support::assert_nulls::<GetRfQsStatus200>(
        "GetRfQsStatus200",
        &full["GetRfQsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetRfQsStatus400>(
        "GetRfQsStatus400",
        &full["GetRfQsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetRfQsStatus401>(
        "GetRfQsStatus401",
        &full["GetRfQsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetRfQsStatus500>(
        "GetRfQsStatus500",
        &full["GetRfQsStatus500"],
    );
    mutations +=
        model_support::assert_nulls::<CreateRfqBody>("CreateRfqBody", &full["CreateRfqBody"]);
    mutations += model_support::assert_nulls::<CreateRfqStatus201>(
        "CreateRfqStatus201",
        &full["CreateRfqStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateRfqStatus400>(
        "CreateRfqStatus400",
        &full["CreateRfqStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateRfqStatus401>(
        "CreateRfqStatus401",
        &full["CreateRfqStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateRfqStatus409>(
        "CreateRfqStatus409",
        &full["CreateRfqStatus409"],
    );
    mutations += model_support::assert_nulls::<CreateRfqStatus500>(
        "CreateRfqStatus500",
        &full["CreateRfqStatus500"],
    );
    mutations += model_support::assert_nulls::<GetRfqParameters>(
        "GetRfqParameters",
        &full["GetRfqParameters"],
    );
    mutations +=
        model_support::assert_nulls::<GetRfqStatus200>("GetRfqStatus200", &full["GetRfqStatus200"]);
    mutations +=
        model_support::assert_nulls::<GetRfqStatus400>("GetRfqStatus400", &full["GetRfqStatus400"]);
    mutations +=
        model_support::assert_nulls::<GetRfqStatus401>("GetRfqStatus401", &full["GetRfqStatus401"]);
    mutations +=
        model_support::assert_nulls::<GetRfqStatus404>("GetRfqStatus404", &full["GetRfqStatus404"]);
    mutations +=
        model_support::assert_nulls::<GetRfqStatus500>("GetRfqStatus500", &full["GetRfqStatus500"]);
    mutations += model_support::assert_nulls::<DeleteRfqParameters>(
        "DeleteRfqParameters",
        &full["DeleteRfqParameters"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqStatus400>(
        "DeleteRfqStatus400",
        &full["DeleteRfqStatus400"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqStatus401>(
        "DeleteRfqStatus401",
        &full["DeleteRfqStatus401"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqStatus404>(
        "DeleteRfqStatus404",
        &full["DeleteRfqStatus404"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqStatus500>(
        "DeleteRfqStatus500",
        &full["DeleteRfqStatus500"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteParameters>(
        "GetRfqQuoteParameters",
        &full["GetRfqQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteStatus200>(
        "GetRfqQuoteStatus200",
        &full["GetRfqQuoteStatus200"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteStatus400>(
        "GetRfqQuoteStatus400",
        &full["GetRfqQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteStatus401>(
        "GetRfqQuoteStatus401",
        &full["GetRfqQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteStatus404>(
        "GetRfqQuoteStatus404",
        &full["GetRfqQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<GetRfqQuoteStatus500>(
        "GetRfqQuoteStatus500",
        &full["GetRfqQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqQuoteParameters>(
        "DeleteRfqQuoteParameters",
        &full["DeleteRfqQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqQuoteStatus400>(
        "DeleteRfqQuoteStatus400",
        &full["DeleteRfqQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqQuoteStatus401>(
        "DeleteRfqQuoteStatus401",
        &full["DeleteRfqQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqQuoteStatus404>(
        "DeleteRfqQuoteStatus404",
        &full["DeleteRfqQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<DeleteRfqQuoteStatus500>(
        "DeleteRfqQuoteStatus500",
        &full["DeleteRfqQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteParameters>(
        "AcceptRfqQuoteParameters",
        &full["AcceptRfqQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteBody>(
        "AcceptRfqQuoteBody",
        &full["AcceptRfqQuoteBody"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteStatus400>(
        "AcceptRfqQuoteStatus400",
        &full["AcceptRfqQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteStatus401>(
        "AcceptRfqQuoteStatus401",
        &full["AcceptRfqQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteStatus404>(
        "AcceptRfqQuoteStatus404",
        &full["AcceptRfqQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<AcceptRfqQuoteStatus500>(
        "AcceptRfqQuoteStatus500",
        &full["AcceptRfqQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteParameters>(
        "ConfirmRfqQuoteParameters",
        &full["ConfirmRfqQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteBody>(
        "ConfirmRfqQuoteBody",
        &full["ConfirmRfqQuoteBody"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteStatus400>(
        "ConfirmRfqQuoteStatus400",
        &full["ConfirmRfqQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteStatus401>(
        "ConfirmRfqQuoteStatus401",
        &full["ConfirmRfqQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteStatus404>(
        "ConfirmRfqQuoteStatus404",
        &full["ConfirmRfqQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<ConfirmRfqQuoteStatus500>(
        "ConfirmRfqQuoteStatus500",
        &full["ConfirmRfqQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<GetQuotesParameters>(
        "GetQuotesParameters",
        &full["GetQuotesParameters"],
    );
    mutations += model_support::assert_nulls::<GetQuotesStatus200>(
        "GetQuotesStatus200",
        &full["GetQuotesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetQuotesStatus400>(
        "GetQuotesStatus400",
        &full["GetQuotesStatus400"],
    );
    mutations += model_support::assert_nulls::<GetQuotesStatus401>(
        "GetQuotesStatus401",
        &full["GetQuotesStatus401"],
    );
    mutations += model_support::assert_nulls::<GetQuotesStatus500>(
        "GetQuotesStatus500",
        &full["GetQuotesStatus500"],
    );
    mutations +=
        model_support::assert_nulls::<CreateQuoteBody>("CreateQuoteBody", &full["CreateQuoteBody"]);
    mutations += model_support::assert_nulls::<CreateQuoteStatus201>(
        "CreateQuoteStatus201",
        &full["CreateQuoteStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateQuoteStatus400>(
        "CreateQuoteStatus400",
        &full["CreateQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateQuoteStatus401>(
        "CreateQuoteStatus401",
        &full["CreateQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateQuoteStatus500>(
        "CreateQuoteStatus500",
        &full["CreateQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<GetQuoteParameters>(
        "GetQuoteParameters",
        &full["GetQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<GetQuoteStatus200>(
        "GetQuoteStatus200",
        &full["GetQuoteStatus200"],
    );
    mutations += model_support::assert_nulls::<GetQuoteStatus400>(
        "GetQuoteStatus400",
        &full["GetQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<GetQuoteStatus401>(
        "GetQuoteStatus401",
        &full["GetQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<GetQuoteStatus404>(
        "GetQuoteStatus404",
        &full["GetQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<GetQuoteStatus500>(
        "GetQuoteStatus500",
        &full["GetQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<DeleteQuoteParameters>(
        "DeleteQuoteParameters",
        &full["DeleteQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<DeleteQuoteStatus400>(
        "DeleteQuoteStatus400",
        &full["DeleteQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<DeleteQuoteStatus401>(
        "DeleteQuoteStatus401",
        &full["DeleteQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<DeleteQuoteStatus404>(
        "DeleteQuoteStatus404",
        &full["DeleteQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<DeleteQuoteStatus500>(
        "DeleteQuoteStatus500",
        &full["DeleteQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<AcceptQuoteParameters>(
        "AcceptQuoteParameters",
        &full["AcceptQuoteParameters"],
    );
    mutations +=
        model_support::assert_nulls::<AcceptQuoteBody>("AcceptQuoteBody", &full["AcceptQuoteBody"]);
    mutations += model_support::assert_nulls::<AcceptQuoteStatus400>(
        "AcceptQuoteStatus400",
        &full["AcceptQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<AcceptQuoteStatus401>(
        "AcceptQuoteStatus401",
        &full["AcceptQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<AcceptQuoteStatus404>(
        "AcceptQuoteStatus404",
        &full["AcceptQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<AcceptQuoteStatus500>(
        "AcceptQuoteStatus500",
        &full["AcceptQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteParameters>(
        "ConfirmQuoteParameters",
        &full["ConfirmQuoteParameters"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteBody>(
        "ConfirmQuoteBody",
        &full["ConfirmQuoteBody"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteStatus400>(
        "ConfirmQuoteStatus400",
        &full["ConfirmQuoteStatus400"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteStatus401>(
        "ConfirmQuoteStatus401",
        &full["ConfirmQuoteStatus401"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteStatus404>(
        "ConfirmQuoteStatus404",
        &full["ConfirmQuoteStatus404"],
    );
    mutations += model_support::assert_nulls::<ConfirmQuoteStatus500>(
        "ConfirmQuoteStatus500",
        &full["ConfirmQuoteStatus500"],
    );
    mutations += model_support::assert_nulls::<GetApiKeysParameters>(
        "GetApiKeysParameters",
        &full["GetApiKeysParameters"],
    );
    mutations += model_support::assert_nulls::<GetApiKeysStatus200>(
        "GetApiKeysStatus200",
        &full["GetApiKeysStatus200"],
    );
    mutations += model_support::assert_nulls::<CreateApiKeyBody>(
        "CreateApiKeyBody",
        &full["CreateApiKeyBody"],
    );
    mutations += model_support::assert_nulls::<CreateApiKeyStatus201>(
        "CreateApiKeyStatus201",
        &full["CreateApiKeyStatus201"],
    );
    mutations += model_support::assert_nulls::<GenerateApiKeyBody>(
        "GenerateApiKeyBody",
        &full["GenerateApiKeyBody"],
    );
    mutations += model_support::assert_nulls::<GenerateApiKeyStatus201>(
        "GenerateApiKeyStatus201",
        &full["GenerateApiKeyStatus201"],
    );
    mutations += model_support::assert_nulls::<DeleteApiKeyParameters>(
        "DeleteApiKeyParameters",
        &full["DeleteApiKeyParameters"],
    );
    mutations += model_support::assert_nulls::<GetAccountApiLimitsStatus200>(
        "GetAccountApiLimitsStatus200",
        &full["GetAccountApiLimitsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetAccountApiUsageLevelVolumeProgressStatus200>(
        "GetAccountApiUsageLevelVolumeProgressStatus200",
        &full["GetAccountApiUsageLevelVolumeProgressStatus200"],
    );
    mutations += model_support::assert_nulls::<GetAccountEndpointCostsStatus200>(
        "GetAccountEndpointCostsStatus200",
        &full["GetAccountEndpointCostsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetTagsForSeriesCategoriesStatus200>(
        "GetTagsForSeriesCategoriesStatus200",
        &full["GetTagsForSeriesCategoriesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFiltersForSportsStatus200>(
        "GetFiltersForSportsStatus200",
        &full["GetFiltersForSportsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetLiveDataByMilestoneParameters>(
        "GetLiveDataByMilestoneParameters",
        &full["GetLiveDataByMilestoneParameters"],
    );
    mutations += model_support::assert_nulls::<GetLiveDataByMilestoneStatus200>(
        "GetLiveDataByMilestoneStatus200",
        &full["GetLiveDataByMilestoneStatus200"],
    );
    mutations += model_support::assert_nulls::<GetLiveDataParameters>(
        "GetLiveDataParameters",
        &full["GetLiveDataParameters"],
    );
    mutations += model_support::assert_nulls::<GetLiveDataStatus200>(
        "GetLiveDataStatus200",
        &full["GetLiveDataStatus200"],
    );
    mutations += model_support::assert_nulls::<GetLiveDatasParameters>(
        "GetLiveDatasParameters",
        &full["GetLiveDatasParameters"],
    );
    mutations += model_support::assert_nulls::<GetLiveDatasStatus200>(
        "GetLiveDatasStatus200",
        &full["GetLiveDatasStatus200"],
    );
    mutations += model_support::assert_nulls::<GetGameStatsParameters>(
        "GetGameStatsParameters",
        &full["GetGameStatsParameters"],
    );
    mutations += model_support::assert_nulls::<GetGameStatsStatus200>(
        "GetGameStatsStatus200",
        &full["GetGameStatsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetEventLiveDataParameters>(
        "GetEventLiveDataParameters",
        &full["GetEventLiveDataParameters"],
    );
    mutations += model_support::assert_nulls::<GetEventLiveDataStatus200>(
        "GetEventLiveDataStatus200",
        &full["GetEventLiveDataStatus200"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexParameters>(
        "GetWeatherIndexParameters",
        &full["GetWeatherIndexParameters"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexStatus200>(
        "GetWeatherIndexStatus200",
        &full["GetWeatherIndexStatus200"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexCalibrationsParameters>(
        "GetWeatherIndexCalibrationsParameters",
        &full["GetWeatherIndexCalibrationsParameters"],
    );
    mutations += model_support::assert_nulls::<GetWeatherIndexCalibrationsStatus200>(
        "GetWeatherIndexCalibrationsStatus200",
        &full["GetWeatherIndexCalibrationsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetsParameters>(
        "GetStructuredTargetsParameters",
        &full["GetStructuredTargetsParameters"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetsStatus200>(
        "GetStructuredTargetsStatus200",
        &full["GetStructuredTargetsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetParameters>(
        "GetStructuredTargetParameters",
        &full["GetStructuredTargetParameters"],
    );
    mutations += model_support::assert_nulls::<GetStructuredTargetStatus200>(
        "GetStructuredTargetStatus200",
        &full["GetStructuredTargetStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMilestoneParameters>(
        "GetMilestoneParameters",
        &full["GetMilestoneParameters"],
    );
    mutations += model_support::assert_nulls::<GetMilestoneStatus200>(
        "GetMilestoneStatus200",
        &full["GetMilestoneStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMilestonesParameters>(
        "GetMilestonesParameters",
        &full["GetMilestonesParameters"],
    );
    mutations += model_support::assert_nulls::<GetMilestonesStatus200>(
        "GetMilestonesStatus200",
        &full["GetMilestonesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionParameters>(
        "GetMultivariateEventCollectionParameters",
        &full["GetMultivariateEventCollectionParameters"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionStatus200>(
        "GetMultivariateEventCollectionStatus200",
        &full["GetMultivariateEventCollectionStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionStatus400>(
        "GetMultivariateEventCollectionStatus400",
        &full["GetMultivariateEventCollectionStatus400"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionStatus404>(
        "GetMultivariateEventCollectionStatus404",
        &full["GetMultivariateEventCollectionStatus404"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionStatus500>(
        "GetMultivariateEventCollectionStatus500",
        &full["GetMultivariateEventCollectionStatus500"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionParameters>(
        "CreateMarketInMultivariateEventCollectionParameters",
        &full["CreateMarketInMultivariateEventCollectionParameters"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionBody>(
        "CreateMarketInMultivariateEventCollectionBody",
        &full["CreateMarketInMultivariateEventCollectionBody"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionStatus200>(
        "CreateMarketInMultivariateEventCollectionStatus200",
        &full["CreateMarketInMultivariateEventCollectionStatus200"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionStatus400>(
        "CreateMarketInMultivariateEventCollectionStatus400",
        &full["CreateMarketInMultivariateEventCollectionStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionStatus401>(
        "CreateMarketInMultivariateEventCollectionStatus401",
        &full["CreateMarketInMultivariateEventCollectionStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionStatus429>(
        "CreateMarketInMultivariateEventCollectionStatus429",
        &full["CreateMarketInMultivariateEventCollectionStatus429"],
    );
    mutations += model_support::assert_nulls::<CreateMarketInMultivariateEventCollectionStatus500>(
        "CreateMarketInMultivariateEventCollectionStatus500",
        &full["CreateMarketInMultivariateEventCollectionStatus500"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionsParameters>(
        "GetMultivariateEventCollectionsParameters",
        &full["GetMultivariateEventCollectionsParameters"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionsStatus200>(
        "GetMultivariateEventCollectionsStatus200",
        &full["GetMultivariateEventCollectionsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionsStatus400>(
        "GetMultivariateEventCollectionsStatus400",
        &full["GetMultivariateEventCollectionsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetMultivariateEventCollectionsStatus500>(
        "GetMultivariateEventCollectionsStatus500",
        &full["GetMultivariateEventCollectionsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetIncentiveProgramsParameters>(
        "GetIncentiveProgramsParameters",
        &full["GetIncentiveProgramsParameters"],
    );
    mutations += model_support::assert_nulls::<GetIncentiveProgramsStatus200>(
        "GetIncentiveProgramsStatus200",
        &full["GetIncentiveProgramsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetIncentiveProgramsStatus400>(
        "GetIncentiveProgramsStatus400",
        &full["GetIncentiveProgramsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetIncentiveProgramsStatus500>(
        "GetIncentiveProgramsStatus500",
        &full["GetIncentiveProgramsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetFcmFillsParameters>(
        "GetFcmFillsParameters",
        &full["GetFcmFillsParameters"],
    );
    mutations += model_support::assert_nulls::<GetFcmFillsStatus200>(
        "GetFcmFillsStatus200",
        &full["GetFcmFillsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFcmOrdersParameters>(
        "GetFcmOrdersParameters",
        &full["GetFcmOrdersParameters"],
    );
    mutations += model_support::assert_nulls::<GetFcmOrdersStatus200>(
        "GetFcmOrdersStatus200",
        &full["GetFcmOrdersStatus200"],
    );
    mutations += model_support::assert_nulls::<ListFcmSubtradersStatus200>(
        "ListFcmSubtradersStatus200",
        &full["ListFcmSubtradersStatus200"],
    );
    mutations += model_support::assert_nulls::<ListFcmSubtradersStatus401>(
        "ListFcmSubtradersStatus401",
        &full["ListFcmSubtradersStatus401"],
    );
    mutations += model_support::assert_nulls::<ListFcmSubtradersStatus403>(
        "ListFcmSubtradersStatus403",
        &full["ListFcmSubtradersStatus403"],
    );
    mutations += model_support::assert_nulls::<ListFcmSubtradersStatus500>(
        "ListFcmSubtradersStatus500",
        &full["ListFcmSubtradersStatus500"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderBody>(
        "CreateFcmSubtraderBody",
        &full["CreateFcmSubtraderBody"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus201>(
        "CreateFcmSubtraderStatus201",
        &full["CreateFcmSubtraderStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus400>(
        "CreateFcmSubtraderStatus400",
        &full["CreateFcmSubtraderStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus401>(
        "CreateFcmSubtraderStatus401",
        &full["CreateFcmSubtraderStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus403>(
        "CreateFcmSubtraderStatus403",
        &full["CreateFcmSubtraderStatus403"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus409>(
        "CreateFcmSubtraderStatus409",
        &full["CreateFcmSubtraderStatus409"],
    );
    mutations += model_support::assert_nulls::<CreateFcmSubtraderStatus500>(
        "CreateFcmSubtraderStatus500",
        &full["CreateFcmSubtraderStatus500"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapParameters>(
        "GetFcmEventContractDailyCapParameters",
        &full["GetFcmEventContractDailyCapParameters"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus200>(
        "GetFcmEventContractDailyCapStatus200",
        &full["GetFcmEventContractDailyCapStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus400>(
        "GetFcmEventContractDailyCapStatus400",
        &full["GetFcmEventContractDailyCapStatus400"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus401>(
        "GetFcmEventContractDailyCapStatus401",
        &full["GetFcmEventContractDailyCapStatus401"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus403>(
        "GetFcmEventContractDailyCapStatus403",
        &full["GetFcmEventContractDailyCapStatus403"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus404>(
        "GetFcmEventContractDailyCapStatus404",
        &full["GetFcmEventContractDailyCapStatus404"],
    );
    mutations += model_support::assert_nulls::<GetFcmEventContractDailyCapStatus500>(
        "GetFcmEventContractDailyCapStatus500",
        &full["GetFcmEventContractDailyCapStatus500"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapBody>(
        "UpdateFcmEventContractDailyCapBody",
        &full["UpdateFcmEventContractDailyCapBody"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus200>(
        "UpdateFcmEventContractDailyCapStatus200",
        &full["UpdateFcmEventContractDailyCapStatus200"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus400>(
        "UpdateFcmEventContractDailyCapStatus400",
        &full["UpdateFcmEventContractDailyCapStatus400"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus401>(
        "UpdateFcmEventContractDailyCapStatus401",
        &full["UpdateFcmEventContractDailyCapStatus401"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus403>(
        "UpdateFcmEventContractDailyCapStatus403",
        &full["UpdateFcmEventContractDailyCapStatus403"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus404>(
        "UpdateFcmEventContractDailyCapStatus404",
        &full["UpdateFcmEventContractDailyCapStatus404"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmEventContractDailyCapStatus500>(
        "UpdateFcmEventContractDailyCapStatus500",
        &full["UpdateFcmEventContractDailyCapStatus500"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapParameters>(
        "DeleteFcmEventContractDailyCapParameters",
        &full["DeleteFcmEventContractDailyCapParameters"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus200>(
        "DeleteFcmEventContractDailyCapStatus200",
        &full["DeleteFcmEventContractDailyCapStatus200"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus400>(
        "DeleteFcmEventContractDailyCapStatus400",
        &full["DeleteFcmEventContractDailyCapStatus400"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus401>(
        "DeleteFcmEventContractDailyCapStatus401",
        &full["DeleteFcmEventContractDailyCapStatus401"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus403>(
        "DeleteFcmEventContractDailyCapStatus403",
        &full["DeleteFcmEventContractDailyCapStatus403"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus404>(
        "DeleteFcmEventContractDailyCapStatus404",
        &full["DeleteFcmEventContractDailyCapStatus404"],
    );
    mutations += model_support::assert_nulls::<DeleteFcmEventContractDailyCapStatus500>(
        "DeleteFcmEventContractDailyCapStatus500",
        &full["DeleteFcmEventContractDailyCapStatus500"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesParameters>(
        "GetFcmSubtraderBlockedCategoriesParameters",
        &full["GetFcmSubtraderBlockedCategoriesParameters"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesStatus200>(
        "GetFcmSubtraderBlockedCategoriesStatus200",
        &full["GetFcmSubtraderBlockedCategoriesStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesStatus400>(
        "GetFcmSubtraderBlockedCategoriesStatus400",
        &full["GetFcmSubtraderBlockedCategoriesStatus400"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesStatus401>(
        "GetFcmSubtraderBlockedCategoriesStatus401",
        &full["GetFcmSubtraderBlockedCategoriesStatus401"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesStatus403>(
        "GetFcmSubtraderBlockedCategoriesStatus403",
        &full["GetFcmSubtraderBlockedCategoriesStatus403"],
    );
    mutations += model_support::assert_nulls::<GetFcmSubtraderBlockedCategoriesStatus500>(
        "GetFcmSubtraderBlockedCategoriesStatus500",
        &full["GetFcmSubtraderBlockedCategoriesStatus500"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesBody>(
        "UpdateFcmSubtraderBlockedCategoriesBody",
        &full["UpdateFcmSubtraderBlockedCategoriesBody"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus200>(
        "UpdateFcmSubtraderBlockedCategoriesStatus200",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus200"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus400>(
        "UpdateFcmSubtraderBlockedCategoriesStatus400",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus400"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus401>(
        "UpdateFcmSubtraderBlockedCategoriesStatus401",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus401"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus403>(
        "UpdateFcmSubtraderBlockedCategoriesStatus403",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus403"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus404>(
        "UpdateFcmSubtraderBlockedCategoriesStatus404",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus404"],
    );
    mutations += model_support::assert_nulls::<UpdateFcmSubtraderBlockedCategoriesStatus500>(
        "UpdateFcmSubtraderBlockedCategoriesStatus500",
        &full["UpdateFcmSubtraderBlockedCategoriesStatus500"],
    );
    mutations += model_support::assert_nulls::<GetTargetBalanceAllocationStatus200>(
        "GetTargetBalanceAllocationStatus200",
        &full["GetTargetBalanceAllocationStatus200"],
    );
    mutations += model_support::assert_nulls::<GetTargetBalanceAllocationStatus401>(
        "GetTargetBalanceAllocationStatus401",
        &full["GetTargetBalanceAllocationStatus401"],
    );
    mutations += model_support::assert_nulls::<GetTargetBalanceAllocationStatus403>(
        "GetTargetBalanceAllocationStatus403",
        &full["GetTargetBalanceAllocationStatus403"],
    );
    mutations += model_support::assert_nulls::<GetTargetBalanceAllocationStatus500>(
        "GetTargetBalanceAllocationStatus500",
        &full["GetTargetBalanceAllocationStatus500"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationBody>(
        "SetTargetBalanceAllocationBody",
        &full["SetTargetBalanceAllocationBody"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationStatus200>(
        "SetTargetBalanceAllocationStatus200",
        &full["SetTargetBalanceAllocationStatus200"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationStatus400>(
        "SetTargetBalanceAllocationStatus400",
        &full["SetTargetBalanceAllocationStatus400"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationStatus401>(
        "SetTargetBalanceAllocationStatus401",
        &full["SetTargetBalanceAllocationStatus401"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationStatus403>(
        "SetTargetBalanceAllocationStatus403",
        &full["SetTargetBalanceAllocationStatus403"],
    );
    mutations += model_support::assert_nulls::<SetTargetBalanceAllocationStatus500>(
        "SetTargetBalanceAllocationStatus500",
        &full["SetTargetBalanceAllocationStatus500"],
    );
    mutations += model_support::assert_nulls::<GetFcmPositionsParameters>(
        "GetFcmPositionsParameters",
        &full["GetFcmPositionsParameters"],
    );
    mutations += model_support::assert_nulls::<GetFcmPositionsStatus200>(
        "GetFcmPositionsStatus200",
        &full["GetFcmPositionsStatus200"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferBody>(
        "CreateFcmPositionTransferBody",
        &full["CreateFcmPositionTransferBody"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus201>(
        "CreateFcmPositionTransferStatus201",
        &full["CreateFcmPositionTransferStatus201"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus400>(
        "CreateFcmPositionTransferStatus400",
        &full["CreateFcmPositionTransferStatus400"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus401>(
        "CreateFcmPositionTransferStatus401",
        &full["CreateFcmPositionTransferStatus401"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus403>(
        "CreateFcmPositionTransferStatus403",
        &full["CreateFcmPositionTransferStatus403"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus404>(
        "CreateFcmPositionTransferStatus404",
        &full["CreateFcmPositionTransferStatus404"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus409>(
        "CreateFcmPositionTransferStatus409",
        &full["CreateFcmPositionTransferStatus409"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus429>(
        "CreateFcmPositionTransferStatus429",
        &full["CreateFcmPositionTransferStatus429"],
    );
    mutations += model_support::assert_nulls::<CreateFcmPositionTransferStatus500>(
        "CreateFcmPositionTransferStatus500",
        &full["CreateFcmPositionTransferStatus500"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalCutoffStatus200>(
        "GetHistoricalCutoffStatus200",
        &full["GetHistoricalCutoffStatus200"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksHistoricalParameters>(
        "GetMarketCandlesticksHistoricalParameters",
        &full["GetMarketCandlesticksHistoricalParameters"],
    );
    mutations += model_support::assert_nulls::<GetMarketCandlesticksHistoricalStatus200>(
        "GetMarketCandlesticksHistoricalStatus200",
        &full["GetMarketCandlesticksHistoricalStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFillsHistoricalParameters>(
        "GetFillsHistoricalParameters",
        &full["GetFillsHistoricalParameters"],
    );
    mutations += model_support::assert_nulls::<GetFillsHistoricalStatus200>(
        "GetFillsHistoricalStatus200",
        &full["GetFillsHistoricalStatus200"],
    );
    mutations += model_support::assert_nulls::<GetFillsHistoricalStatus404>(
        "GetFillsHistoricalStatus404",
        &full["GetFillsHistoricalStatus404"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalOrdersParameters>(
        "GetHistoricalOrdersParameters",
        &full["GetHistoricalOrdersParameters"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalOrdersStatus200>(
        "GetHistoricalOrdersStatus200",
        &full["GetHistoricalOrdersStatus200"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalOrdersStatus400>(
        "GetHistoricalOrdersStatus400",
        &full["GetHistoricalOrdersStatus400"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalOrdersStatus401>(
        "GetHistoricalOrdersStatus401",
        &full["GetHistoricalOrdersStatus401"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalOrdersStatus500>(
        "GetHistoricalOrdersStatus500",
        &full["GetHistoricalOrdersStatus500"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalPositionsParameters>(
        "GetHistoricalPositionsParameters",
        &full["GetHistoricalPositionsParameters"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalPositionsStatus200>(
        "GetHistoricalPositionsStatus200",
        &full["GetHistoricalPositionsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalPositionsStatus400>(
        "GetHistoricalPositionsStatus400",
        &full["GetHistoricalPositionsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalPositionsStatus401>(
        "GetHistoricalPositionsStatus401",
        &full["GetHistoricalPositionsStatus401"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalPositionsStatus500>(
        "GetHistoricalPositionsStatus500",
        &full["GetHistoricalPositionsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetTradesHistoricalParameters>(
        "GetTradesHistoricalParameters",
        &full["GetTradesHistoricalParameters"],
    );
    mutations += model_support::assert_nulls::<GetTradesHistoricalStatus200>(
        "GetTradesHistoricalStatus200",
        &full["GetTradesHistoricalStatus200"],
    );
    mutations += model_support::assert_nulls::<GetTradesHistoricalStatus400>(
        "GetTradesHistoricalStatus400",
        &full["GetTradesHistoricalStatus400"],
    );
    mutations += model_support::assert_nulls::<GetTradesHistoricalStatus404>(
        "GetTradesHistoricalStatus404",
        &full["GetTradesHistoricalStatus404"],
    );
    mutations += model_support::assert_nulls::<GetTradesHistoricalStatus500>(
        "GetTradesHistoricalStatus500",
        &full["GetTradesHistoricalStatus500"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketsParameters>(
        "GetHistoricalMarketsParameters",
        &full["GetHistoricalMarketsParameters"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketsStatus200>(
        "GetHistoricalMarketsStatus200",
        &full["GetHistoricalMarketsStatus200"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketsStatus400>(
        "GetHistoricalMarketsStatus400",
        &full["GetHistoricalMarketsStatus400"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketsStatus500>(
        "GetHistoricalMarketsStatus500",
        &full["GetHistoricalMarketsStatus500"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketParameters>(
        "GetHistoricalMarketParameters",
        &full["GetHistoricalMarketParameters"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketStatus200>(
        "GetHistoricalMarketStatus200",
        &full["GetHistoricalMarketStatus200"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketStatus404>(
        "GetHistoricalMarketStatus404",
        &full["GetHistoricalMarketStatus404"],
    );
    mutations += model_support::assert_nulls::<GetHistoricalMarketStatus500>(
        "GetHistoricalMarketStatus500",
        &full["GetHistoricalMarketStatus500"],
    );
    println!("Recursive null mutations verified: {mutations}");
}
