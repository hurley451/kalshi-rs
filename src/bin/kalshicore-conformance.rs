use std::fs;
use std::path::{Path, PathBuf};

use kalshi_rs::{
    FillQuery, build_portfolio_fills_path, build_signing_preimage, parse_fills_page,
    parse_websocket_message,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
struct CheckResult {
    id: String,
    passed: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct Report {
    #[serde(rename = "targetId")]
    target_id: String,
    #[serde(rename = "kalshiCoreRoot")]
    kalshi_core_root: String,
    #[serde(rename = "generatedAtUtc")]
    generated_at_utc: String,
    checks: Vec<CheckResult>,
}

fn main() {
    let kalshi_core_root = option_value("--kalshicore-root")
        .or_else(|| std::env::var("KALSHICORE_ROOT").ok())
        .expect("Set KALSHICORE_ROOT or pass --kalshicore-root <path>.");
    let kalshi_core_root = PathBuf::from(kalshi_core_root);
    let report_path = PathBuf::from(
        option_value("--report")
            .unwrap_or_else(|| "artifacts/kalshicore-conformance/report.json".to_string()),
    );

    let manifest =
        read_json(&kalshi_core_root.join("targets/rust/kalshi-rs.json")).unwrap_or_else(|_| {
            read_json(&kalshi_core_root.join("targets/typescript/KalshiTS.json")).unwrap()
        });
    let target_id = manifest
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("rust.kalshi-rs")
        .to_string();
    let fixture_ids = manifest["conformance"]["fixtureIds"]
        .as_array()
        .expect("target manifest must contain conformance.fixtureIds")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("fixture id must be a string")
                .to_string()
        })
        .collect::<Vec<_>>();

    let checks = fixture_ids
        .iter()
        .map(|fixture_id| run_fixture(&kalshi_core_root, fixture_id))
        .collect::<Vec<_>>();
    let report = Report {
        target_id,
        kalshi_core_root: kalshi_core_root.display().to_string(),
        generated_at_utc: timestamp_utc(),
        checks,
    };

    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).expect("report directory can be created");
    }
    fs::write(
        &report_path,
        format!("{}\n", serde_json::to_string_pretty(&report).unwrap()),
    )
    .expect("report can be written");

    let mut failed = false;
    for check in &report.checks {
        println!(
            "{} {}: {}",
            if check.passed { "PASS" } else { "FAIL" },
            check.id,
            check.message
        );
        failed |= !check.passed;
    }
    println!("Report: {}", report_path.display());
    if failed {
        std::process::exit(1);
    }
}

fn run_fixture(root: &Path, fixture_id: &str) -> CheckResult {
    match read_fixture(root, fixture_id).and_then(|fixture| match fixture["kind"].as_str() {
        Some("auth-signing") => run_auth_signing(fixture_id, &fixture),
        Some("rest-request-serialization") => run_rest_request(fixture_id, &fixture),
        Some("rest-response-shape") => run_rest_response(fixture_id, &fixture),
        Some("websocket-replay") => run_websocket_replay(fixture_id, &fixture),
        Some(kind) => Err(format!("Unsupported fixture kind {kind}.")),
        None => Err("Fixture kind must be a string.".to_string()),
    }) {
        Ok(message) => pass(fixture_id, &message),
        Err(message) => fail(fixture_id, &message),
    }
}

fn run_auth_signing(_fixture_id: &str, fixture: &Value) -> Result<String, String> {
    let input = &fixture["input"];
    let expected = &fixture["expected"];
    let message = build_signing_preimage(
        string(input, "timestampMs")?,
        string(input, "method")?,
        string(input, "path")?,
    );
    expect_eq(
        string(expected, "signatureMessage")?,
        &message,
        "signature message",
    )?;
    let canonical = string(input, "path")?.split('?').next().unwrap_or_default();
    expect_eq(
        string(expected, "canonicalPath")?,
        canonical,
        "canonical path",
    )?;
    Ok("Signing canonical message matched.".to_string())
}

fn run_rest_request(_fixture_id: &str, fixture: &Value) -> Result<String, String> {
    let input = &fixture["input"];
    let query = &input["query"];
    let expected = &fixture["expected"];
    let fills_query = FillQuery {
        tickers: string_array(query, "tickers")?,
        limit: optional_u32(query, "limit")?,
        cursor: optional_string(query, "cursor"),
        ..FillQuery::default()
    };
    let path = build_portfolio_fills_path(&fills_query).map_err(|error| error.to_string())?;
    expect_eq(
        string(expected, "method")?,
        string(input, "method")?,
        "HTTP method",
    )?;
    expect_eq(string(expected, "pathAndQuery")?, &path, "path and query")?;
    Ok("Portfolio fills request serialization matched.".to_string())
}

