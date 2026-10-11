use thiserror::Error;

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

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("fixture error: {0}")]
    Fixture(String),
}
