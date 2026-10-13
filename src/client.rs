use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value;

use crate::auth::{KalshiAuth, sign_kalshi_request};
use crate::environment::{KalshiEnvironment, PRODUCTION};
use crate::error::{KalshiApiError, KalshiError};
use crate::operation::{
    PathParameters, QueryParameters, build_operation_path, build_trade_api_path, operation_or_error,
};
use crate::transport::{
    CancellationToken, LocalRateLimiter, RequestContext, RequestOptions, ResponseBytes,
    TransportOptions, attempts_for_method, consume, retry_delay, retryable,
};

#[derive(Clone)]
pub struct KalshiClient {
    environment: KalshiEnvironment,
    auth: Option<Arc<KalshiAuth>>,
    http: reqwest::Client,
    transport: Arc<ClientTransport>,
    options: RequestOptions,
    cancellations: Vec<CancellationToken>,
}

struct ClientTransport {
    options: TransportOptions,
    limiter: Option<Arc<LocalRateLimiter>>,
}

pub struct KalshiClientBuilder {
    environment: KalshiEnvironment,
    auth: Option<KalshiAuth>,
    http: reqwest::ClientBuilder,
    options: TransportOptions,
    limiter: Option<Arc<LocalRateLimiter>>,
}

impl KalshiClientBuilder {
    pub fn auth(mut self, auth: KalshiAuth) -> Self {
        self.auth = Some(auth);
        self
    }
    /// The SDK applies redirect, retry and content-decoding guards after customization.
    pub fn http_builder(
        mut self,
        configure: impl FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder,
    ) -> Self {
        self.http = configure(self.http);
        self
    }
    pub fn transport_options(mut self, options: TransportOptions) -> Self {
        self.options = options;
        self
    }
    pub fn rate_limiter(mut self, limiter: Arc<LocalRateLimiter>) -> Self {
        self.limiter = Some(limiter);
        self
    }
    pub fn build(self) -> Result<KalshiClient, KalshiError> {
        self.options.validate()?;
        let http = self
            .http
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .referer(false)
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()?;
        Ok(KalshiClient {
            environment: self.environment,
            auth: self.auth.map(Arc::new),
            http,
            transport: Arc::new(ClientTransport {
                options: self.options,
                limiter: self.limiter,
            }),
            options: RequestOptions::default(),
            cancellations: Vec::new(),
        })
    }
}

impl std::fmt::Debug for KalshiClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KalshiClient")
            .field("authenticated", &self.auth.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct KalshiRequest {
    pub path_parameters: PathParameters,
    pub query: QueryParameters,
    pub body: Option<Value>,
    pub headers: BTreeMap<String, String>,
    pub authenticated: bool,
}
impl std::fmt::Debug for KalshiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KalshiRequest")
            .field("authenticated", &self.authenticated)
            .finish_non_exhaustive()
    }
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
            .field("url", &"[REDACTED]")
            .field("headers", &"[REDACTED]")
            .field("body", &"[REDACTED]")
            .finish()
    }
}

impl KalshiClient {
    pub fn new(environment: KalshiEnvironment, auth: Option<KalshiAuth>) -> Self {
        Self::with_http_client_builder(environment, auth, |http| http)
            .expect("default Kalshi HTTP client configuration")
    }