fn run_rest_response(_fixture_id: &str, fixture: &Value) -> Result<String, String> {
    let expected = &fixture["expected"];
    let page =
        parse_fills_page(fixture["input"]["body"].clone()).map_err(|error| error.to_string())?;
    let first_fill = page
        .fills
        .first()
        .ok_or("Fills response did not contain a first fill.")?;
    let expected_first = &expected["firstFill"];
    expect_eq(
        number(expected, "fillsCount")?,
        page.fills.len() as i64,
        "fills count",
    )?;
    expect_eq(
        string(expected_first, "fillId")?,
        first_fill.fill_id.as_deref().unwrap_or_default(),
        "fill id",
    )?;
    expect_eq(
        number(expected_first, "exchangeIndex")?,
        i64::from(first_fill.exchange_index.unwrap_or_default()),
        "exchange index",
    )?;
    expect_eq(
        string(expected_first, "tradeId")?,
        &first_fill.trade_id,
        "trade id",
    )?;
    expect_eq(
        string(expected_first, "orderId")?,
        &first_fill.order_id,
        "order id",
    )?;
    expect_eq(
        string(expected_first, "ticker")?,
        &first_fill.ticker,
        "ticker",
    )?;
    expect_eq(
        string(expected_first, "marketTicker")?,
        first_fill.market_ticker.as_deref().unwrap_or_default(),
        "market ticker",
    )?;
    expect_eq(
        string(expected_first, "outcomeSide")?,
        first_fill.outcome_side.as_deref().unwrap_or_default(),
        "outcome side",
    )?;
    expect_eq(
        string(expected_first, "bookSide")?,
        first_fill.book_side.as_deref().unwrap_or_default(),
        "book side",
    )?;
    expect_eq(
        string(expected_first, "countFp")?,
        first_fill.count_fp.as_deref().unwrap_or_default(),
        "count fp",
    )?;
    expect_eq(
        string(expected_first, "yesPriceDollars")?,
        first_fill.yes_price_dollars.as_deref().unwrap_or_default(),
        "yes price dollars",
    )?;
    expect_eq(
        string(expected_first, "noPriceDollars")?,
        first_fill.no_price_dollars.as_deref().unwrap_or_default(),
        "no price dollars",
    )?;
    expect_eq(
        string(expected_first, "feeCost")?,
        first_fill.fee_cost.as_deref().unwrap_or_default(),
        "fee cost",
    )?;
    expect_eq(string(expected_first, "side")?, &first_fill.side, "side")?;
    expect_eq(
        string(expected_first, "action")?,
        first_fill.action.as_deref().unwrap_or_default(),
        "action",
    )?;
    expect_eq(
        boolean(expected_first, "isTaker")?,
        first_fill.is_taker,
        "is taker",
    )?;
    expect_eq(
        number(expected_first, "subaccountNumber")?,
        i64::from(first_fill.subaccount_number.unwrap_or_default()),
        "subaccount number",
    )?;
    expect_eq(
        number(expected_first, "ts")?,
        first_fill.ts.unwrap_or_default(),
        "ts",
    )?;
    expect_eq(
        string(expected, "cursor")?,
        page.cursor.as_deref().unwrap_or_default(),
        "cursor",
    )?;
    Ok("Portfolio fills response shape parsed.".to_string())
}

