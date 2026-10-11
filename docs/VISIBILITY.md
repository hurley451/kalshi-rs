# kalshi-rs Visibility Tracking

## Baseline

Captured from the GitHub repository API on 2026-10-11.

| Metric | Value |
|--------|-------|
| GitHub stars | 0 |
| Forks | 0 |
| Watchers/subscribers | 0 |
| Registry package | `kalshi-client`, not yet published |

Traffic, contributor counts, external references, and package downloads were not
captured in this baseline. Do not treat missing measurements as zero.

## Monthly Checks

Run from this repository with authenticated GitHub access:

```bash
gh repo view --json nameWithOwner,description,homepageUrl,repositoryTopics,stargazerCount,forkCount
gh api repos/hurley451/kalshi-rs --jq .subscribers_count
gh api repos/hurley451/kalshi-rs/contributors --paginate --jq '.[].login'
gh api repos/hurley451/kalshi-rs/traffic/views
gh api repos/hurley451/kalshi-rs/traffic/clones
gh api repos/hurley451/kalshi-rs/traffic/popular/referrers
gh api repos/hurley451/kalshi-rs/traffic/popular/paths
gh api 'search/code?q=%22kalshi-client%22+-repo:hurley451/kalshi-rs'
gh api 'search/issues?q=%22kalshi-client%22+-repo:hurley451/kalshi-rs'
cargo info kalshi-client
```

GitHub traffic provides a rolling 14-day window and requires appropriate repository
access. Save dated snapshots outside committed source when tracking longer trends.
Search results are directional: private usage and unindexed references are absent,
and bot/catalog mirrors can inflate apparent adoption.

## Discovery Objectives

- Keep About metadata and topics specific to Kalshi, prediction markets, and the language.
- Keep installation, examples, coverage boundaries, and support easy to find.
- Cross-link KalshiCore and sibling SDKs so users can choose their language.
- Use small working examples and specific issue titles to attract useful references.
- Measure external usage separately from catalog mirrors and automated clones.

## Ecosystem Details

Cargo metadata includes Kalshi, trading, prediction-market keywords and API-binding,
asynchronous, and HTTP-client categories. The repository is `kalshi-rs`; the
package is `kalshi-client`, and Rust imports use `kalshi_client`.
After publication, add crates.io version/download and docs.rs badges.
Keep the README and rustdoc examples aligned.
