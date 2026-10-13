# kalshi-rs API Coverage

The predictions OpenAPI snapshot contains 118 REST operations. Each has a named
async method, typed serde input/response/error models and runtime validation.
Snapshot verification is not a claim of live compatibility with later changes.

## Support Levels

| Surface | Current support |
|---------|-----------------|
| REST methods | 118 named async methods and domain facades |
| Models | Complete declared inputs, responses/errors and source optionality |
| Pagination | 28 lazy page iterators preserving filters and full page shapes |
| Transport | Guarded HTTP builder, total/attempt budgets, cancellation and byte limits |
| Resilience | Optional shared read/write admission and bounded GET-only retries |
| Authentication | RSA-PSS and Ed25519 signing, API-key headers, canonical signing paths |
| Environments | Production and demo |
| Portfolio fills | Query helper and response parsing |
| WebSocket | Envelope parsing and offline replay; no connection/subscription client |
| Conformance | Four registered offline fixtures |

Named models live in `kalshi_client::models`. Fixed-point values preserve strings;
nullable and optional fields retain their distinct presence. Generic requests
remain an escape hatch returning `serde_json::Value`. Undeclared successful
bodies retain status, headers, decoded content and exact raw bytes.
Async HTTP uses reqwest and requires a Tokio runtime.
Redirects and reqwest's native retry/decompression policies are disabled after
caller customization. The SDK retains encoded response bytes separately from
decoded payloads and bounds both. Request controls cover queueing, attempts,
body consumption and retry backoff; pagers also accept traversal controls.

The [endpoint matrix](../tools/typed-coverage.json) records parameters, request
bodies, responses, named methods and pagination. The
[model manifest](../tools/typed-model-manifest.json) records schema/model mapping.
Tests cover every method through actual HTTP, positive/negative schema fixtures,
recursive field presence, fixed-point precision, signature verification and
external Cargo consumers.

## Validated Contracts

The target manifest registers these fixtures:

- `auth.signing.rest-get-portfolio-fills`
- `rest.request.portfolio-fills-query`
- `rest.response.portfolio-fills-page`
- `websocket.replay.orderbook-delta`

These four shared fixtures supplement endpoint/model tests. See the
[parent framework](https://github.com/hurley451/KalshiCore) for fixture contents.

## Remaining Gaps

- Separate perps, clearing-member and manual CF REST coverage.
- WebSocket transport, subscriptions, reconnect, and order-book state management.
- Credentialed integration tests and independent live API validation.
- Public registry release and published-package consumer verification.

Track SDK implementation gaps in this repository. File shared contract or fixture
gaps in KalshiCore with evidence and a link to the affected SDK issue.
