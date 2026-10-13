use thiserror::Error;

pub struct KalshiApiError {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: serde_json::Value,
    /// Complete encoded HTTP entity bytes, before content decoding.
    pub raw_body: Vec<u8>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFailureReason {
    Read,
    EncodedSizeLimit,
    DecodedSizeLimit,
    UnsupportedEncoding,
    TooManyEncodingLayers,
    MalformedEncoding,
}

/// A response that could not be completely read or content-decoded.
pub struct KalshiResponseError {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub raw_body: Vec<u8>,
    /// False means raw_body is only the retained, bounded prefix.
    pub raw_body_complete: bool,
    pub reason: ResponseFailureReason,
    pub(crate) read_error: Option<reqwest::Error>,
}

impl std::fmt::Debug for KalshiResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KalshiResponseError")
            .field("status", &self.status)
            .field("reason", &self.reason)
            .field("raw_body_complete", &self.raw_body_complete)
            .finish_non_exhaustive()
    }
}
impl std::fmt::Display for KalshiResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP {} response could not be consumed ({:?})",
            self.status, self.reason
        )
    }
}
impl std::error::Error for KalshiResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.read_error.as_ref().map(|error| error as _)
    }
}

#[derive(Debug, Error)]
pub enum KalshiError {
    #[error("Kalshi request cancelled")]
    Cancelled,
    #[error("Kalshi request total deadline exceeded")]
    DeadlineExceeded,
    #[error("Kalshi HTTP attempt timed out")]
    AttemptTimeout,
    #[error("invalid transport configuration: {0}")]
    InvalidTransportOptions(&'static str),
    #[error(transparent)]
    Response(Box<KalshiResponseError>),
    #[error("typed contract error: {0}")]
    Contract(String),
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

    #[error("invalid Kalshi signing credentials")]
    InvalidSigningCredentials,

    #[error("Kalshi request signing failed")]
    SigningFailed,

    #[error("HTTP error: {0}")]
    Http(#[source] reqwest::Error),

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

impl From<reqwest::Error> for KalshiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
    }
}
