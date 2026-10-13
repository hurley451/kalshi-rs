use std::{fs, path::Path, process::Command};

fn compile(source: &str, deps: &Path, library: &Path) -> std::process::Output {
    let directory = tempfile::tempdir().unwrap();
    let file = directory.path().join("consumer.rs");
    fs::write(&file, source).unwrap();
    Command::new("rustc")
        .args(["--edition=2024", "--crate-type=bin", "--emit=metadata"])
        .arg("--extern")
        .arg(format!("kalshi_client={}", library.display()))
        .arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg("-o")
        .arg(directory.path().join("consumer.rmeta"))
        .arg(file)
        .output()
        .unwrap()
}

#[test]
fn compiler_enforces_required_fields_single_assignment_and_builder_secrecy() {
    let target = tempfile::tempdir().unwrap();
    let build = Command::new("cargo")
        .args([
            "build",
            "--locked",
            "--offline",
            "--lib",
            "--message-format=json",
            "--manifest-path",
        ])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .arg("--target-dir")
        .arg(target.path())
        .output()
        .unwrap();
    let diagnostics = String::from_utf8_lossy(&build.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|message| message["reason"] == "compiler-message")
        .filter_map(|message| message["message"]["rendered"].as_str().map(str::to_owned))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        build.status.success(),
        "consumer dependency build failed: {}\n{}",
        String::from_utf8_lossy(&build.stderr),
        diagnostics
    );
    let libraries: Vec<std::path::PathBuf> = String::from_utf8(build.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|message| {
            message["reason"] == "compiler-artifact" && message["target"]["name"] == "kalshi_client"
        })
        .flat_map(|message| message["filenames"].as_array().unwrap().clone())
        .filter_map(|file| {
            file.as_str()
                .filter(|file| file.ends_with(".rlib"))
                .map(std::path::PathBuf::from)
        })
        .collect();
    assert_eq!(libraries.len(), 1, "exact compiled SDK library missing");
    // Each candidate must expose the API before its negative results count.
    for library in libraries {
        let dependency_directory = library.parent().unwrap().join("deps");
        let deps = dependency_directory.as_path();
        let positive = compile(
            r#"
            use kalshi_client::models::*;
            fn main() {
                let _ = GetMarketsParameters::builder().opt_limit(Some(100)).opt_cursor(None).build();
                let _ = CreateOrderGroupRequest::builder().contracts_limit_fp(None).build();
                let _ = PriceDistributionHistorical::builder().close(None).high(None).low(None)
                    .mean(None).open(None).previous(None).build();
            }
        "#,
            deps,
            &library,
        );
        assert!(
            positive.status.success(),
            "positive consumer failed: {}",
            String::from_utf8_lossy(&positive.stderr)
        );
        let cases = [
            (
                "missing required",
                "CreateOrderV2Request::builder().ticker(\"T\").build();",
                "Missing",
            ),
            (
                "required nullable",
                "PriceDistributionHistorical::builder().high(None).low(None).mean(None).open(None).previous(None).build();",
                "close",
            ),
            (
                "duplicate setter",
                "CreateOrderV2Request::builder().ticker(\"T\").ticker(\"U\");",
                "ticker",
            ),
            (
                "unset builder Debug",
                "format!(\"{:?}\", CreateOrderV2Request::builder());",
                "Debug",
            ),
            (
                "partial builder Debug",
                "format!(\"{:?}\", CreateOrderV2Request::builder().ticker(\"SENTINEL\"));",
                "Debug",
            ),
            (
                "complete builder Debug",
                "format!(\"{:?}\", CreateQuoteRequest::builder().rfq_id(\"SENTINEL\").yes_bid(\"0.0100\".parse::<FixedPointDollars>().unwrap()).no_bid(\"0.0200\".parse::<FixedPointDollars>().unwrap()).rest_remainder(false));",
                "Debug",
            ),
            (
                "wrong native field",
                "CreateOrderV2Request::builder().count(\"1.00\");",
                "mismatched types",
            ),
        ];
        for (name, body, diagnostic) in cases {
            let output = compile(
                &format!("use kalshi_client::models::*; fn main() {{ {body} }}"),
                deps,
                &library,
            );
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(!output.status.success(), "{name} unexpectedly compiled");
            assert!(
                stderr.contains(diagnostic),
                "{name} failed for the wrong reason: {stderr}"
            );
        }
    }
}
