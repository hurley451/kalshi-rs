use std::collections::BTreeMap;

use serde_json::Value;

use crate::auth::{KalshiAuth, sign_kalshi_request};
use crate::environment::{KalshiEnvironment, PRODUCTION};
use crate::error::{KalshiApiError, KalshiError};
use crate::operation::{
    PathParameters, QueryParameters, build_operation_path, build_trade_api_path, operation_or_error,
};

#[derive(Clone)]
pub struct KalshiClient {
    environment: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    http: reqwest::Client,
}

impl std::fmt::Debug for KalshiClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KalshiClient")
            .field("environment", &self.environment)
            .field("authenticated", &self.auth.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct KalshiRequest {
    pub path_parameters: PathParameters,
    pub query: QueryParameters,
    pub body: Option<Value>,
    pub headers: BTreeMap<String, String>,
    pub authenticated: bool,
}

#[derive(Clone, PartialEq)]
pub struct PreparedKalshiRequest {
    pub method: String,
    pub url: String,
    pub signing_path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Option<Value>,
}

impl std::fmt::Debug for PreparedKalshiRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedKalshiRequest")
            .field("method", &self.method)
            .field("url", &self.url)
            .field("headers", &"[REDACTED]")
            .field("body", &"[REDACTED]")
            .finish()
    }
}

impl KalshiClient {
    pub fn new(environment: KalshiEnvironment, auth: Option<KalshiAuth>) -> Self {
        Self::with_http_client(environment, auth, reqwest::Client::new())
    }

    pub fn with_http_client(
        environment: KalshiEnvironment,
        auth: Option<KalshiAuth>,
        http: reqwest::Client,
    ) -> Self {
        Self {
            environment,
            auth,
            http,
        }
    }

    pub fn production(auth: KalshiAuth) -> Self {
        Self::new(PRODUCTION, Some(auth))
    }

    pub fn prepare_request(
        &self,
        operation_id: &str,
        mut request: KalshiRequest,
        timestamp_ms: &str,
    ) -> Result<PreparedKalshiRequest, KalshiError> {
        let operation = operation_or_error(operation_id)?;
        let path_and_query =
            build_operation_path(operation, &request.path_parameters, &request.query)?;
        let signing_path = build_trade_api_path(&path_and_query);
        let url = format!(
            "{}{}",
            self.environment.rest_base_url.trim_end_matches('/'),
            signing_path.trim_start_matches("/trade-api/v2")
        );

        let mut headers = BTreeMap::new();
        for (name, value) in request.headers {
            let parsed = reqwest::header::HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| KalshiError::InvalidHeader(name.clone()))?;
            reqwest::header::HeaderValue::from_str(&value)
                .map_err(|_| KalshiError::InvalidHeader(name.clone()))?;
            let canonical = match parsed.as_str() {
                "accept" => "Accept",
                "content-type" => "Content-Type",
                "kalshi-access-key" => "KALSHI-ACCESS-KEY",
                "kalshi-access-timestamp" => "KALSHI-ACCESS-TIMESTAMP",
                "kalshi-access-signature" => "KALSHI-ACCESS-SIGNATURE",
                other => other,
            };
            headers.insert(canonical.to_string(), value);
        }
        request.headers = headers;
        request
            .headers
            .entry("Accept".to_string())
            .or_insert_with(|| "application/json".to_string());
        if request.body.is_some() {
            request
                .headers
                .entry("Content-Type".to_string())
                .or_insert_with(|| "application/json".to_string());
        }

        if request.authenticated {
            let auth = self.auth.as_ref().ok_or(KalshiError::MissingAuth)?;
            request.headers.extend(
                sign_kalshi_request(auth, operation.method, &signing_path, timestamp_ms)?.headers,
            );
        }

        Ok(PreparedKalshiRequest {
            method: operation.method.to_string(),
            url,
            signing_path,
            headers: request.headers,
            body: request.body,
        })
    }

    pub async fn request(
        &self,
        operation_id: &str,
        request: KalshiRequest,
    ) -> Result<Value, KalshiError> {
        let (_, _, bytes) = self.execute_request(operation_id, request).await?;
        if bytes.is_empty() {
            return Ok(Value::Null);
        }
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub(crate) async fn request_unspecified(
        &self,
        operation_id: &str,
        request: KalshiRequest,
    ) -> Result<crate::typed::UnspecifiedResponse, KalshiError> {
        let (status, headers, bytes) = self.execute_request(operation_id, request).await?;
        let body = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()))
        };
        Ok(crate::typed::UnspecifiedResponse {
            status,
            headers,
            body,
            raw_body: bytes,
        })
    }

    async fn execute_request(
        &self,
        operation_id: &str,
        request: KalshiRequest,
    ) -> Result<(reqwest::StatusCode, reqwest::header::HeaderMap, Vec<u8>), KalshiError> {
        let timestamp_ms = current_timestamp_ms();
        let prepared = self.prepare_request(operation_id, request, &timestamp_ms)?;
        let method = prepared
            .method
            .parse::<reqwest::Method>()
            .map_err(|error| KalshiError::Fixture(error.to_string()))?;
        let mut builder = self.http.request(method, prepared.url);
        for (name, value) in prepared.headers {
            builder = builder.header(name, value);
        }
        if let Some(body) = prepared.body {
            builder = builder.json(&body);
        }
        let response = builder.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.bytes().await?;
        if !status.is_success() {
            let body = if bytes.is_empty() {
                Value::Null
            } else {
                serde_json::from_slice(&bytes)
                    .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()))
            };
            return Err(KalshiError::Api(Box::new(KalshiApiError {
                status,
                headers,
                body,
            })));
        }
        Ok((status, headers, bytes.to_vec()))
    }
}

impl Default for KalshiRequest {
    fn default() -> Self {
        Self {
            path_parameters: BTreeMap::new(),
            query: Vec::new(),
            body: None,
            headers: BTreeMap::new(),
            authenticated: true,
        }
    }
}

fn current_timestamp_ms() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}
