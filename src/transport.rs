//! Shared, bounded request controls. All waits run in the caller's task.
use std::{collections::BTreeMap, future::Future, num::NonZeroU32, sync::Arc, time::Duration};

use async_compression::tokio::bufread::{BrotliDecoder, GzipDecoder, ZlibDecoder, ZstdDecoder};
pub use governor::Quota as RateQuota;
use governor::{DefaultDirectRateLimiter, clock::Clock};
use rand::Rng;
use tokio::{
    io::{AsyncRead, AsyncReadExt, BufReader},
    time::Instant,
};
pub use tokio_util::sync::CancellationToken;

use crate::{
    KalshiError, KalshiResponseError, ResponseFailureReason, operation::operation_or_error,
};

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    /// Includes the first attempt. Mutations always have one attempt.
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}
impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 1,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
        }
    }
}
impl RetryPolicy {
    pub fn safe_get(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Self::default()
        }
    }
    pub(crate) fn validate(&self) -> Result<(), KalshiError> {
        if !(1..=32).contains(&self.max_attempts)
            || self.base_delay.is_zero()
            || self.max_delay < self.base_delay
            || Instant::now().checked_add(self.max_delay).is_none()
        {
            return Err(KalshiError::InvalidTransportOptions(
                "retry attempts must be 1..=32 and backoff must be positive, ordered and representable",
            ));
        }
        Ok(())
    }
    pub(crate) fn backoff(&self, attempt: u32) -> Duration {
        let cap = self
            .base_delay
            .saturating_mul(
                1u32.checked_shl(attempt.saturating_sub(1))
                    .unwrap_or(u32::MAX),
            )
            .min(self.max_delay);
        jitter(cap, rand::thread_rng().gen_range(0.0..1.0))
    }
}
fn jitter(cap: Duration, fraction: f64) -> Duration {
    cap.mul_f64(fraction)
}

