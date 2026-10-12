use std::collections::BTreeMap;

use crate::error::KalshiError;
use crate::generated::operations::{KalshiOperation, get_operation};

pub type PathParameters = BTreeMap<String, String>;
pub type QueryParameters = Vec<(String, QueryValue)>;

#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    String(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<QueryValue>),
}

impl From<&str> for QueryValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i64> for QueryValue {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<u32> for QueryValue {
    fn from(value: u32) -> Self {
        Self::Number(value.into())
    }
}

impl From<bool> for QueryValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub fn operation_or_error(operation_id: &str) -> Result<&'static KalshiOperation, KalshiError> {
    get_operation(operation_id)
        .ok_or_else(|| KalshiError::UnknownOperation(operation_id.to_string()))
}

pub fn build_operation_path(
    operation: &KalshiOperation,
    path_parameters: &PathParameters,
    query: &QueryParameters,
) -> Result<String, KalshiError> {
    let mut path = operation.path.to_string();
    for parameter in path_parameter_names(operation.path) {
        let value =
            path_parameters
                .get(parameter)
                .ok_or_else(|| KalshiError::MissingPathParameter {
                    operation_id: operation.operation_id.to_string(),
                    parameter: parameter.to_string(),
                })?;
        path = path.replace(&format!("{{{parameter}}}"), &urlencoding::encode(value));
    }

    let unused: Vec<_> = path_parameters
        .keys()
        .filter(|name| !operation.path.contains(&format!("{{{}}}", name)))
        .cloned()
        .collect();
    if !unused.is_empty() {
        return Err(KalshiError::UnusedPathParameters {
            operation_id: operation.operation_id.to_string(),
            parameters: unused.join(", "),
        });
    }

    let query_string = serialize_operation_query(operation, query);
    if query_string.is_empty() {
        Ok(path)
    } else {
        Ok(format!("{path}?{query_string}"))
    }
}

pub fn build_trade_api_path(path_and_query: &str) -> String {
    let normalized = if path_and_query.starts_with('/') {
        path_and_query.to_string()
    } else {
        format!("/{path_and_query}")
    };

    if normalized.starts_with("/trade-api/v2/") {
        normalized
    } else {
        format!("/trade-api/v2{normalized}")
    }
}

pub fn serialize_query(query: &QueryParameters) -> String {
    query
        .iter()
        .map(|(name, value)| {
            format!(
                "{}={}",
                urlencoding::encode(name),
                urlencoding::encode(&query_value_to_string(value))
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

pub fn serialize_operation_query(operation: &KalshiOperation, query: &QueryParameters) -> String {
    query
        .iter()
        .flat_map(|(name, value)| {
            let values = if operation.repeated_query_parameters.contains(&name.as_str()) {
                match value {
                    QueryValue::Array(values) => values.iter().collect::<Vec<_>>(),
                    value => vec![value],
                }
            } else {
                vec![value]
            };
            values
                .into_iter()
                .map(|value| {
                    format!(
                        "{}={}",
                        urlencoding::encode(name),
                        urlencoding::encode(&query_value_to_string(value))
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn query_value_to_string(value: &QueryValue) -> String {
    match value {
        QueryValue::String(value) => value.clone(),
        QueryValue::Number(value) => value.to_string(),
        QueryValue::Float(value) => value.to_string(),
        QueryValue::Bool(value) => value.to_string(),
        QueryValue::Array(values) => values
            .iter()
            .map(query_value_to_string)
            .collect::<Vec<_>>()
            .join(","),
    }
}

fn path_parameter_names(path: &str) -> Vec<&str> {
    path.split('{')
        .skip(1)
        .filter_map(|part| part.split('}').next())
        .collect()
}
