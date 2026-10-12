//! Shared schema validation and cursor pagination for the generated typed API.
use crate::{KalshiClient, KalshiError, QueryValue};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    sync::{Mutex, OnceLock},
};

/// A successful response whose body is not declared by the official contract.
/// JSON is preserved as JSON; an undeclared text body is preserved as a string.
pub struct UnspecifiedResponse {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: Value,
    /// Original response bytes, including bodies that are not valid UTF-8.
    pub raw_body: Vec<u8>,
}
impl std::fmt::Debug for UnspecifiedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnspecifiedResponse")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

pub(crate) fn optional_nullable<'de, D: serde::Deserializer<'de>, T: serde::Deserialize<'de>>(
    d: D,
) -> Result<Option<Option<T>>, D::Error> {
    serde::Deserialize::deserialize(d).map(Some)
}
pub(crate) fn required_nullable<'de, D: serde::Deserializer<'de>, T: serde::Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    serde::Deserialize::deserialize(d)
}
pub(crate) fn optional_non_null<'de, D: serde::Deserializer<'de>, T: serde::Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

/// A typed documented error payload alongside the original structured transport error.
pub struct TypedError<E> {
    pub source: KalshiError,
    pub payload: Option<E>,
}
impl<E> TypedError<E> {
    pub fn new(source: KalshiError, payload: Option<E>) -> Self {
        Self { source, payload }
    }
}
impl<E> std::fmt::Debug for TypedError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TypedError([REDACTED])")
    }
}
impl<E> std::fmt::Display for TypedError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kalshi typed request failed")
    }
}
impl<E> std::error::Error for TypedError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

pub(crate) fn contract_error(message: &str) -> KalshiError {
    KalshiError::Contract(message.into())
}

/// Validate a model using the normalized official schema. Diagnostics exclude
/// input values because bodies may contain credentials.
pub fn validate(name: &str, value: &Value) -> Result<(), KalshiError> {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    static VALIDATORS: OnceLock<Mutex<std::collections::HashMap<String, jsonschema::Validator>>> =
        OnceLock::new();
    let schema = SCHEMA.get_or_init(|| {
        serde_json::from_str(include_str!("generated/typed-schema.json")).expect("generated schema")
    });
    if schema["definitions"].get(name).is_none() {
        return Err(contract_error("unknown typed schema"));
    }
    let mut validators = VALIDATORS
        .get_or_init(Default::default)
        .lock()
        .map_err(|_| contract_error("schema validator unavailable"))?;
    if !validators.contains_key(name) {
        let root = serde_json::json!({"$schema":"http://json-schema.org/draft-07/schema#", "$ref": format!("#/definitions/{name}"), "definitions": schema["definitions"]});
        let validator = jsonschema::options()
            .should_validate_formats(true)
            .build(&root)
            .map_err(|_| contract_error("invalid generated schema"))?;
        validators.insert(name.into(), validator);
    }
    if validators[name].is_valid(value) {
        Ok(())
    } else {
        Err(contract_error(&format!("payload violates {name} schema")))
    }
}

pub(crate) fn decode_model<T: DeserializeOwned>(
    name: &str,
    value: Value,
) -> Result<T, KalshiError> {
    validate(name, &value)?;
    serde_json::from_value(value)
        .map_err(|_| contract_error(&format!("payload cannot be decoded as {name}")))
}

pub(crate) fn parameter_text(value: &Value) -> Result<String, KalshiError> {
    match value {
        Value::String(v) => Ok(v.clone()),
        Value::Number(v) => Ok(v.to_string()),
        Value::Bool(v) => Ok(v.to_string()),
        _ => Err(contract_error("parameter must be scalar")),
    }
}
pub(crate) fn query_value(value: &Value) -> Result<QueryValue, KalshiError> {
    match value {
        Value::Array(v) => Ok(QueryValue::Array(
            v.iter().map(query_value).collect::<Result<_, _>>()?,
        )),
        _ => Ok(QueryValue::String(parameter_text(value)?)),
    }
}

type PageFuture<'a, R, E> = Pin<Box<dyn Future<Output = Result<R, TypedError<E>>> + Send + 'a>>;
type FetchPage<'a, P, R, E> = fn(&'a KalshiClient, P) -> PageFuture<'a, R, E>;

/// Lazy cursor pagination. Requests can be retried after a transport failure.
pub struct Pager<'a, P, R, E> {
    client: &'a KalshiClient,
    parameters: P,
    fetch: FetchPage<'a, P, R, E>,
    set_cursor: fn(&mut P, String),
    cursor: fn(&R) -> Option<String>,
    seen: HashSet<String>,
    done: bool,
}
impl<'a, P: Clone + Serialize, R, E> Pager<'a, P, R, E> {
    pub(crate) fn new(
        client: &'a KalshiClient,
        parameters: P,
        fetch: FetchPage<'a, P, R, E>,
        set_cursor: fn(&mut P, String),
        cursor: fn(&R) -> Option<String>,
    ) -> Self {
        let mut seen = HashSet::new();
        if let Ok(value) = serde_json::to_value(&parameters)
            && let Some(cursor) = value
                .get("cursor")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
        {
            seen.insert(cursor.into());
        }
        Self {
            client,
            parameters,
            fetch,
            set_cursor,
            cursor,
            seen,
            done: false,
        }
    }
    pub async fn next_page(&mut self) -> Result<Option<R>, TypedError<E>> {
        if self.done {
            return Ok(None);
        }
        let page = (self.fetch)(self.client, self.parameters.clone()).await?;
        if let Some(cursor) = (self.cursor)(&page) {
            if !self.seen.insert(cursor.clone()) {
                self.done = true;
                return Err(TypedError::new(
                    contract_error("pagination cursor repeated"),
                    None,
                ));
            }
            (self.set_cursor)(&mut self.parameters, cursor);
        } else {
            self.done = true;
        }
        Ok(Some(page))
    }
    pub async fn collect_pages(mut self) -> Result<Vec<R>, TypedError<E>> {
        let mut pages = Vec::new();
        while let Some(page) = self.next_page().await? {
            pages.push(page);
        }
        Ok(pages)
    }
}