#[derive(Clone, Debug)]
pub struct TransportOptions {
    /// None explicitly disables the default total budget.
    pub total_timeout: Option<Duration>,
    pub attempt_timeout: Option<Duration>,
    pub retry: RetryPolicy,
    /// Applied to encoded bytes and independently to each decoded layer.
    pub max_response_bytes: usize,
}
impl Default for TransportOptions {
    fn default() -> Self {
        Self {
            total_timeout: Some(Duration::from_secs(30)),
            attempt_timeout: None,
            retry: RetryPolicy::default(),
            max_response_bytes: 64 * 1024 * 1024,
        }
    }
}
impl TransportOptions {
    pub(crate) fn validate(&self) -> Result<(), KalshiError> {
        self.retry.validate()?;
        if self.max_response_bytes == 0 {
            return Err(KalshiError::InvalidTransportOptions(
                "response byte limit must be positive",
            ));
        }
        for duration in [self.total_timeout, self.attempt_timeout]
            .into_iter()
            .flatten()
        {
            if Instant::now().checked_add(duration).is_none() {
                return Err(KalshiError::InvalidTransportOptions(
                    "timeout is not representable",
                ));
            }
        }
        if self.attempt_timeout.is_some_and(|v| v.is_zero()) {
            return Err(KalshiError::InvalidTransportOptions(
                "attempt timeout must be positive",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Default, Debug)]
pub struct RequestOptions {
    pub timeout: Option<Duration>,
    pub deadline: Option<Instant>,
    pub cancellation: Option<CancellationToken>,
    pub retry: Option<RetryPolicy>,
}
impl RequestOptions {
    pub(crate) fn merge(&self, other: Self) -> Self {
        Self {
            timeout: other.timeout.or(self.timeout),
            deadline: earliest(self.deadline, other.deadline),
            cancellation: other.cancellation.or_else(|| self.cancellation.clone()),
            retry: other.retry.or_else(|| self.retry.clone()),
        }
    }
}
pub(crate) fn earliest(a: Option<Instant>, b: Option<Instant>) -> Option<Instant> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

#[derive(Clone)]
pub(crate) struct RequestContext {
    pub(crate) deadline: Option<Instant>,
    tokens: Vec<CancellationToken>,
}
impl RequestContext {
    pub(crate) fn new(
        transport: &TransportOptions,
        options: &RequestOptions,
        tokens: &[CancellationToken],
    ) -> Result<Self, KalshiError> {
        let relative =
            options
                .timeout
                .or(transport.total_timeout)
                .map(|duration| {
                    Instant::now().checked_add(duration).ok_or(
                        KalshiError::InvalidTransportOptions("timeout is not representable"),
                    )
                })
                .transpose()?;
        let context = Self {
            deadline: earliest(relative, options.deadline),
            tokens: tokens.to_vec(),
        };
        context.check()?;
        options
            .retry
            .as_ref()
            .unwrap_or(&transport.retry)
            .validate()?;
        Ok(context)
    }
    pub(crate) fn check(&self) -> Result<(), KalshiError> {
        if self.tokens.iter().any(CancellationToken::is_cancelled) {
            return Err(KalshiError::Cancelled);
        }
        if self.deadline.is_some_and(|end| Instant::now() >= end) {
            return Err(KalshiError::DeadlineExceeded);
        }
        Ok(())
    }
    pub(crate) async fn run<T>(
        &self,
        future: impl Future<Output = Result<T, KalshiError>>,
    ) -> Result<T, KalshiError> {
        self.check()?;
        let cancelled = async {
            if self.tokens.is_empty() {
                std::future::pending::<()>().await;
            }
            let _ = futures_util::future::select_all(
                self.tokens.iter().map(|t| Box::pin(t.cancelled())),
            )
            .await;
        };
        let expired = async {
            match self.deadline {
                Some(end) => tokio::time::sleep_until(end).await,
                None => std::future::pending::<()>().await,
            }
        };
        let result = tokio::select! {
            biased;
            _ = cancelled => Err(KalshiError::Cancelled),
            _ = expired => Err(KalshiError::DeadlineExceeded),
            result = future => result,
        };
        self.check()?;
        result
    }
    pub(crate) fn fits(&self, delay: Duration) -> bool {
        Instant::now()
            .checked_add(delay)
            .is_some_and(|until| self.deadline.is_none_or(|end| until < end))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateBucket {
    Read,
    Write,
}
#[derive(Clone, Copy, Debug)]
pub struct OperationCost {
    pub bucket: RateBucket,
    pub units: NonZeroU32,
}
#[derive(Default, Debug)]
pub struct RateLimitOptions {
    pub read: Option<RateQuota>,
    pub write: Option<RateQuota>,
    pub costs: BTreeMap<String, OperationCost>,
}
pub struct LocalRateLimiter {
    read: Option<DefaultDirectRateLimiter>,
    write: Option<DefaultDirectRateLimiter>,
    costs: BTreeMap<String, OperationCost>,
}
impl std::fmt::Debug for LocalRateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalRateLimiter").finish_non_exhaustive()
    }
}
impl LocalRateLimiter {
    pub fn new(options: RateLimitOptions) -> Result<Arc<Self>, KalshiError> {
        for quota in [options.read, options.write].into_iter().flatten() {
            let interval = quota.replenish_interval().as_nanos();
            // Governor adds elapsed time to this span using u64 nanoseconds.
            // Retain counter headroom instead of accepting near-overflow quotas.
            if interval == 0
                || interval
                    .checked_mul(u128::from(quota.burst_size().get()))
                    .is_none_or(|burst| burst > u128::from(u64::MAX / 2))
                || Instant::now()
                    .checked_add(quota.replenish_interval())
                    .is_none()
            {
                return Err(KalshiError::InvalidTransportOptions(
                    "quota replenishment and burst timing must be positive and representable",
                ));
            }
        }
        for (id, cost) in &options.costs {
            operation_or_error(id)?;
            let quota = match cost.bucket {
                RateBucket::Read => options.read,
                RateBucket::Write => options.write,
            };
            if quota.is_none_or(|q| cost.units > q.burst_size()) {
                return Err(KalshiError::InvalidTransportOptions(
                    "operation cost requires a configured bucket with sufficient burst",
                ));
            }
        }
        Ok(Arc::new(Self {
            read: options.read.map(governor::RateLimiter::direct),
            write: options.write.map(governor::RateLimiter::direct),
            costs: options.costs,
        }))
    }
    pub(crate) async fn acquire(
        &self,
        operation_id: &str,
        method: &str,
        context: &RequestContext,
    ) -> Result<(), KalshiError> {
        let cost = self
            .costs
            .get(operation_id)
            .copied()
            .unwrap_or(OperationCost {
                bucket: if method == "GET" {
                    RateBucket::Read
                } else {
                    RateBucket::Write
                },
                units: NonZeroU32::MIN,
            });
        let limiter = match cost.bucket {
            RateBucket::Read => &self.read,
            RateBucket::Write => &self.write,
        };
        let Some(limiter) = limiter else {
            return Ok(());
        };
        loop {
            context.check()?;
            match limiter.check_n(cost.units) {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(until)) => {
                    tokio::time::sleep(until.wait_time_from(limiter.clock().now())).await
                }
                Err(_) => {
                    return Err(KalshiError::InvalidTransportOptions(
                        "operation cost exceeds burst",
                    ));
                }
            }
        }
    }
}

pub(crate) fn transient(error: &reqwest::Error) -> bool {
    if error.is_timeout() {
        return true;
    }
    // Response::chunk wraps network framing errors as "decode" errors too.
    // Only identified IO/framing sources below qualify; codec failures never enter here.
    if error.is_builder() || error.is_redirect() {
        return false;
    }
    let mut source = std::error::Error::source(error);
    while let Some(value) = source {
        if let Some(io) = value.downcast_ref::<std::io::Error>() {
            return matches!(
                io.kind(),
                std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::BrokenPipe
                    | std::io::ErrorKind::UnexpectedEof
                    | std::io::ErrorKind::TimedOut
            );
        }
        if let Some(http) = value.downcast_ref::<hyper::Error>()
            && http.is_incomplete_message()
        {
            return true;
        }
        source = value.source();
    }
    false
}
pub(crate) fn retryable(error: &KalshiError) -> bool {
    match error {
        KalshiError::Api(api) => matches!(api.status.as_u16(), 429 | 502 | 503 | 504),
        KalshiError::AttemptTimeout => true,
        KalshiError::Http(error) => transient(error),
        KalshiError::Response(error) if error.reason == ResponseFailureReason::Read => {
            error.read_error.as_ref().is_some_and(transient)
        }
        _ => false,
    }
}
pub(crate) fn attempts_for_method(method: &str, policy: &RetryPolicy) -> u32 {
    if method == "GET" {
        policy.max_attempts
    } else {
        1
    }
}
pub(crate) fn retry_delay(error: &KalshiError, backoff: Duration) -> Option<Duration> {
    let KalshiError::Api(api) = error else {
        return Some(backoff);
    };
    let Some(value) = api
        .headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
    else {
        return Some(backoff);
    };
    let value = value.trim();
    let delay = if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        Duration::from_secs(value.parse::<u64>().ok()?)
    } else {
        httpdate::parse_http_date(value)
            .ok()
            .map(|date| {
                date.duration_since(std::time::SystemTime::now())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    };
    Some(backoff.max(delay))
}

pub(crate) struct ResponseBytes {
    pub(crate) status: reqwest::StatusCode,
    pub(crate) headers: reqwest::header::HeaderMap,
    pub(crate) raw: Vec<u8>,
    pub(crate) decoded: Vec<u8>,
}
pub(crate) async fn consume(
    mut response: reqwest::Response,
    limit: usize,
    context: &RequestContext,
) -> Result<ResponseBytes, KalshiError> {
    let status = response.status();
    let headers = response.headers().clone();
    let mut raw = Vec::new();
    loop {
        context.check()?;
        let chunk = match response.chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break,
            Err(error) => {
                return Err(response_error(
                    status,
                    headers,
                    raw,
                    false,
                    ResponseFailureReason::Read,
                    Some(error.without_url()),
                ));
            }
        };
        if chunk.len() > limit - raw.len() {
            raw.extend_from_slice(&chunk[..limit - raw.len()]);
            return Err(response_error(
                status,
                headers,
                raw,
                false,
                ResponseFailureReason::EncodedSizeLimit,
                None,
            ));
        }
        raw.extend_from_slice(&chunk);
        tokio::task::yield_now().await;
    }
    let decoded = match decode(&raw, &headers, limit, context).await {
        Ok(bytes) => bytes,
        Err(DecodeFailure::Control(error)) => return Err(error),
        Err(DecodeFailure::Encoding(reason)) => {
            return Err(response_error(status, headers, raw, true, reason, None));
        }
    };
    context.check()?;
    Ok(ResponseBytes {
        status,
        headers,
        raw,
        decoded,
    })
}
fn response_error(
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    raw_body: Vec<u8>,
    raw_body_complete: bool,
    reason: ResponseFailureReason,
    read_error: Option<reqwest::Error>,
) -> KalshiError {
    KalshiError::Response(Box::new(KalshiResponseError {
        status,
        headers,
        raw_body,
        raw_body_complete,
        reason,
        read_error,
    }))
}
enum DecodeFailure {
    Control(KalshiError),
    Encoding(ResponseFailureReason),
}
async fn decode(
    raw: &[u8],
    headers: &reqwest::header::HeaderMap,
    limit: usize,
    context: &RequestContext,
) -> Result<Vec<u8>, DecodeFailure> {
    let mut encodings = Vec::new();
    for value in headers.get_all(reqwest::header::CONTENT_ENCODING) {
        let text = value
            .to_str()
            .map_err(|_| DecodeFailure::Encoding(ResponseFailureReason::MalformedEncoding))?;
        for name in text.split(',') {
            let name = name.trim().to_ascii_lowercase();
            if name.is_empty() {
                return Err(DecodeFailure::Encoding(
                    ResponseFailureReason::MalformedEncoding,
                ));
            }
            encodings.push(name);
            if encodings.len() > 4 {
                return Err(DecodeFailure::Encoding(
                    ResponseFailureReason::TooManyEncodingLayers,
                ));
            }
        }
    }
    let mut bytes = raw.to_vec();
    for encoding in encodings.iter().rev() {
        // Yield on compressed input as well as output: empty gzip members
        // otherwise let a decoder consume an entire body without yielding.
        let chunks = futures_util::stream::unfold(bytes.as_slice(), |input| async move {
            if input.is_empty() {
                return None;
            }
            tokio::task::yield_now().await;
            let count = input.len().min(8192);
            Some((
                Ok::<_, std::io::Error>(bytes::Bytes::copy_from_slice(&input[..count])),
                &input[count..],
            ))
        });
        let reader = BufReader::new(tokio_util::io::StreamReader::new(Box::pin(chunks)));
        macro_rules! decoder {
            ($kind:ident) => {{
                let mut decoder = $kind::new(reader);
                decoder.multiple_members(true);
                Box::new(decoder) as Box<dyn AsyncRead + Send + Unpin + '_>
            }};
        }
        let mut decoder = match encoding.as_str() {
            "identity" => continue,
            "gzip" => decoder!(GzipDecoder),
            "deflate" => decoder!(ZlibDecoder),
            "br" => decoder!(BrotliDecoder),
            "zstd" => decoder!(ZstdDecoder),
            _ => {
                return Err(DecodeFailure::Encoding(
                    ResponseFailureReason::UnsupportedEncoding,
                ));
            }
        };
        let mut output = Vec::new();
        let mut buffer = [0; 8192];
        loop {
            context.check().map_err(DecodeFailure::Control)?;
            let count = decoder
                .read(&mut buffer)
                .await
                .map_err(|_| DecodeFailure::Encoding(ResponseFailureReason::MalformedEncoding))?;
            if count == 0 {
                break;
            }
            if count > limit - output.len() {
                return Err(DecodeFailure::Encoding(
                    ResponseFailureReason::DecodedSizeLimit,
                ));
            }
            output.extend_from_slice(&buffer[..count]);
            tokio::task::yield_now().await;
        }
        drop(decoder);
        bytes = output;
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_jitter_is_bounded() {
        assert_eq!(
            jitter(Duration::from_secs(1), 0.5),
            Duration::from_millis(500)
        );
        assert_eq!(jitter(Duration::from_secs(1), 0.0), Duration::ZERO);
    }
    #[test]
    fn non_get_methods_including_patch_never_enable_replay() {
        let policy = RetryPolicy::safe_get(32);
        assert_eq!(attempts_for_method("GET", &policy), 32);
        for method in ["POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"] {
            assert_eq!(attempts_for_method(method, &policy), 1);
        }
    }
    #[test]
    fn governor_cost_accounting_uses_fake_clock_without_reserving_denied_units() {
        let clock = governor::clock::FakeRelativeClock::default();
        let limiter = governor::RateLimiter::direct_with_clock(
            RateQuota::with_period(Duration::from_secs(1))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
            clock.clone(),
        );
        assert!(
            limiter
                .check_n(NonZeroU32::new(2).unwrap())
                .unwrap()
                .is_ok()
        );
        assert!(limiter.check_n(NonZeroU32::MIN).unwrap().is_err());
        clock.advance(Duration::from_secs(1));
        assert!(limiter.check_n(NonZeroU32::MIN).unwrap().is_ok());
    }
    #[tokio::test(start_paused = true)]
    async fn asynchronous_decode_observes_cancellation_and_deadline_between_bounded_chunks() {
        use async_compression::tokio::bufread::GzipEncoder;
        let plain = vec![b' '; 4 * 1024 * 1024];
        let mut encoder = GzipEncoder::new(BufReader::new(plain.as_slice()));
        let mut encoded = Vec::new();
        encoder.read_to_end(&mut encoded).await.unwrap();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("content-encoding", "gzip".parse().unwrap());
        let mut empty_encoder = GzipEncoder::new(BufReader::new(&b""[..]));
        let mut empty_member = Vec::new();
        empty_encoder.read_to_end(&mut empty_member).await.unwrap();
        for encoded in [encoded, empty_member.repeat(16_384)] {
            for cancellation in [true, false] {
                let token = CancellationToken::new();
                let options = RequestOptions {
                    timeout: Some(Duration::from_secs(1)),
                    ..Default::default()
                };
                let context = RequestContext::new(
                    &TransportOptions::default(),
                    &options,
                    std::slice::from_ref(&token),
                )
                .unwrap();
                let cancel = tokio::spawn(async move {
                    if cancellation {
                        token.cancel();
                    } else {
                        tokio::time::advance(Duration::from_secs(2)).await;
                    }
                });
                let result = context
                    .run(async {
                        match decode(&encoded, &headers, 64 * 1024 * 1024, &context).await {
                            Ok(bytes) => Ok(bytes),
                            Err(DecodeFailure::Control(error)) => Err(error),
                            Err(DecodeFailure::Encoding(_)) => panic!("valid gzip failed"),
                        }
                    })
                    .await;
                if cancellation {
                    assert!(matches!(result, Err(KalshiError::Cancelled)));
                } else {
                    assert!(matches!(result, Err(KalshiError::DeadlineExceeded)));
                }
                cancel.await.unwrap();
            }
        }
    }
}
