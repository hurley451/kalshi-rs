#![allow(dead_code)]
use kalshi_client::*;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Notify,
    task::{JoinHandle, JoinSet},
};

#[derive(Clone)]
pub enum Reply {
    Body(u16, Vec<(String, String)>, Vec<u8>),
    Drop,
    StallHeaders,
    StallBody(Vec<u8>),
    Incomplete(Vec<u8>),
    Trickle,
}
impl Reply {
    pub fn json(status: u16, body: serde_json::Value) -> Self {
        Self::Body(
            status,
            vec![
                ("Content-Type".into(), "application/json".into()),
                ("X-Request-Id".into(), "FAKE_SECRET_RESPONSE_HEADER".into()),
            ],
            serde_json::to_vec(&body).unwrap(),
        )
    }
    pub fn bytes(status: u16, body: Vec<u8>) -> Self {
        Self::Body(status, vec![], body)
    }
    pub fn header(mut self, name: &str, value: &str) -> Self {
        if let Self::Body(_, headers, _) = &mut self {
            headers.push((name.into(), value.into()));
        }
        self
    }
}
pub struct Server {
    pub environment: KalshiEnvironment,
    pub logs: Arc<Mutex<Vec<String>>>,
    changed: Arc<Notify>,
    stop: CancellationToken,
    task: JoinHandle<()>,
}
impl Server {
    pub async fn new(replies: Vec<Reply>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let environment = environment(listener.local_addr().unwrap());
        let logs = Arc::new(Mutex::new(Vec::new()));
        let changed = Arc::new(Notify::new());
        let stop = CancellationToken::new();
        let (requests, notify, shutdown) = (logs.clone(), changed.clone(), stop.clone());
        let replies = Arc::new(replies);
        let task = tokio::spawn(async move {
            let mut tasks = JoinSet::new();
            loop {
                tokio::select! {
                    biased;
                    _ = shutdown.cancelled() => break,
                    result = tasks.join_next(), if !tasks.is_empty() => { result.unwrap().unwrap(); },
                    accepted = listener.accept() => {
                        let (mut socket, _) = accepted.unwrap();
                        let (logs, changed, stop, replies) = (requests.clone(), notify.clone(), shutdown.clone(), replies.clone());
                        tasks.spawn(async move {
                            let Some(request) = tokio::time::timeout(Duration::from_secs(3), read_request(&mut socket)).await.unwrap() else { return };
                            let index = { let mut logs = logs.lock().unwrap(); let index = logs.len(); logs.push(request); index };
                            changed.notify_waiters();
                            let reply = replies.get(index).or_else(|| replies.last()).unwrap();
                            tokio::select! { _ = stop.cancelled() => {}, _ = respond(&mut socket, reply) => {} }
                        });
                    }
                }
            }
            tasks.abort_all();
            while tasks.join_next().await.is_some() {}
        });
        Self {
            environment,
            logs,
            changed,
            stop,
            task,
        }
    }
    pub fn client(&self, options: TransportOptions) -> KalshiClient {
        KalshiClient::builder(self.environment)
            .http_builder(|http| http.no_proxy())
            .transport_options(options)
            .build()
            .unwrap()
    }
    pub fn count(&self) -> usize {
        self.logs.lock().unwrap().len()
    }
    pub async fn wait_for(&self, count: usize) {
        tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                let notified = self.changed.notified();
                if self.count() >= count {
                    return;
                }
                notified.await;
            }
        })
        .await
        .unwrap();
    }
    pub async fn close(self) {
        self.stop.cancel();
        self.task.await.unwrap();
    }
}
pub fn environment(address: std::net::SocketAddr) -> KalshiEnvironment {
    KalshiEnvironment {
        name: "loopback",
        rest_base_url: Box::leak(format!("http://{address}/trade-api/v2").into_boxed_str()),
        websocket_url: "ws://unused",
    }
}
pub async fn read_request(socket: &mut TcpStream) -> Option<String> {
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; 4096];
        let count = socket.read(&mut buffer).await.ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&bytes[..end]);
            let length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .and_then(|n| n.parse::<usize>().ok())
                })
                .unwrap_or(0);
            if bytes.len() >= end + 4 + length {
                return Some(String::from_utf8(bytes).unwrap());
            }
        }
        assert!(bytes.len() < 1024 * 1024);
    }
}
async fn respond(socket: &mut TcpStream, reply: &Reply) {
    match reply {
        Reply::Body(status, headers, body) => {
            let headers = headers
                .iter()
                .map(|(name, value)| format!("{name}: {value}\r\n"))
                .collect::<String>();
            if socket.write_all(format!("HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n", body.len()).as_bytes()).await.is_ok() { let _ = socket.write_all(body).await; }
        }
        Reply::Drop => {}
        Reply::StallHeaders => std::future::pending::<()>().await,
        Reply::StallBody(prefix) | Reply::Incomplete(prefix) => {
            let _ = socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\n")
                .await;
            let _ = socket.write_all(prefix).await;
            if matches!(reply, Reply::StallBody(_)) {
                std::future::pending::<()>().await;
            }
        }
        Reply::Trickle => {
            let _ = socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\n")
                .await;
            for _ in 0..100 {
                if socket.write_all(b" ").await.is_err() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        }
    }
}
pub fn public_request() -> KalshiRequest {
    KalshiRequest {
        authenticated: false,
        ..Default::default()
    }
}
pub async fn get(client: &KalshiClient) -> Result<serde_json::Value, KalshiError> {
    client.request("GetExchangeStatus", public_request()).await
}
pub fn fast_retry() -> TransportOptions {
    TransportOptions {
        total_timeout: Some(Duration::from_secs(3)),
        retry: RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
        },
        ..Default::default()
    }
}
pub fn mutation_request(id: &str) -> KalshiRequest {
    let coverage: serde_json::Value =
        serde_json::from_str(include_str!("../../tools/typed-coverage.json")).unwrap();
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("../typed-fixtures.json")).unwrap();
    let operation = coverage["operations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|op| op["id"] == id)
        .unwrap();
    let mut request = public_request();
    for binding in operation["parameterBindings"].as_array().unwrap() {
        if binding["in"] == "path" {
            let name = binding["name"].as_str().unwrap();
            request.path_parameters.insert(
                name.into(),
                fixtures[id]["parameters"][name].as_str().unwrap().into(),
            );
        }
    }
    if !fixtures[id]["body"].is_null() {
        request.body = Some(fixtures[id]["body"].clone());
    }
    request
}
