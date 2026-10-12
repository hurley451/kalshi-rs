use thiserror::Error;

pub struct KalshiApiError {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: serde_json::Value,
}

impl std::fmt::Debug for KalshiApiError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KalshiApiError")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for KalshiApiError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Kalshi API returned HTTP {}", self.status)
    }
}

impl std::error::Error for KalshiApiError {}

#[derive(Debug, Error)]
pub enum KalshiError {
    #[error("unknown Kalshi operation: {0}")]
    UnknownOperation(String),

    #[error("missing path parameter `{parameter}` for {operation_id}")]
    MissingPathParameter {
        operation_id: String,
        parameter: String,
    },

    #[error("unused path parameter(s) for {operation_id}: {parameters}")]
    UnusedPathParameters {
        operation_id: String,
        parameters: String,
    },

    #[error("authenticated Kalshi request requires auth options")]
    MissingAuth,

    #[error("RSA key error: {0}")]
    RsaKey(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Api(Box<KalshiApiError>),

    #[error("invalid request header: {0}")]
    InvalidHeader(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("fixture error: {0}")]
    Fixture(String),
}
