use kalshi_client::{
    DEMO, FillQuery, KalshiAuth, KalshiClient, KalshiRequest, QueryValue, parse_fills_page,
    portfolio_fills_query,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let scenario = args.next().unwrap_or_else(|| "status".to_string());
    let is_private = scenario == "fills";
    let auth = if is_private {
        Some(KalshiAuth {
            api_key_id: std::env::var("KALSHI_API_KEY_ID")?,
            private_key_pem: std::fs::read_to_string(std::env::var("KALSHI_PRIVATE_KEY_FILE")?)?,
        })
    } else {
        None
    };
    let client = KalshiClient::new(DEMO, auth);
    let mut request = KalshiRequest {
        authenticated: is_private,
        ..Default::default()
    };
    let operation = match scenario.as_str() {
        "status" => "GetExchangeStatus",
        "markets" => {
            request.query = vec![
                ("limit".into(), QueryValue::from(5_u32)),
                ("status".into(), QueryValue::from("open")),
            ];
            "GetMarkets"
        }
        "orderbook" => {
            let ticker = args
                .next()
                .ok_or("Pass a demo market ticker after orderbook.")?;
            request.path_parameters.insert("ticker".into(), ticker);
            request
                .query
                .push(("depth".into(), QueryValue::from(5_u32)));
            "GetMarketOrderbook"
        }
        "fills" => {
            request.query = portfolio_fills_query(&FillQuery {
                limit: Some(5),
                ..Default::default()
            })?;
            "GetFills"
        }
        _ => return Err("Choose status, markets, orderbook, or fills.".into()),
    };
    let body = client.request(operation, request).await?;
    if is_private {
        println!("{:#?}", parse_fills_page(body)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&body)?);
    }
    Ok(())
}
