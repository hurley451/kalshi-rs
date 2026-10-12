# Contributing to kalshi-rs

Focused fixes, API coverage, examples, and documentation improvements are welcome.
Open an issue before a substantial change and describe the endpoint or behavior.

## Development

Use a checkout of [KalshiCore](https://github.com/hurley451/KalshiCore) at
`da235871e8c55ca1f6bcc2020f5324e909f1998a` for reproducible shared contracts
and offline fixtures. Normal CI uses this commit; scheduled CI checks `main`
for contract drift.

```bash
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --all -- --check
cargo run --locked --bin kalshicore-conformance -- --kalshicore-root /path/to/KalshiCore --report artifacts/kalshicore-conformance/report.json
```

## Generated Typed Outputs

Install generation dependencies from their lockfiles and run the checks:

```bash
export KALSHICORE_ROOT=/path/to/KalshiCore
npm ci --prefix tools/probe-fixtures
cargo fetch --locked --manifest-path tools/typed-generator/Cargo.toml
node tools/check_generated_typed.mjs
node --test tools/check_generated_typed.test.mjs
```

The checker generates all 14 typed Rust, schema, coverage, manifest, and fixture
outputs in a fresh temporary directory and compares every output byte-for-byte
with its original. Missing or differing originals fail without being repaired.
The tests run independent generations and check missing and tampered copies of
each output, plus full-check failure preservation. Neither checker writes the
original outputs. Rustfmt formats only the eight generated Rust files, with
module traversal disabled; handwritten runtime and support modules are untouched.

To prepare an intentional contract update for review, generate into a staging
directory and then generate fixtures using that directory as the working directory:

```bash
staging="$(mktemp -d)"
repo="$PWD"
node tools/generate_typed.mjs "$KALSHICORE_ROOT/contracts/current-official.json" --out-dir "$staging"
(cd "$staging" && node "$repo/tools/probe-fixtures/generate.mjs")
```

An explicit bundle argument can also be passed to the checker. Typed generation
requires that argument or `KALSHICORE_ROOT`; it has no machine-specific fallback.
Operation inventory drift is checked independently in CI using a staged
`generate_operations.mjs --out` output and `cmp`, without replacing the original.

Include the commands and results in your pull request. Keep credentials out of
source, examples, logs, and issue reports.

## SDK and Framework Boundaries

Rust implementation, packaging, runtime behavior, and user documentation
belong here. Shared API contracts and fixtures belong in KalshiCore. Link official
Kalshi documentation when changing API behavior, and include a focused regression
test when behavior changes.

Use [questions](https://github.com/hurley451/kalshi-rs/issues/new?template=question.yml)
or [discussions](https://github.com/hurley451/kalshi-rs/discussions) for usage help.
