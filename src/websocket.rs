use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::KalshiError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WebSocketEnvelope {
    pub id: Option<i64>,
    pub sending_ts_ms: Option<i64>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub sid: Option<i64>,
    pub seq: Option<i64>,
    pub msg: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderBookDeltaMessage {
    pub market_ticker: String,
    pub market_id: Option<String>,
    pub price: Option<i64>,
    pub delta: Option<i64>,
    pub price_dollars: Option<String>,
    pub delta_fp: Option<String>,
    pub side: String,
}

pub fn parse_websocket_message(value: Value) -> Result<WebSocketEnvelope, KalshiError> {
    Ok(serde_json::from_value(value)?)
}
