use crate::{KalshiError, typed::contract_error};
use serde_json::Value;

pub(crate) fn validate_order_request(operation_id: &str, body: &Value) -> Result<(), KalshiError> {
    match operation_id {
        "CreateOrderV2" => validate_order(body),
        "BatchCreateOrdersV2" => {
            let orders = body
                .get("orders")
                .and_then(Value::as_array)
                .ok_or_else(|| contract_error("V2 batch requires orders"))?;
            for order in orders {
                validate_order(order)?;
            }
            Ok(())
        }
        _ => Err(contract_error("unknown V2 order rule binding")),
    }
}

fn validate_order(order: &Value) -> Result<(), KalshiError> {
    let ioc = order.get("time_in_force").and_then(Value::as_str) == Some("immediate_or_cancel");
    if ioc && order.get("expiration_time").is_some() {
        return Err(contract_error("IOC orders cannot include expiration_time"));
    }
    if order.get("reduce_only").and_then(Value::as_bool) == Some(true) && !ioc {
        return Err(contract_error("reduce_only requires immediate_or_cancel"));
    }
    Ok(())
}
