# kalshi-rs / kalshi-client

[![CI](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml/badge.svg)](https://github.com/hurley451/kalshi-rs/actions/workflows/kalshicore-conformance.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust Edition 2024](https://img.shields.io/badge/Rust-edition%202024-black.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![Status: prerelease](https://img.shields.io/badge/status-prerelease-yellow.svg)](https://github.com/hurley451/kalshi-rs/releases)
[![GitHub stars](https://img.shields.io/github/stars/hurley451/kalshi-rs.svg?style=social)](https://github.com/hurley451/kalshi-rs/stargazers)

A Rust SDK for the [Kalshi](https://kalshi.com) prediction market API. The Cargo
package is `kalshi-client`; Rust imports use `kalshi_client`.

Built from [KalshiCore](https://github.com/hurley451/KalshiCore) contracts and
conformance fixtures, with async REST requests through reqwest, RSA-PSS signing,
and serde JSON handling. This is a community SDK, not an official Kalshi SDK.

## Installation

The package has not yet been published to crates.io. Until the first release,
use the repository dependency:

```toml
[dependencies]
kalshi-client = { git = "https://github.com/hurley451/kalshi-rs", branch = "main" }
```

For reproducible builds, replace `branch` with `rev` set to a reviewed commit SHA.
After the first crates.io release, install with `cargo add kalshi-client`.

## Quick Start

Call public endpoints without credentials from an async function running on a
Tokio runtime:

```rust,no_run
use kalshi_client::{DEMO, KalshiClient, KalshiError, KalshiRequest};

async fn exchange_status() -> Result<(), KalshiError> {
    let client = KalshiClient::new(DEMO, None);
    let request = KalshiRequest {
        authenticated: false,
        ..Default::default()
    };
    let status = client.request("GetExchangeStatus", request).await?;
    println!("{status}");
    Ok(())
}
```

For private endpoints, construct `KalshiAuth` with your API key ID and RSA private
key in PEM format, then pass `Some(auth)` to `KalshiClient::new`. Keep private keys
outside source control. Requests are authenticated by default.

## API Coverage

- Generated inventory of 118 REST operations in the KalshiCore OpenAPI snapshot.
- Generic operation-ID requests with path parameters, queries, and JSON bodies.
- Production and demo environments, async reqwest HTTP, and RSA-PSS signing.
- Portfolio fills query helpers and serde response parsing.
- WebSocket envelope parsing and offline replay fixtures.

Operation inventory coverage does not mean every request or response has a typed
model or a dedicated compatibility test. WebSocket connection management, retries,
and rate limiting are not implemented. See [API coverage](docs/API_COVERAGE.md).

## SDK Family

| Project | Role |
|---------|------|
| [KalshiCore](https://github.com/hurley451/KalshiCore) | Shared contracts, fixtures, and conformance framework |
| [KalshiSharp](https://github.com/hurley451/KalshiSharp) | C# / .NET SDK |
| [KalshiTS](https://github.com/hurley451/KalshiTS) | TypeScript / JavaScript SDK for Node.js |
| [kalshi-rs](https://github.com/hurley451/kalshi-rs) | Rust SDK, package name `kalshi-client` |

## Development and Support

```bash
node tools/generate_operations.mjs --kalshicore-root /path/to/KalshiCore
cargo test
cargo run --bin kalshicore-conformance -- --kalshicore-root /path/to/KalshiCore --report artifacts/kalshicore-conformance/report.json
```

Read [contributing](CONTRIBUTING.md), ask a [usage question](https://github.com/hurley451/kalshi-rs/issues/new?template=question.yml),
or open a [discussion](https://github.com/hurley451/kalshi-rs/discussions).
See [visibility tracking](docs/VISIBILITY.md) for discovery metrics.

Scheduled parent issue filing requires a `KALSHICORE_ISSUE_TOKEN` repository secret
with issue access to KalshiCore. Official API documentation: [docs.kalshi.com](https://docs.kalshi.com).

## License

[MIT](LICENSE).
