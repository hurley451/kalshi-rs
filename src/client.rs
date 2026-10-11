use std::collections::BTreeMap;

use serde_json::Value;

use crate::auth::{KalshiAuth, sign_kalshi_request};
use crate::environment::{KalshiEnvironment, PRODUCTION};
use crate::error::KalshiError;
use crate::operation::{
    PathParameters, QueryParameters, build_operation_path, build_trade_api_path, operation_or_error,
};

#[derive(Debug, Clone)]
pub struct KalshiClient {
    environment: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct KalshiRequest {
    pub path_parameters: PathParameters,
    pub query: QueryParameters,
    pub body: Option<Value>,
    pub headers: BTreeMap<String, String>,
    pub authenticated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedKalshiRequest {
    pub method: String,
    pub url: String,
    pub signing_path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Option<Value>,
}

impl KalshiClient {
    pub fn new(environment: KalshiEnvironment, auth: Option<KalshiAuth>) -> Self {
        Self {
            environment,
            auth,
            http: reqwest::Client::new(),
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
        let response = builder.send().await?.error_for_status()?;
        Ok(response.json().await?)
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
