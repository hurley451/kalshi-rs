# kalshi-rs / kalshi-client

[![CI](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml/badge.svg?branch=main)](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/hurley451/kalshi-rs/blob/main/LICENSE)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-edition%202024-black.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Status: prerelease](https://img.shields.io/badge/status-prerelease-yellow.svg)](https://github.com/hurley451/kalshi-rs/releases)
[![GitHub stars](https://img.shields.io/github/stars/hurley451/kalshi-rs.svg?style=social)](https://github.com/hurley451/kalshi-rs/stargazers)

A Rust SDK for the [Kalshi](https://kalshi.com) prediction market API, with async
REST requests through reqwest, RSA-PSS or Ed25519 authentication, and serde JSON handling.

This is a community client, not an official Kalshi SDK.
The repository is `kalshi-rs`, the Cargo package is `kalshi-client`,
and Rust imports use `kalshi_client`. The crate is not yet published.

## Contents

- [Features and coverage](#features)
- [Requirements and installation](#requirements)
- [Quick start](#quick-start)
- [Authentication](#authentication)
- [REST examples](#rest-examples)
- [WebSocket message parsing](#websocket-message-parsing)
- [Error handling](#error-handling)
- [Configuration](#configuration)
- [Operation lookup](#operation-lookup)
- [Running examples](#running-examples)
- [SDK family and support](#sdk-family-and-support)

## Features

- 118 named async REST methods with domain facades and typed serde models.
- Runtime validation of documented inputs, responses and error payloads.
- 28 lazy cursor paginators preserving complete page shapes and filters.
- Total request deadlines, cancellation, optional shared rate limiting, and bounded GET retries.
- Async HTTP requests with production and demo environment constants.
- RSA-PSS or Ed25519 signing with API-key headers and canonical signing helpers.
- Encoded path/query values and JSON request bodies.
- Portfolio fills query builders and serde response models.
- WebSocket envelope parsing and shared offline conformance fixtures.
- Request preparation for inspecting method, URL, headers, and body before sending.

Named methods use `kalshi_client::models` and validate the official wire schemas.
Generic requests remain an escape hatch returning `serde_json::Value`.
Undeclared response bodies remain unspecified; `UnspecifiedResponse` retains
status, headers, decoded content and exact `raw_body` bytes.

### API Coverage

| API family | Selected generated operation IDs | Support |
|------------|----------------------------------|---------|
| Exchange | `GetExchangeStatus`, `GetExchangeSchedule` | Typed REST |
| Account | `GetAccountApiLimits`, `GetAccountEndpointCosts` | Typed REST |
| Markets | `GetMarkets`, `GetMarket`, `GetMarketOrderbook`, `GetTrades` | Typed REST |
| Events and series | `GetEvents`, `GetEvent`, `GetSeriesList`, `GetSeries` | Typed REST |
| Discovery | `GetStructuredTargets`, `GetMilestones`, `GetMultivariateEventCollections` | Typed REST |
| V2 orders | `CreateOrderV2`, `CancelOrderV2`, `AmendOrderV2`, `BatchCreateOrdersV2` | Typed REST |
| Order groups | `GetOrderGroups`, `CreateOrderGroup`, `TriggerOrderGroup` | Typed REST |
| Portfolio | `GetBalance`, `GetPositions`, `GetSettlements`, `GetFills` | Typed REST; legacy fills helpers |
| Subaccounts | `GetSubaccountBalances`, `ApplySubaccountTransfer`, `GetSubaccountTransfers` | Typed REST |
| Historical | `GetHistoricalCutoff`, `GetHistoricalMarkets`, `GetFillsHistorical` | Typed REST |
| Incentives and allocation | `GetIncentivePrograms`, `GetTargetBalanceAllocation` | Typed REST |
| Live data | `GetEventLiveData`, `GetWeatherIndex`, `GetWeatherIndexCalibrations` | Typed REST |
| API keys | `GetApiKeys`, `CreateApiKey`, `DeleteApiKey` | Typed REST |
| Communications and FCM | `GetCommunicationsID`, `GetRFQs`, `GetQuotes`, `GetFCMOrders` | Typed REST |
| WebSocket | `parse_websocket_message` | Parsing only; no transport or subscriptions |

The predictions snapshot contains 118 operations. Named methods, schema tests and
wire tests cover that snapshot, not universal live compatibility. See [API coverage and gaps](https://github.com/hurley451/kalshi-rs/blob/main/docs/API_COVERAGE.md)
and [official API docs](https://docs.kalshi.com).

WebSocket connection management is not implemented in this version.
HTTP transport can be configured with `KalshiClient::builder` or
`KalshiClient::with_http_client_builder`.
Generic empty successes return JSON null; typed HTTP 204 operations return `()`.

## Requirements

- A current stable Rust toolchain supporting edition 2024 and the locked dependencies.
- A Tokio runtime for async HTTP execution.
- Kalshi API credentials for private endpoints, separately issued for demo/production.
- Node.js and a pinned KalshiCore checkout for contract regeneration.
- A KalshiCore checkout for offline conformance.

The package records its immutable Core source commit in
`package.metadata.kalshicore.ref`. See [contribution guidance](CONTRIBUTING.md)
for locked generation dependencies and staged reproducibility checks.

## Installation

Until the first crates.io release, add a Git dependency to your application:

```toml
[dependencies]
kalshi-client = { git = "https://github.com/hurley451/kalshi-rs", branch = "main" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

For reproducible builds, replace `branch` with `rev` set to a reviewed commit SHA.
After publication, install with `cargo add kalshi-client`.
A local checkout can also be referenced with `kalshi-client = { path = "/path/to/kalshi-rs" }`.

## Quick Start

Create a binary application with `cargo new kalshi-demo`, add the dependencies
above, and put this in `src/main.rs`. Run it with `cargo run`.

```rust,no_run
use kalshi_client::{DEMO, KalshiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KalshiClient::new(DEMO, None);
    let status = client.exchange().get_exchange_status().await?;
    println!("exchange active: {}", status.exchange_active);
    Ok(())
}
```

Named public methods do not require credentials. Generic public requests must
set `authenticated: false`; `KalshiRequest::default()` requests authentication.

## Authentication

Use environment variables for the API key ID and private-key file location:

```bash
export KALSHI_API_KEY_ID="your-demo-api-key-id"
export KALSHI_PRIVATE_KEY_FILE="/absolute/path/to/demo-private-key.pem"
```

```rust,no_run
use kalshi_client::{DEMO, KalshiAuth, KalshiClient};

fn authenticated_demo_client() -> Result<KalshiClient, Box<dyn std::error::Error>> {
    let auth = KalshiAuth {
        api_key_id: std::env::var("KALSHI_API_KEY_ID")?,
        private_key_pem: std::fs::read_to_string(
            std::env::var("KALSHI_PRIVATE_KEY_FILE")?
        )?,
    };
    Ok(KalshiClient::new(DEMO, Some(auth)))
}
```

The signer accepts RSA PKCS#1 or PKCS#8 PEM private keys and Ed25519 PKCS#8
PEM private keys. It selects the algorithm from the parsed key, not the PEM
header. Encrypted PEM input and other key types are not supported. Invalid keys
or header credentials fail with sanitized errors before dispatch.

RSA-PSS uses SHA-256 and signs the timestamp, uppercase method, and API path
without the query string. Keep system time accurate. Store keys outside the
repository and use credentials issued for the selected environment.
Ed25519 signs the same message directly, without prehashing. Signing failures
are returned as errors instead of panicking.

## REST Examples

These functions accept a client from the quick start or authentication examples.
They compile as README doctests but are not executed by the test suite.
Models and generic request fields use Kalshi's wire format.

### List Markets with Pagination

```rust,no_run
use kalshi_client::{KalshiClient, models::GetMarketsParameters};

async fn list_markets(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut pages = client.markets().get_markets_pages(GetMarketsParameters {
        limit: Some(100), ..Default::default()
    });
    while let Some(page) = pages.next_page().await? {
        for market in &page.markets {
            println!("{}: {}", market.ticker, market.title.as_deref().unwrap_or(""));
        }
    }
    Ok(())
}
```

Paginated methods expose `..._pages`. Each `next_page()` sends one request;
dropping the pager stops further requests. Empty/absent final cursors terminate,
repeated cursors fail, and filters stay unchanged. Pages retain every response
collection rather than flattening an arbitrary array.

### Get Market Details and an Order Book

```rust,no_run
use kalshi_client::{KalshiClient, models::{GetMarketParameters, GetMarketOrderbookParameters}};

async fn inspect_market(client: &KalshiClient, ticker: &str) -> Result<(), Box<dyn std::error::Error>> {
    let market = client.markets().get_market(&GetMarketParameters { ticker: ticker.into() }).await?;
    println!("{}", market.market.ticker);
    let book = client.markets().get_market_orderbook(&GetMarketOrderbookParameters {
        ticker: ticker.into(), depth: Some(10)
    }).await?;
    println!("{}", serde_json::to_string(&book.orderbook_fp)?);
    Ok(())
}
```

Choose a ticker returned by market discovery. The fixed-point order book uses
`yes_dollars` and `no_dollars` levels. Preserve price and quantity strings or use
an application-selected decimal type rather than floating-point arithmetic.

### Portfolio Fills and Pagination

```rust,no_run
use kalshi_client::{KalshiClient, models::GetFillsParameters};

async fn list_fills(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut pages = client.portfolio().get_fills_pages(GetFillsParameters {
        limit: std::num::NonZeroU64::new(100), ..Default::default()
    });
    while let Some(page) = pages.next_page().await? {
        for fill in &page.fills {
            println!("{} {}", fill.ticker, fill.count_fp.as_str());
        }
    }
    Ok(())
}
```

Pass an authenticated client. The generated query uses the official `ticker`
filter; the legacy `FillQuery` helper additionally supports a ticker list.
Filters include `order_id`, `subaccount`, `exchange_index`, `min_ts` and `max_ts`.
Fixed-point values preserve decimal strings. Deprecated `side` and `action`,
`created_time` and `ts` are optional. `cursor` is a required string; an empty
cursor marks the last page.

### Balance, Positions, and Historical Data

```rust,no_run
use kalshi_client::{KalshiClient, models::{GetBalanceParameters, GetPositionsParameters, GetHistoricalMarketsParameters}};

async fn portfolio_and_history(client: &KalshiClient) -> Result<(), Box<dyn std::error::Error>> {
    let balance = client.portfolio().get_balance(&GetBalanceParameters::default()).await?;
    let positions = client.portfolio().get_positions(&GetPositionsParameters::default()).await?;
    let cutoff = client.historical().get_historical_cutoff().await?;
    let markets = client.historical().get_historical_markets(&GetHistoricalMarketsParameters::default()).await?;
    println!("{}", serde_json::to_string(&balance)?);
    let _ = (positions, cutoff, markets);
    Ok(())
}
```

Private portfolio calls require authentication. Historical access is explicit;
the client does not reroute live requests.

### Events, Series, and Discovery

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, KalshiRequest};

async fn discovery(client: &KalshiClient) -> Result<(), KalshiError> {
    for operation in ["GetEvents", "GetSeriesList", "GetStructuredTargets", "GetMilestones"] {
        let mut request = KalshiRequest {
            authenticated: false,
            ..Default::default()
        };
        if operation == "GetMilestones" {
            request.query.push(("limit".into(), kalshi_client::QueryValue::from(100_u32)));
        }
        println!("{}", client.request(operation, request).await?);
    }
    let mut weather = KalshiRequest { authenticated: false, ..Default::default() };
    weather.path_parameters.insert("city".into(), "miami".into());
    println!("{}", client.request("GetWeatherIndex", weather).await?);
    Ok(())
}
```

### Prepare a Demo Order without Sending It

This example builds a request and does not submit it. The payload uses V2
fixed-point strings and auto-routing by ticker. Verify the current
[official OpenAPI](https://docs.kalshi.com/openapi.yaml) before executing mutations.

```rust,no_run
use kalshi_client::{DEMO, KalshiAuth, KalshiClient, KalshiError, KalshiRequest};

fn prepare_demo_order(auth: KalshiAuth, ticker: &str) -> Result<(), KalshiError> {
    let client = KalshiClient::new(DEMO, Some(auth));
    let request = KalshiRequest {
        body: Some(serde_json::json!({
            "ticker": ticker, "side": "bid", "count": "1.00", "price": "0.0100",
            "time_in_force": "good_till_canceled",
            "self_trade_prevention_type": "taker_at_cross",
            "exchange_index": -1
        })),
        ..Default::default()
    };
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis().to_string();
    let prepared = client.prepare_request("CreateOrderV2", request, &timestamp)?;
    println!("{} {}", prepared.method, prepared.url);
    // Do not log signed headers or private account payloads.
    Ok(())
}
```

To submit, use `client.request("CreateOrderV2", request).await` instead of
`prepare_request`. A successful JSON response contains `order_id`; cancellation
uses `CancelOrderV2` with path parameter `order_id` and queries `exchange_index`
and `market_ticker`. Do not assume cancelling reverses fills. Generic mutation
schemas are not validated locally. The read-only example runner below performs no mutations.

## WebSocket Message Parsing

Decode JSON before parsing the envelope:

```rust
use kalshi_client::parse_websocket_message;

let text = r#"{"type":"orderbook_delta","sid":1,"seq":2,"msg":{"market_ticker":"DEMO"}}"#;
let envelope = parse_websocket_message(serde_json::from_str(text)?)?;
assert_eq!(envelope.message_type, "orderbook_delta");
println!("{:?} {:?}", envelope.seq, envelope.msg);
# Ok::<(), Box<dyn std::error::Error>>(())
```

An application must supply transport and subscription management. The parser does
not connect, reconnect, or maintain order-book state. Channel-specific payloads
remain JSON unless you deserialize them into a model such as `OrderBookDeltaMessage`.

## Error Handling

Named methods return an operation-specific `TypedError`. Its `payload` contains
the declared error variant when decoding succeeds; `source` retains transport or
API context even when that payload is malformed or unspecified.

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, models::GetBalanceParameters};

async fn read_balance(client: &KalshiClient) {
    match client.portfolio().get_balance(&GetBalanceParameters::default()).await {
        Ok(balance) => println!("{}", serde_json::to_string(&balance).unwrap()),
        Err(error) => match &error.source {
            KalshiError::Api(api) => eprintln!("HTTP status: {}", api.status),
            _ => eprintln!("{error}"),
        },
    }
}
```

Generic request errors remain available for the escape hatch:

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, KalshiRequest};

async fn inspect_errors(client: &KalshiClient) {
    match client.request("GetBalance", KalshiRequest::default()).await {
        Ok(body) => println!("{body}"),
        Err(KalshiError::Http(error)) => {
            eprintln!("HTTP/transport failure, status: {:?}", error.status());
            if error.is_decode() { eprintln!("Response was not valid expected JSON."); }
        }
        Err(KalshiError::Api(error)) => {
            eprintln!("API status: {}", error.status);
            if error.status == 429 {
                eprintln!("Retry-After: {:?}", error.headers.get("retry-after"));
            }
        }
        Err(KalshiError::MissingAuth) => eprintln!("Configure authentication."),
        Err(KalshiError::InvalidSigningCredentials) => eprintln!("Check API key ID and private key format"),
        Err(KalshiError::SigningFailed) => eprintln!("Request signing failed"),
        Err(KalshiError::UnknownOperation(id)) => eprintln!("Unknown operation: {id}"),
        Err(error) => eprintln!("{error}"),
    }
}
```

`KalshiError` also distinguishes missing/unused path parameters, JSON decoding,
I/O, and helper validation errors. `KalshiError::Api` retains status, response
headers, the JSON or text error body, and exact encoded `raw_body` bytes.
`KalshiError::Response` retains bounded bytes and `raw_body_complete` when a body
cannot be completely read or decoded. Cancellation and deadline errors do not
claim a complete response. Debug formatting omits credentials and
account payloads; do not log those fields directly.
Retries are disabled by default. Configured retries apply only to GET requests;
POST, PUT, PATCH, DELETE and other methods are never replayed by the SDK.

## Configuration

| Surface | Default | Behavior |
|---------|---------|----------|
| `KalshiClient::new(environment, auth)` | Explicit arguments | Select environment and optional authentication |
| `KalshiClient::production(auth)` | `PRODUCTION` | Production with required credentials |
| `KalshiClient::builder(environment)` | Default transport | Configure authentication, HTTP builder, transport and shared limiter |
| `TransportOptions::total_timeout` | 30 seconds | Total budget across admission, attempts, body reads and backoff; `None` disables it |
| `TransportOptions::attempt_timeout` | None | Optional per-attempt budget |
| `TransportOptions::retry` | One attempt | Bounded GET-only attempts; mutations always have one attempt |
| `TransportOptions::max_response_bytes` | 64 MiB | Limit encoded bytes and each decoded content layer |
| `RequestOptions` | Inherit client settings | Override timeout/retry and add an absolute deadline or cancellation token |
| `LocalRateLimiter` | None | Shared, configurable read/write quotas and per-operation costs |
| `PRODUCTION` / `DEMO` | Constants | REST and WebSocket URLs |
| `KalshiRequest::authenticated` | `true` | False for unsigned public calls |
| Request `path_parameters` | Empty BTreeMap | Path-template values |
| Request `query` | Empty Vec | Ordered wire-name / QueryValue pairs |
| Request `body` | None | Optional serde JSON value |
| Request `headers` | Empty BTreeMap | Custom HTTP headers; signing supplies auth headers |
| `prepare_request` timestamp | Caller-supplied | Unix milliseconds for offline/request preparation |
| `request` timestamp | Current system time | Signing time supplied internally |

Custom environments use `KalshiEnvironment` with `name`, `rest_base_url`, and
`websocket_url`, each a static string. Customize the reqwest builder before the
SDK applies its guards. Already-built reqwest clients are not accepted. Redirects,
native HTTP retries and automatic content decoding remain disabled, including
when another dependency enables reqwest's compression features.

```rust,no_run
use std::{num::NonZeroU32, time::Duration};
use kalshi_client::{
    DEMO, KalshiAuth, KalshiClient, KalshiError, LocalRateLimiter,
    RateLimitOptions, RateQuota, RetryPolicy, TransportOptions,
};

fn configured_client(auth: KalshiAuth) -> Result<KalshiClient, KalshiError> {
    let limiter = LocalRateLimiter::new(RateLimitOptions {
        read: Some(RateQuota::per_second(NonZeroU32::new(10).unwrap())),
        write: Some(RateQuota::per_second(NonZeroU32::new(2).unwrap())),
        ..Default::default()
    })?;
    KalshiClient::builder(DEMO)
        .auth(auth)
        .http_builder(|http| http.connect_timeout(Duration::from_secs(5)))
        .transport_options(TransportOptions {
            total_timeout: Some(Duration::from_secs(20)),
            retry: RetryPolicy::safe_get(3),
            ..Default::default()
        })
        .rate_limiter(limiter)
        .build()
}
```

The example quotas are local application settings, not exchange account limits.
Configure `RateLimitOptions::costs` when an endpoint costs more than the default
one token. Client clones share the same limiter.

```rust,no_run
use std::time::Duration;
use kalshi_client::{CancellationToken, KalshiClient, RequestOptions};

fn request_scope(client: &KalshiClient, cancellation: CancellationToken) -> KalshiClient {
    client.with_request_options(RequestOptions {
        timeout: Some(Duration::from_secs(5)),
        cancellation: Some(cancellation),
        ..Default::default()
    })
}
```

Use the scoped client's ordinary typed methods. A retained cancellation-token
clone cancels its requests without cancelling unrelated client calls. Inherited
cancellation tokens and earlier absolute deadlines cannot be removed by a child
scope. Pagers provide `with_deadline`, `with_cancellation` and
`next_page_with_options` for traversal-wide and per-page controls.

Enabled GET retries cover transient transport failures and HTTP 429, 502, 503
and 504. They respect valid `Retry-After` values and stop when the total budget
cannot accommodate the wait. Each physical attempt is signed and admitted again.
Unsupported or malformed content encoding is not retried. Gzip, zlib deflate,
Brotli and Zstandard decoding is bounded while original encoded bytes and
response headers are retained.

Cancellation or timeout does not prove that a submitted mutation was rejected
by the exchange. Reconcile its state rather than automatically replaying it.

## Operation Lookup

```rust
use kalshi_client::{OPERATIONS, get_operation};

assert_eq!(get_operation("GetMarket").unwrap().path, "/markets/{ticker}");
for operation in OPERATIONS.iter().filter(|op| op.path.starts_with("/portfolio")) {
    println!("{} {} {}", operation.operation_id, operation.method, operation.path);
}
```

See [contributing](CONTRIBUTING.md) for development and test commands.

## Project Structure

```text
src/
  auth.rs             RSA-PSS and Ed25519 signing
  client.rs           Request preparation and async HTTP execution
  environment.rs      Production/demo constants
  error.rs            SDK error enum
  generated/          Operation definitions
  operation.rs        Path and query handling
  portfolio.rs        Fills models, filters, and parsing
  websocket.rs        Envelope and delta models
  bin/                Offline conformance executable
examples/read_only.rs Read-only demo examples
tests/                SDK tests
tools/                Operation generator
docs/                 API documentation
```

## Running Examples

From a repository checkout:

```bash
cargo run --example read_only -- status
cargo run --example read_only -- markets
cargo run --example read_only -- orderbook YOUR-DEMO-MARKET-TICKER
# Requires demo API key ID and supported PEM private-key file environment variables:
cargo run --example read_only -- fills
```

Every scenario uses demo and performs read-only requests. Public scenarios need
no credentials. See [example source](https://github.com/hurley451/kalshi-rs/blob/main/examples/read_only.rs). The example's Tokio
runtime is a development dependency; SDK consumers choose their own runtime setup.

## SDK Family and Support

| Project | Role |
|---------|------|
| [KalshiCore](https://github.com/hurley451/KalshiCore) | Shared contracts, fixtures, and conformance framework |
| [KalshiSharp](https://github.com/hurley451/KalshiSharp) | C# / .NET SDK |
| [KalshiTS](https://github.com/hurley451/KalshiTS) | TypeScript / JavaScript SDK for Node.js |
| [kalshi-rs](https://github.com/hurley451/kalshi-rs) | Rust SDK, package name `kalshi-client` |

Read [contributing](https://github.com/hurley451/kalshi-rs/blob/main/CONTRIBUTING.md), ask a
[usage question](https://github.com/hurley451/kalshi-rs/issues/new?template=question.yml),
or open a [discussion](https://github.com/hurley451/kalshi-rs/discussions).
Report SDK behavior here and shared contract/fixture gaps in KalshiCore.

## License

[MIT](https://github.com/hurley451/kalshi-rs/blob/main/LICENSE).
