#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KalshiEnvironment {
    pub name: &'static str,
    pub rest_base_url: &'static str,
    pub websocket_url: &'static str,
}

pub const PRODUCTION: KalshiEnvironment = KalshiEnvironment {
    name: "production",
    rest_base_url: "https://external-api.kalshi.com/trade-api/v2",
    websocket_url: "wss://external-api-ws.kalshi.com/trade-api/ws/v2",
};

pub const DEMO: KalshiEnvironment = KalshiEnvironment {
    name: "demo",
    rest_base_url: "https://demo-api.kalshi.co/trade-api/v2",
    websocket_url: "wss://demo-api.kalshi.co/trade-api/ws/v2",
};
