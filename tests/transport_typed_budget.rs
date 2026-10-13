use kalshi_client::*;
use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    time::Duration,
};
use tokio::time::Instant;

// Each test reexecutes itself so response/input schema validators are genuinely cold.
fn isolated(name: &str, run: impl FnOnce()) {
    if std::env::var("KALSHI_TYPED_BUDGET_CASE").as_deref() == Ok(name) {
        run();
        return;
    }
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env("KALSHI_TYPED_BUDGET_CASE", name)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn environment(listener: &TcpListener) -> KalshiEnvironment {
    KalshiEnvironment {
        name: "cold-typed-budget",
        rest_base_url: Box::leak(
            format!("http://{}/trade-api/v2", listener.local_addr().unwrap()).into_boxed_str(),
        ),
        websocket_url: "ws://unused",
    }
}

fn cold_response(status: u16, cancellation: bool) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let client = KalshiClient::builder(environment(&listener))
                .http_builder(|http| http.no_proxy())
                .build()
                .unwrap();
            let token = CancellationToken::new();
            let deadline = Instant::now() + Duration::from_millis(250);
            let server_token = token.clone();
            let server = std::thread::spawn(move || {
                let (mut socket, _) = loop {
                    match listener.accept() {
                        Ok(pair) => break pair,
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline + Duration::from_secs(2));
                            std::thread::sleep(Duration::from_millis(1));
                        }
                        Err(error) => panic!("{error}"),
                    }
                };
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = Vec::new();
                while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                    let mut bytes = [0; 4096];
                    let count = socket.read(&mut bytes).unwrap();
                    assert!(count > 0);
                    request.extend_from_slice(&bytes[..count]);
                }
                if !cancellation {
                    while Instant::now() < deadline - Duration::from_millis(2) {
                        std::thread::sleep(Duration::from_micros(100));
                    }
                }
                let body = if status == 200 {
                    json!({"exchange_active":true,"trading_active":true})
                } else {
                    json!({"code":"test", "message":"test"})
                };
                let body = serde_json::to_vec(&body).unwrap();
                write!(
                    socket,
                    "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                let _ = socket.write_all(&body);
                if cancellation {
                    std::thread::sleep(Duration::from_millis(1));
                    server_token.cancel();
                }
                request
            });
            let client = client.with_request_options(RequestOptions {
                deadline: (!cancellation).then_some(deadline),
                cancellation: cancellation.then_some(token.clone()),
                ..Default::default()
            });
            let result = client.get_exchange_status().await;
            let expired_at_return = Instant::now() >= deadline;
            let cancelled_at_return = token.is_cancelled();
            let error =
                result.expect_err("cold named decode returned success outside its control budget");
            if cancellation {
                assert!(cancelled_at_return);
                assert!(matches!(error.source, KalshiError::Cancelled));
            } else {
                assert!(expired_at_return);
                assert!(matches!(error.source, KalshiError::DeadlineExceeded));
            }
            assert!(error.payload.is_none());
            let request = server.join().unwrap();
            assert!(
                String::from_utf8(request)
                    .unwrap()
                    .starts_with("GET /trade-api/v2/exchange/status ")
            );
        });
}

#[test]
fn cold_named_success_decode_observes_deadline() {
    isolated("cold_named_success_decode_observes_deadline", || {
        cold_response(200, false)
    });
}
#[test]
fn cold_named_error_decode_observes_deadline() {
    isolated("cold_named_error_decode_observes_deadline", || {
        cold_response(500, false)
    });
}
#[test]
fn cold_named_success_decode_observes_concurrent_cancellation() {
    isolated(
        "cold_named_success_decode_observes_concurrent_cancellation",
        || cold_response(200, true),
    );
}
#[test]
fn cold_input_preparation_uses_entry_deadline_and_never_dispatches() {
    isolated(
        "cold_input_preparation_uses_entry_deadline_and_never_dispatches",
        || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let client = KalshiClient::builder(environment(&listener))
                .http_builder(|http| http.no_proxy()).build().unwrap()
                .with_request_options(RequestOptions { timeout: Some(Duration::from_millis(1)), ..Default::default() });
            let result = client.get_markets(&Default::default()).await.unwrap_err();
            assert!(matches!(result.source, KalshiError::DeadlineExceeded));
            assert!(matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock));
        });
        },
    );
}
