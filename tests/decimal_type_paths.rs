use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp_project_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("decimal-type-path-tests")
        .join(format!("{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).expect("temp project directory should be created");
    dir
}

fn write_project(dir: &Path, main_rs: &str) {
    let dependency_path = env!("CARGO_MANIFEST_DIR").replace('\\', "\\\\");
    let manifest = format!(
        r#"[package]
name = "decimal-type-path-test"
version = "0.0.0"
edition = "2024"

[dependencies]
alpaca-data = {{ path = "{dependency_path}" }}
rust_decimal = "1"
"#
    );
    fs::write(dir.join("Cargo.toml"), manifest).expect("temp manifest should be written");
    fs::write(dir.join("src/main.rs"), main_rs).expect("temp main.rs should be written");
}

fn cargo_check(dir: &Path) -> Output {
    Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--offline")
        .current_dir(dir)
        .output()
        .expect("cargo check should run")
}

#[test]
fn rust_decimal_path_is_the_supported_public_entrypoint() {
    let dir = temp_project_dir("rust-decimal");
    write_project(
        &dir,
        r#"use std::str::FromStr;

use alpaca_data::options;
use rust_decimal::Decimal;

fn main() {
    let request = options::ChainRequest {
        underlying_symbol: "AAPL".into(),
        feed: None,
        r#type: Some(options::ContractType::Call),
        strike_price_gte: Some(Decimal::from_str("180.0").expect("decimal literal should parse")),
        strike_price_lte: Some(Decimal::from_str("220.0").expect("decimal literal should parse")),
        expiration_date: None,
        expiration_date_gte: None,
        expiration_date_lte: None,
        root_symbol: None,
        updated_since: None,
        limit: Some(5),
        page_token: None,
    };

    let _ = request;
}"#,
    );

    let output = cargo_check(&dir);
    if !output.status.success() {
        panic!(
            "rust_decimal public path should compile\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn crate_root_decimal_reexport_stays_unavailable() {
    let dir = temp_project_dir("crate-root-decimal");
    let crate_name = ["alpaca_data", "Decimal"].join("::");
    let main_rs = format!(r#"fn main() {{ let _: {crate_name} = "1.23".parse().unwrap(); }}"#);
    write_project(&dir, &main_rs);

    let output = cargo_check(&dir);
    assert!(
        !output.status.success(),
        "crate root Decimal path should stay unavailable"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot find type `Decimal` in crate `alpaca_data`"),
        "stderr should explain that the crate root Decimal path is missing, got:\n{stderr}"
    );
}
