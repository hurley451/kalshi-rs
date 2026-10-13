use kalshi_client::{KalshiAuth, KalshiClient, KalshiEnvironment};
use rsa::{RsaPrivateKey, pkcs8::EncodePrivateKey};
use serde_json::Value;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::{Duration, Instant},
};

#[allow(dead_code)]
pub fn auth() -> KalshiAuth {
    let key = RsaPrivateKey::new(&mut rand_core::OsRng, 2048).unwrap();
    KalshiAuth {
        api_key_id: "FAKE_TEST_KEY_ID".into(),
        private_key_pem: key.to_pkcs8_pem(Default::default()).unwrap().to_string(),
    }
}
pub fn server(
    responses: Vec<(u16, Value)>,
    auth: Option<KalshiAuth>,
) -> (KalshiClient, thread::JoinHandle<Vec<String>>) {
    server_bytes(
        responses
            .into_iter()
            .map(|(status, body)| {
                (
                    status,
                    if body.is_null() {
                        Vec::new()
                    } else {
                        serde_json::to_vec(&body).unwrap()
                    },
                )
            })
            .collect(),
        auth,
    )
}

pub fn server_bytes(
    responses: Vec<(u16, Vec<u8>)>,
    auth: Option<KalshiAuth>,
) -> (KalshiClient, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        for (status, body) in responses {
            let deadline = Instant::now() + Duration::from_secs(10);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            Instant::now() < deadline,
                            "named method did not reach mock transport"
                        );
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) => panic!("{error}"),
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut bytes = Vec::new();
            loop {
                let mut buffer = [0; 4096];
                let n = stream.read(&mut buffer).unwrap();
                assert!(n > 0);
                bytes.extend_from_slice(&buffer[..n]);
                if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]);
                    let length = headers
                        .lines()
                        .find_map(|l| {
                            l.to_ascii_lowercase()
                                .strip_prefix("content-length: ")
                                .map(|v| v.parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if bytes.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            requests.push(String::from_utf8(bytes).unwrap());
            write!(stream,"HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\nx-request-id: fake-probe\r\n\r\n",body.len()).unwrap();
            stream.write_all(&body).unwrap();
        }
        requests
    });
    let environment = KalshiEnvironment {
        name: "typed-loopback",
        rest_base_url: Box::leak(format!("http://{address}/trade-api/v2").into_boxed_str()),
        websocket_url: "ws://unused",
    };
    let http = |builder: reqwest::ClientBuilder| builder.no_proxy().timeout(Duration::from_secs(5));
    (
        KalshiClient::with_http_client_builder(environment, auth, http).unwrap(),
        server,
    )
}

#[allow(dead_code)]
pub fn assert_request(
    id: &str,
    request: &str,
    parameters: Option<&Value>,
    body: Option<&Value>,
    authenticated: bool,
    auth: &KalshiAuth,
) {
    use base64::Engine;
    use rsa::{
        pkcs8::DecodePrivateKey,
        pss::{Signature, VerifyingKey},
        signature::Verifier,
    };
    let coverage: Value =
        serde_json::from_str(include_str!("../../tools/typed-coverage.json")).unwrap();
    let op = coverage["operations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|op| op["id"] == id)
        .unwrap();
    let registry = kalshi_client::get_operation(id).unwrap();
    let mut path = registry.path.to_string();
    if let Some(parameters) = parameters {
        for (name, value) in parameters.as_object().unwrap() {
            if path.contains(&format!("{{{name}}}")) {
                path = path.replace(
                    &format!("{{{name}}}"),
                    &urlencoding::encode(
                        value
                            .as_str()
                            .unwrap_or_else(|| panic!("path fixture must be string")),
                    ),
                );
            }
        }
        kalshi_client::typed::validate(op["parameters"].as_str().unwrap(), parameters).unwrap();
    }
    let first = request.lines().next().unwrap();
    let parts: Vec<_> = first.split_whitespace().collect();
    assert_eq!(parts[0], registry.method, "{id}");
    assert_eq!(
        parts[1].split('?').next().unwrap(),
        format!("/trade-api/v2{path}"),
        "{id}"
    );
    let mut expected = Vec::new();
    if let Some(parameters) = parameters {
        for binding in op["parameterBindings"].as_array().unwrap() {
            if binding["in"] != "query" {
                continue;
            }
            let name = binding["name"].as_str().unwrap();
            let Some(value) = parameters.get(name) else {
                continue;
            };
            let text = |v: &Value| {
                v.as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| v.to_string())
            };
            let values = if let Some(array) = value.as_array() {
                if binding["explode"] == true {
                    array.iter().map(text).collect()
                } else {
                    vec![array.iter().map(text).collect::<Vec<_>>().join(",")]
                }
            } else {
                vec![text(value)]
            };
            for value in values {
                expected.push((name.to_owned(), value));
            }
        }
    }
    let mut actual: Vec<_> = parts[1]
        .split_once('?')
        .map(|(_, query)| {
            query
                .split('&')
                .filter(|s| !s.is_empty())
                .map(|p| {
                    let (name, value) = p.split_once('=').unwrap();
                    (
                        urlencoding::decode(name).unwrap().into_owned(),
                        urlencoding::decode(value).unwrap().into_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected, "{id} query mismatch");
    let headers: std::collections::HashMap<_, _> = request
        .split("\r\n\r\n")
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once(": "))
        .map(|(k, v)| (k.to_ascii_lowercase(), v))
        .collect();
    assert_eq!(
        headers.contains_key("kalshi-access-signature"),
        authenticated,
        "{id}"
    );
    if authenticated {
        assert_eq!(headers["kalshi-access-key"], auth.api_key_id);
        let preimage = kalshi_client::build_signing_preimage(
            headers["kalshi-access-timestamp"],
            parts[0],
            parts[1],
        );
        let key = RsaPrivateKey::from_pkcs8_pem(&auth.private_key_pem).unwrap();
        let signature = Signature::try_from(
            base64::engine::general_purpose::STANDARD
                .decode(headers["kalshi-access-signature"])
                .unwrap()
                .as_slice(),
        )
        .unwrap();
        VerifyingKey::<sha2::Sha256>::new(key.to_public_key())
            .verify(preimage.as_bytes(), &signature)
            .unwrap();
    }
    let sent = request.split_once("\r\n\r\n").unwrap().1;
    if let Some(body) = body {
        assert_eq!(&serde_json::from_str::<Value>(sent).unwrap(), body, "{id}");
    } else {
        assert!(sent.is_empty(), "{id}");
    }
}