    pub fn builder(environment: KalshiEnvironment) -> KalshiClientBuilder {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT_ENCODING,
            reqwest::header::HeaderValue::from_static("identity"),
        );
        KalshiClientBuilder {
            environment,
            auth: None,
            http: reqwest::Client::builder().default_headers(headers),
            options: TransportOptions::default(),
            limiter: None,
        }
    }

    pub fn with_http_client_builder(
        environment: KalshiEnvironment,
        auth: Option<KalshiAuth>,
        configure: impl FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder,
    ) -> Result<Self, KalshiError> {
        let mut builder = Self::builder(environment).http_builder(configure);
        if let Some(auth) = auth {
            builder = builder.auth(auth);
        }
        builder.build()
    }

    /// A cheap configured clone. Absolute deadlines and cancellation remain inherited.
    pub fn with_request_options(&self, options: RequestOptions) -> Self {
        let mut client = self.clone();
        if let Some(token) = &options.cancellation {
            client.cancellations.push(token.clone());
        }
        client.options = client.options.merge(options);
        client
    }

    pub(crate) fn request_context(&self) -> Result<RequestContext, KalshiError> {
        RequestContext::new(&self.transport.options, &self.options, &self.cancellations)
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
                .map_err(|_| KalshiError::InvalidHeader("request header".into()))?;
            reqwest::header::HeaderValue::from_str(&value)
                .map_err(|_| KalshiError::InvalidHeader("request header".into()))?;
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
        self.request_with_options(operation_id, request, RequestOptions::default())
            .await
    }

    pub async fn request_with_options(
        &self,
        operation_id: &str,
        request: KalshiRequest,
        options: RequestOptions,
    ) -> Result<Value, KalshiError> {
        let client = self.with_request_options(options);
        let context = client.request_context()?;
        context
            .run(client.request_in_context(operation_id, request, &context))
            .await
    }

    pub(crate) async fn request_in_context(
        &self,
        operation_id: &str,
        request: KalshiRequest,
        context: &RequestContext,
    ) -> Result<Value, KalshiError> {
        let response = self.execute_request(operation_id, request, context).await?;
        if response.decoded.is_empty() {
            return Ok(Value::Null);
        }
        Ok(serde_json::from_slice(&response.decoded)?)
    }

    pub(crate) async fn request_unspecified_in_context(
        &self,
        operation_id: &str,
        request: KalshiRequest,
        context: &RequestContext,
    ) -> Result<crate::typed::UnspecifiedResponse, KalshiError> {
        let response = self.execute_request(operation_id, request, context).await?;
        Ok(crate::typed::UnspecifiedResponse {
            status: response.status,
            headers: response.headers,
            body: unstructured_body(&response.decoded),
            raw_body: response.raw,
        })
    }

    fn execute_request<'a>(
        &'a self,
        operation_id: &'a str,
        request: KalshiRequest,
        context: &'a RequestContext,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<ResponseBytes, KalshiError>> + Send + 'a>,
    > {
        // Keep codec/HTTP state out of every generated method's future stack frame.
        Box::pin(async move {
            let operation = operation_or_error(operation_id)?;
            if request.authenticated && self.auth.is_none() {
                return Err(KalshiError::MissingAuth);
            }
            // Fail invalid URL/header/body construction before spending rate capacity or signing.
            let mut validation = request.clone();
            validation.authenticated = false;
            self.build_http_request(self.prepare_request(operation_id, validation, "0")?)?;
            context.check()?;
            let retry = self
                .options
                .retry
                .as_ref()
                .unwrap_or(&self.transport.options.retry);
            let attempts = attempts_for_method(operation.method, retry);
            for attempt in 1..=attempts {
                if let Some(limiter) = &self.transport.limiter {
                    limiter
                        .acquire(operation_id, operation.method, context)
                        .await?;
                }
                context.check()?;
                let prepared =
                    self.prepare_request(operation_id, request.clone(), &current_timestamp_ms())?;
                let request = self.build_http_request(prepared)?;
                context.check()?;
                let execute = async {
                    let response = self.http.execute(request).await?;
                    let response =
                        consume(response, self.transport.options.max_response_bytes, context)
                            .await?;
                    if !response.status.is_success() {
                        return Err(KalshiError::Api(Box::new(KalshiApiError {
                            status: response.status,
                            headers: response.headers,
                            body: unstructured_body(&response.decoded),
                            raw_body: response.raw,
                        })));
                    }
                    Ok(response)
                };
                let result = match self.transport.options.attempt_timeout {
                    Some(timeout) => tokio::time::timeout(timeout, execute)
                        .await
                        .unwrap_or(Err(KalshiError::AttemptTimeout)),
                    None => execute.await,
                };
                context.check()?;
                match result {
                    Ok(response) => return Ok(response),
                    Err(error) => {
                        if attempt == attempts || !retryable(&error) {
                            return Err(error);
                        }
                        let Some(delay) = retry_delay(&error, retry.backoff(attempt)) else {
                            return Err(error);
                        };
                        if !context.fits(delay) {
                            return Err(error);
                        }
                        tokio::time::sleep(delay).await;
                        context.check()?;
                    }
                }
            }
            unreachable!("validated retry count is positive")
        })
    }

    fn build_http_request(
        &self,
        prepared: PreparedKalshiRequest,
    ) -> Result<reqwest::Request, KalshiError> {
        let method = prepared
            .method
            .parse::<reqwest::Method>()
            .map_err(|error| KalshiError::Fixture(error.to_string()))?;
        let mut builder = self.http.request(method, prepared.url);
        for (name, value) in prepared.headers {
            let name = reqwest::header::HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| KalshiError::InvalidHeader("request header".into()))?;
            let mut value = reqwest::header::HeaderValue::from_str(&value)
                .map_err(|_| KalshiError::InvalidHeader("request header".into()))?;
            value.set_sensitive(true);
            builder = builder.header(name, value);
        }
        if let Some(body) = prepared.body {
            builder = builder.json(&body);
        }
        Ok(builder.build()?)
    }
}

fn unstructured_body(bytes: &[u8]) -> Value {
    if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(bytes).into_owned()))
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
