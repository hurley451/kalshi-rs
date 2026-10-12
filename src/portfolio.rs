use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::KalshiError;
use crate::operation::{QueryParameters, QueryValue, serialize_query};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FillQuery {
    pub ticker: Option<String>,
    pub tickers: Vec<String>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
    pub order_id: Option<String>,
    pub min_ts: Option<i64>,
    pub max_ts: Option<i64>,
    pub subaccount: Option<u32>,
    pub exchange_index: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FillsPage {
    pub fills: Vec<Fill>,
    pub cursor: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fill {
    #[serde(rename = "fill_id")]
    pub fill_id: String,
    #[serde(rename = "exchange_index")]
    pub exchange_index: i64,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    pub ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: String,
    #[serde(rename = "count_fp")]
    pub count_fp: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "fee_cost")]
    pub fee_cost: String,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    pub side: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    pub action: Option<String>,
    #[serde(rename = "is_taker")]
    pub is_taker: bool,
    #[serde(rename = "created_time")]
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_time: Option<String>,
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: Option<i64>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    pub ts: Option<i64>,
}

fn deserialize_present<'de, T: Deserialize<'de>, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

pub fn build_portfolio_fills_path(query: &FillQuery) -> Result<String, KalshiError> {
    let params = portfolio_fills_query(query)?;
    let serialized = serialize_query(&params);
    if serialized.is_empty() {
        Ok("/trade-api/v2/portfolio/fills".to_string())
    } else {
        Ok(format!("/trade-api/v2/portfolio/fills?{serialized}"))
    }
}

pub fn parse_fills_page(value: Value) -> Result<FillsPage, KalshiError> {
    let page: FillsPage = serde_json::from_value(value)?;
    for fill in &page.fills {
        if !["yes", "no"].contains(&fill.outcome_side.as_str())
            || !["bid", "ask"].contains(&fill.book_side.as_str())
            || fill
                .side
                .as_deref()
                .is_some_and(|side| !["yes", "no"].contains(&side))
            || fill
                .action
                .as_deref()
                .is_some_and(|action| !["buy", "sell"].contains(&action))
            || fill
                .created_time
                .as_deref()
                .is_some_and(|value| chrono::DateTime::parse_from_rfc3339(value).is_err())
        {
            return Err(KalshiError::Fixture(
                "Invalid fill direction or action.".into(),
            ));
        }
    }
    Ok(page)
}

pub fn portfolio_fills_query(query: &FillQuery) -> Result<QueryParameters, KalshiError> {
    if query
        .limit
        .is_some_and(|limit| !(1..=1000).contains(&limit))
    {
        return Err(KalshiError::Fixture(
            "Fill limit must be between 1 and 1000.".into(),
        ));
    }
    if query.subaccount.is_some_and(|value| value > 63) {
        return Err(KalshiError::Fixture(
            "Subaccount must be between 0 and 63.".into(),
        ));
    }
    if query.ticker.is_some() && !query.tickers.is_empty() {
        return Err(KalshiError::Fixture(
            "Specify either ticker or tickers, not both.".to_string(),
        ));
    }
    if query.tickers.len() > 100 {
        return Err(KalshiError::Fixture(
            "At most 100 market tickers can be supplied.".to_string(),
        ));
    }
    if query.tickers.iter().any(|ticker| ticker.trim().is_empty()) {
        return Err(KalshiError::Fixture(
            "Market tickers cannot contain empty values.".to_string(),
        ));
    }

    let mut params: QueryParameters = Vec::new();
    insert_optional_u32(&mut params, "limit", query.limit);
    insert_optional_string(&mut params, "cursor", query.cursor.clone());
    if !query.tickers.is_empty() {
        params.push((
            "ticker".to_string(),
            QueryValue::Array(
                query
                    .tickers
                    .iter()
                    .cloned()
                    .map(QueryValue::String)
                    .collect(),
            ),
        ));
    } else {
        insert_optional_string(&mut params, "ticker", query.ticker.clone());
    }
    insert_optional_string(&mut params, "order_id", query.order_id.clone());
    insert_optional_u32(&mut params, "subaccount", query.subaccount);
    insert_optional_u32(&mut params, "exchange_index", query.exchange_index);
    insert_optional_i64(&mut params, "min_ts", query.min_ts);
    insert_optional_i64(&mut params, "max_ts", query.max_ts);
    Ok(params)
}

fn insert_optional_string(params: &mut QueryParameters, name: &str, value: Option<String>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        params.push((name.to_string(), QueryValue::String(value)));
    }
}

fn insert_optional_u32(params: &mut QueryParameters, name: &str, value: Option<u32>) {
    if let Some(value) = value {
        params.push((name.to_string(), QueryValue::Number(i64::from(value))));
    }
}

fn insert_optional_i64(params: &mut QueryParameters, name: &str, value: Option<i64>) {
    if let Some(value) = value {
        params.push((name.to_string(), QueryValue::Number(value)));
    }
}
