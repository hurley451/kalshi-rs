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

impl crate::transport::RequestContext {
    pub(crate) fn run_typed<'a, T: Send + 'a, E: Send + 'a>(
        &'a self,
        future: impl Future<Output = Result<T, KalshiError>> + Send + 'a,
        decode_error: impl FnOnce(&KalshiError) -> Option<E> + Send + 'a,
    ) -> Pin<Box<dyn Future<Output = Result<T, TypedError<E>>> + Send + 'a>> {
        // Box the shared pipeline, not an executor task, to bound generated caller stack frames.
        Box::pin(async move {
            // Include synchronous model validation and error decoding before the final control check.
            self.run(async {
                Ok(future.await.map_err(|source| {
                    let payload = decode_error(&source);
                    TypedError::new(source, payload)
                }))
            })
            .await
            .map_err(|source| TypedError::new(source, None))?
        })
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
type FetchPage<P, R, E> = for<'call> fn(&'call KalshiClient, P) -> PageFuture<'call, R, E>;

/// Lazy cursor pagination. Requests can be retried after a transport failure.
pub struct Pager<'a, P, R, E> {
    client: &'a KalshiClient,
    parameters: P,
    fetch: FetchPage<P, R, E>,
    set_cursor: fn(&mut P, String),
    cursor: fn(&R) -> Option<String>,
    seen: HashSet<String>,
    done: bool,
    traversal: crate::RequestOptions,
    traversal_cancellations: Vec<crate::CancellationToken>,
}
impl<'a, P: Clone + Serialize, R, E> Pager<'a, P, R, E> {
    pub(crate) fn new(
        client: &'a KalshiClient,
        parameters: P,
        fetch: FetchPage<P, R, E>,
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
            traversal: crate::RequestOptions::default(),
            traversal_cancellations: Vec::new(),
        }
    }
    pub async fn next_page(&mut self) -> Result<Option<R>, TypedError<E>> {
        self.next_page_with_options(crate::RequestOptions::default())
            .await
    }

    pub fn with_deadline(mut self, deadline: tokio::time::Instant) -> Self {
        self.traversal.deadline =
            crate::transport::earliest(self.traversal.deadline, Some(deadline));
        self
    }
    pub fn with_cancellation(mut self, cancellation: crate::CancellationToken) -> Self {
        self.traversal_cancellations.push(cancellation);
        self
    }
    pub async fn next_page_with_options(
        &mut self,
        options: crate::RequestOptions,
    ) -> Result<Option<R>, TypedError<E>> {
        if self.done {
            return Ok(None);
        }
        let mut client = self
            .client
            .with_request_options(self.traversal.clone())
            .with_request_options(options);
        for token in &self.traversal_cancellations {
            client = client.with_request_options(crate::RequestOptions {
                cancellation: Some(token.clone()),
                ..Default::default()
            });
        }
        let context = client
            .request_context()
            .map_err(|error| TypedError::new(error, None))?;
        // The outer page budget also checks synchronous typed validation before cursor commit.
        let page = context
            .run(async { Ok((self.fetch)(&client, self.parameters.clone()).await) })
            .await
            .map_err(|source| TypedError::new(source, None))??;
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
    pub async fn collect_pages(self) -> Result<Vec<R>, TypedError<E>> {
        self.collect_pages_with_options(crate::RequestOptions::default())
            .await
    }
    pub async fn collect_pages_with_options(
        mut self,
        options: crate::RequestOptions,
    ) -> Result<Vec<R>, TypedError<E>> {
        let mut pages = Vec::new();
        while let Some(page) = self.next_page_with_options(options.clone()).await? {
            pages.push(page);
        }
        Ok(pages)
    }
}

#[cfg(test)]
mod budget_tests {
    use super::*;
    use crate::{CancellationToken, RequestOptions, TransportOptions, transport::RequestContext};
    use std::{sync::Arc, time::Duration};

    fn context(timeout: Option<Duration>, token: Option<CancellationToken>) -> RequestContext {
        RequestContext::new(
            &TransportOptions {
                total_timeout: timeout,
                ..Default::default()
            },
            &RequestOptions::default(),
            &token.into_iter().collect::<Vec<_>>(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn synchronous_success_decode_cannot_escape_the_entry_deadline() {
        let context = context(Some(Duration::from_millis(5)), None);
        let result = context
            .run_typed(
                async {
                    std::thread::sleep(Duration::from_millis(20));
                    Ok(())
                },
                |_| None::<()>,
            )
            .await;
        assert!(matches!(
            result.unwrap_err().source,
            KalshiError::DeadlineExceeded
        ));
    }

    #[tokio::test]
    async fn synchronous_error_decode_is_checked_before_returning_a_payload() {
        let context = context(Some(Duration::from_millis(5)), None);
        let result = context
            .run_typed(async { Err::<(), _>(contract_error("test")) }, |_| {
                std::thread::sleep(Duration::from_millis(20));
                Some(())
            })
            .await
            .unwrap_err();
        assert!(matches!(result.source, KalshiError::DeadlineExceeded));
        assert!(result.payload.is_none());
    }

    #[tokio::test]
    async fn concurrent_cancellation_during_synchronous_decode_cannot_return_success() {
        let token = CancellationToken::new();
        let context = context(None, Some(token.clone()));
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let other = barrier.clone();
        let cancel = std::thread::spawn(move || {
            other.wait();
            token.cancel();
            other.wait();
        });
        let result = context
            .run_typed(
                async {
                    barrier.wait();
                    barrier.wait();
                    Ok(())
                },
                |_| None::<()>,
            )
            .await;
        cancel.join().unwrap();
        assert!(matches!(result.unwrap_err().source, KalshiError::Cancelled));
    }
}