fn run_websocket_replay(_fixture_id: &str, fixture: &Value) -> Result<String, String> {
    let messages = fixture["input"]["messages"]
        .as_array()
        .ok_or("input.messages must be an array.")?;
    let expected = &fixture["expected"];
    let mut dispatched_types = Vec::new();
    let mut last_delta: Option<Value> = None;
    for raw_message in messages {
        let message =
            parse_websocket_message(raw_message.clone()).map_err(|error| error.to_string())?;
        dispatched_types.push(message.message_type.clone());
        if message.message_type == "orderbook_delta" {
            last_delta = message.msg;
        }
    }
    expect_eq(
        string_array(expected, "dispatchedTypes")?.join(","),
        dispatched_types.join(","),
        "dispatched types",
    )?;
    let last_delta = last_delta.ok_or("Replay did not dispatch an orderbook_delta message.")?;
    let patch = &expected["finalBookPatch"];
    let yes = patch["yes"]
        .as_object()
        .ok_or("finalBookPatch.yes must be an object.")?;
    let (price, delta) = yes
        .iter()
        .next()
        .ok_or("finalBookPatch.yes must have an entry.")?;
    expect_eq(
        string(patch, "ticker")?,
        string(&last_delta, "market_ticker")?,
        "order book ticker",
    )?;
    expect_eq(
        price.parse::<i64>().map_err(|error| error.to_string())?,
        number(&last_delta, "price")?,
        "order book price",
    )?;
    expect_eq(
        delta.as_i64().ok_or("order book delta must be a number.")?,
        number(&last_delta, "delta")?,
        "order book delta",
    )?;
    expect_eq("yes", string(&last_delta, "side")?, "order book side")?;
    Ok("WebSocket replay dispatched expected messages.".to_string())
}

fn read_fixture(root: &Path, fixture_id: &str) -> Result<Value, String> {
    let inventory = read_json(&root.join("fixtures/inventory.json")).ok();
    if let Some(inventory) = inventory {
        if let Some(fixtures) = inventory["fixtures"].as_array() {
            for fixture in fixtures {
                if fixture["id"].as_str() == Some(fixture_id) {
                    if let Some(path) = fixture["path"].as_str() {
                        return read_json(&root.join(path)).map_err(|error| error.to_string());
                    }
                }
            }
        }
    }

    let path = match fixture_id {
        "auth.signing.rest-get-portfolio-fills" => {
            "fixtures/auth/signing/rest-get-portfolio-fills.json"
        }
        "rest.request.portfolio-fills-query" => {
            "fixtures/rest/request-serialization/portfolio-fills-query.json"
        }
        "rest.response.portfolio-fills-page" => {
            "fixtures/rest/response-shape/portfolio-fills-page.json"
        }
        "websocket.replay.orderbook-delta" => "fixtures/websocket/replay/orderbook-delta.json",
        _ => return Err(format!("Fixture {fixture_id} was not found.")),
    };
    read_json(&root.join(path)).map_err(|error| error.to_string())
}

fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn option_value(name: &str) -> Option<String> {
    let args = std::env::args().collect::<Vec<_>>();
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
}

fn pass(id: &str, message: &str) -> CheckResult {
    CheckResult {
        id: id.to_string(),
        passed: true,
        message: message.to_string(),
    }
}

fn fail(id: &str, message: &str) -> CheckResult {
    CheckResult {
        id: id.to_string(),
        passed: false,
        message: message.to_string(),
    }
}

fn string<'a>(body: &'a Value, name: &str) -> Result<&'a str, String> {
    body[name]
        .as_str()
        .ok_or_else(|| format!("{name} must be a string."))
}

fn optional_string(body: &Value, name: &str) -> Option<String> {
    body[name].as_str().map(ToString::to_string)
}

fn string_array(body: &Value, name: &str) -> Result<Vec<String>, String> {
    body[name]
        .as_array()
        .ok_or_else(|| format!("{name} must be an array."))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToString::to_string)
                .ok_or_else(|| format!("{name} entries must be strings."))
        })
        .collect()
}

fn number(body: &Value, name: &str) -> Result<i64, String> {
    body[name]
        .as_i64()
        .ok_or_else(|| format!("{name} must be a number."))
}

fn optional_u32(body: &Value, name: &str) -> Result<Option<u32>, String> {
    match body[name].as_u64() {
        Some(value) => u32::try_from(value)
            .map(Some)
            .map_err(|error| error.to_string()),
        None => Ok(None),
    }
}

fn boolean(body: &Value, name: &str) -> Result<bool, String> {
    body[name]
        .as_bool()
        .ok_or_else(|| format!("{name} must be a boolean."))
}

fn expect_eq<T>(expected: T, actual: T, name: &str) -> Result<(), String>
where
    T: PartialEq + std::fmt::Display,
{
    if expected == actual {
        Ok(())
    } else {
        Err(format!(
            "{name} mismatch. Expected {expected}, got {actual}."
        ))
    }
}

fn timestamp_utc() -> String {
    chrono::Utc::now().to_rfc3339()
}
