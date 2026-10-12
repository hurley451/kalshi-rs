# kalshi-rs / kalshi-client

[![CI](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml/badge.svg?branch=main)](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/hurley451/kalshi-rs/blob/main/LICENSE)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-edition%202024-black.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Status: prerelease](https://img.shields.io/badge/status-prerelease-yellow.svg)](https://github.com/hurley451/kalshi-rs/releases)
[![GitHub stars](https://img.shields.io/github/stars/hurley451/kalshi-rs.svg?style=social)](https://github.com/hurley451/kalshi-rs/stargazers)

A Rust SDK for the [Kalshi](https://kalshi.com) prediction market API, with async
REST requests through reqwest, RSA-PSS authentication, and serde JSON handling.

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

- Generated REST operation registry with IDs, methods, paths, summaries, and tags.
- Async HTTP requests with production and demo environment constants.
- RSA-PSS signing with API-key headers and canonical signing helpers.
- Encoded path/query values and JSON request bodies.
- Portfolio fills query builders and serde response models.
- WebSocket envelope parsing and shared offline conformance fixtures.
- Request preparation for inspecting method, URL, headers, and body before sending.

Generic requests return `serde_json::Value`. Generated operations do not include
parameter, request-body, or response schemas. Typed fills parsing is provided by
free functions, not a `client.portfolio` facade.

### API Coverage

| API family | Selected generated operation IDs | Support |
|------------|----------------------------------|---------|
| Exchange | `GetExchangeStatus`, `GetExchangeSchedule` | Generic REST |
| Account | `GetAccountApiLimits`, `GetAccountEndpointCosts` | Generic REST |
| Markets | `GetMarkets`, `GetMarket`, `GetMarketOrderbook`, `GetTrades` | Generic REST |
| Events and series | `GetEvents`, `GetEvent`, `GetSeriesList`, `GetSeries` | Generic REST |
| Discovery | `GetStructuredTargets`, `GetMilestones`, `GetMultivariateEventCollections` | Generic REST |
| V2 orders | `CreateOrderV2`, `CancelOrderV2`, `AmendOrderV2`, `BatchCreateOrdersV2` | Generic REST |
| Order groups | `GetOrderGroups`, `CreateOrderGroup`, `TriggerOrderGroup` | Generic REST |
| Portfolio | `GetBalance`, `GetPositions`, `GetSettlements`, `GetFills` | Generic REST; fills query/parser helpers |
| Subaccounts | `GetSubaccountBalances`, `ApplySubaccountTransfer`, `GetSubaccountTransfers` | Generic REST |
| Historical | `GetHistoricalCutoff`, `GetHistoricalMarkets`, `GetFillsHistorical` | Generic REST |
| Incentives and allocation | `GetIncentivePrograms`, `GetTargetBalanceAllocation` | Generic REST |
| Live data | `GetEventLiveData`, `GetWeatherIndex`, `GetWeatherIndexCalibrations` | Generic REST |
| API keys | `GetApiKeys`, `CreateApiKey`, `DeleteApiKey` | Generic REST |
| Communications | `GetCommunicationsID`, `GetRFQs`, `GetQuotes` | Generic REST |
| WebSocket | `parse_websocket_message` | Parsing only; no transport or subscriptions |

The checked-in inventory contains 118 operations. Inventory access is broader
than typed models and contract-tested coverage. See [API coverage and gaps](https://github.com/hurley451/kalshi-rs/blob/main/docs/API_COVERAGE.md)
and [official API docs](https://docs.kalshi.com).

Automatic retries, rate limiting and WebSocket connection management are not
implemented. HTTP transport can be configured with `KalshiClient::with_http_client`.
Empty successful responses return JSON null.

## Requirements

- A current stable Rust toolchain supporting edition 2024 and the locked dependencies.
- A Tokio runtime for async HTTP execution.
- Kalshi API credentials for private endpoints, separately issued for demo/production.
- Node.js and a KalshiCore checkout only for operation regeneration.
- A KalshiCore checkout for offline conformance.

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
use kalshi_client::{DEMO, KalshiClient, KalshiRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KalshiClient::new(DEMO, None);
    let status = client.request("GetExchangeStatus", KalshiRequest {
        authenticated: false,
        ..Default::default()
    }).await?;
    println!("{status}");
    Ok(())
}
```

Public calls without credentials must set `authenticated: false`.
`KalshiRequest::default()` requests authentication.

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

The signer accepts a **PKCS#8 RSA PEM private key**, typically beginning with
`-----BEGIN PRIVATE KEY-----`. PKCS#1 (`BEGIN RSA PRIVATE KEY`) and Ed25519
keys are not accepted by the current signer.

RSA-PSS uses SHA-256 and signs the timestamp, uppercase method, and API path
without the query string. Keep system time accurate. Store keys outside the
repository and use credentials issued for the selected environment.

## REST Examples

These functions accept a client from the quick start or authentication examples.
They compile as README doctests but are not executed by the test suite.
Generic query/body field names use Kalshi's wire format.

### List Markets with Pagination

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, KalshiRequest, QueryValue};

async fn list_markets(client: &KalshiClient) -> Result<(), KalshiError> {
    let mut cursor: Option<String> = None;
    loop {
        let mut request = KalshiRequest {
            authenticated: false,
            query: vec![
                ("limit".into(), QueryValue::from(100_u32)),
                ("status".into(), QueryValue::from("open")),
            ],
            ..Default::default()
        };
        if let Some(value) = cursor.take() {
            request.query.push(("cursor".into(), QueryValue::from(value)));
        }
        let page = client.request("GetMarkets", request).await?;
        let markets = page["markets"].as_array()
            .ok_or_else(|| KalshiError::Fixture("Missing markets array".into()))?;
        for market in markets {
            println!("{}: {}", market["ticker"], market["title"]);
        }
        cursor = page["cursor"].as_str().filter(|s| !s.is_empty()).map(str::to_owned);
        if cursor.is_none() { break; }
    }
    Ok(())
}
```

There is no automatic paginator. Preserve filters when continuing a cursor.
The response is JSON; add application-specific serde models when needed.

### Get Market Details and an Order Book

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, KalshiRequest, QueryValue};

async fn inspect_market(client: &KalshiClient, ticker: &str) -> Result<(), KalshiError> {
    let mut request = KalshiRequest {
        authenticated: false,
        ..Default::default()
    };
    request.path_parameters.insert("ticker".into(), ticker.into());
    println!("{}", client.request("GetMarket", request.clone()).await?);
    request.query.push(("depth".into(), QueryValue::from(10_u32)));
    let book = client.request("GetMarketOrderbook", request).await?;
    println!("{}", book["orderbook_fp"]);
    Ok(())
}
```

Choose a ticker returned by market discovery. The fixed-point order book uses
`yes_dollars` and `no_dollars` levels. Preserve price and quantity strings or use
an application-selected decimal type rather than floating-point arithmetic.

### Portfolio Fills and Pagination

```rust,no_run
use kalshi_client::{
    FillQuery, KalshiClient, KalshiError, KalshiRequest,
    parse_fills_page, portfolio_fills_query,
};

async fn list_fills(client: &KalshiClient) -> Result<(), KalshiError> {
    let mut query = FillQuery {
        tickers: vec!["YOUR-MARKET-A".into(), "YOUR-MARKET-B".into()],
        limit: Some(100),
        ..Default::default()
    };
    loop {
        let body = client.request("GetFills", KalshiRequest {
            query: portfolio_fills_query(&query)?,
            ..Default::default()
        }).await?;
        let page = parse_fills_page(body)?;
        for fill in page.fills {
            println!("{} {:?} {:?}", fill.ticker, fill.count_fp, fill.yes_price_dollars);
        }
        query.cursor = (!page.cursor.is_empty()).then_some(page.cursor);
        if query.cursor.is_none() { break; }
    }
    Ok(())
}
```

Pass an authenticated client. Use either `ticker: Some(...)` or a populated
`tickers` list, not both. Lists accept at most 100 entries and reject blanks.
Other filters are `order_id`, `subaccount`, `exchange_index`, `min_ts`, and `max_ts`.
The helper emits a comma-separated `ticker` query parameter. Optional
fixed-point fields stay strings in the serde model. Deprecated `side` and `action`,
`created_time` and `ts` are optional. `cursor` is a required string; an empty
cursor marks the last page.

### Balance, Positions, and Historical Data

```rust,no_run
use kalshi_client::{KalshiClient, KalshiError, KalshiRequest, QueryValue};

async fn portfolio_and_history(client: &KalshiClient) -> Result<(), KalshiError> {
    println!("{}", client.request("GetBalance", KalshiRequest::default()).await?);
    println!("{}", client.request("GetPositions", KalshiRequest {
        query: vec![("limit".into(), QueryValue::from(100_u32))],
        ..Default::default()
    }).await?);
    for operation in ["GetHistoricalCutoff", "GetHistoricalMarkets"] {
        println!("{}", client.request(operation, KalshiRequest {
            authenticated: false,
            ..Default::default()
        }).await?);
    }
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
        Err(KalshiError::RsaKey(error)) => eprintln!("Check PKCS#8 RSA PEM: {error}"),
        Err(KalshiError::UnknownOperation(id)) => eprintln!("Unknown operation: {id}"),
        Err(error) => eprintln!("{error}"),
    }
}
```

`KalshiError` also distinguishes missing/unused path parameters, JSON decoding,
I/O, and helper validation errors. `KalshiError::Api` retains status, response
headers and the JSON or text error body. Debug formatting omits credentials and
account payloads; do not log those fields directly.
Retry policy belongs to the application. Never blindly replay a failed mutation.

## Configuration

| Surface | Default | Behavior |
|---------|---------|----------|
| `KalshiClient::new(environment, auth)` | Explicit arguments | Select environment and optional authentication |
| `KalshiClient::production(auth)` | `PRODUCTION` | Production with required credentials |
| `PRODUCTION` / `DEMO` | Constants | REST and WebSocket URLs |
| `KalshiRequest::authenticated` | `true` | False for unsigned public calls |
| Request `path_parameters` | Empty BTreeMap | Path-template values |
| Request `query` | Empty Vec | Ordered wire-name / QueryValue pairs |
| Request `body` | None | Optional serde JSON value |
| Request `headers` | Empty BTreeMap | Custom HTTP headers; signing supplies auth headers |
| `prepare_request` timestamp | Caller-supplied | Unix milliseconds for offline/request preparation |
| `request` timestamp | Current system time | Signing time supplied internally |

Custom environments use `KalshiEnvironment` with `name`, `rest_base_url`, and
`websocket_url`, each a static string. Use a configured reqwest client with
`KalshiClient::with_http_client` to set transport timeouts. A caller can bound
an async wait using its runtime, but dropping a request does not prove that a
mutation was never accepted.

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
  auth.rs             RSA-PSS signing
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
# Requires demo API key ID and PKCS#8 private-key file environment variables:
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
