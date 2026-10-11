# kalshi-rs

Rust SDK target for [KalshiCore](https://github.com/hurley451/KalshiCore)
conformance.

This crate exposes the official REST operation inventory tracked by KalshiCore,
request signing helpers, request preparation, fills parsing, websocket envelope
parsing, and an offline KalshiCore conformance adapter.

## Local Verification

```bash
node tools/generate_operations.mjs --kalshicore-root /path/to/KalshiCore
cargo test
cargo run --bin kalshicore-conformance -- --kalshicore-root /path/to/KalshiCore --report artifacts/kalshicore-conformance/report.json
```
