use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn generate_doc_site_writes_expected_public_artifacts() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_root = unique_temp_dir();

    copy_path(
        &repo_root.join("Cargo.toml"),
        &fixture_root.join("Cargo.toml"),
    );
    copy_path(
        &repo_root.join("README.md"),
        &fixture_root.join("README.md"),
    );
    copy_path(&repo_root.join("src"), &fixture_root.join("src"));
    copy_path(&repo_root.join("docs"), &fixture_root.join("docs"));
    copy_path(&repo_root.join("examples"), &fixture_root.join("examples"));
    copy_path(&repo_root.join("tests"), &fixture_root.join("tests"));
    copy_path(&repo_root.join("benches"), &fixture_root.join("benches"));
    copy_path(&repo_root.join("tools"), &fixture_root.join("tools"));

    let output = Command::new("python3")
        .arg("tools/docs/generate-doc-site")
        .current_dir(&fixture_root)
        .output()
        .expect("generator command should launch");

    assert!(
        output.status.success(),
        "generator should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let docs_index = read_to_string(&fixture_root.join("docs/index.md"));
    assert!(docs_index.contains("# Documentation"));
    assert!(docs_index.contains("API Reference"));

    let project_structure = read_to_string(&fixture_root.join("docs/project-structure.md"));
    assert!(project_structure.contains("# Project Structure"));
    assert!(project_structure.contains("src/stocks"));

    let stocks_reference = read_to_string(&fixture_root.join("docs/reference/stocks.md"));
    assert!(stocks_reference.contains("alpaca_data::stocks"));
    assert!(stocks_reference.contains("StocksClient"));
    assert!(stocks_reference.contains("bars_all"));

    let common_reference = read_to_string(&fixture_root.join("docs/reference/common.md"));
    assert!(common_reference.contains("Currency"));
    assert!(!common_reference.contains("https://docs.rs/alpaca-data/latest/alpaca_data/common/"));
    assert!(
        !common_reference
            .contains("https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/")
    );

    let transport_reference = read_to_string(&fixture_root.join("docs/reference/transport.md"));
    assert!(transport_reference.contains("ObservedResponseMeta"));
    assert!(transport_reference.contains("TransportObserver"));
    assert!(transport_reference.contains(
        "https://docs.rs/alpaca-data/latest/alpaca_data/struct.ObservedResponseMeta.html"
    ));
    assert!(
        transport_reference.contains(
            "https://docs.rs/alpaca-data/latest/alpaca_data/trait.TransportObserver.html"
        )
    );
    assert!(transport_reference.contains(
        "https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ObservedResponseMeta.html"
    ));
    assert!(transport_reference.contains(
        "https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/trait.TransportObserver.html"
    ));
    assert!(
        !transport_reference.contains("https://docs.rs/alpaca-data/latest/alpaca_data/transport/")
    );
    assert!(
        !transport_reference
            .contains("https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/")
    );
    assert!(!transport_reference.contains("TransportErrorMeta"));
    assert!(!transport_reference.contains("struct.TransportErrorMeta.html"));

    let api_index = read_to_string(&fixture_root.join("docs/generated/api-index.json"));
    assert!(api_index.contains("\"resource_accessors\""));
    assert!(api_index.contains("\"stocks\""));
    assert!(api_index.contains("\"ObservedResponseMeta\""));
    assert!(api_index.contains("\"TransportObserver\""));
    assert!(!api_index.contains("\"TransportErrorMeta\""));

    let sidebars = read_to_string(&fixture_root.join("website/sidebars.ts"));
    assert!(sidebars.contains("reference/stocks"));
    assert!(sidebars.contains("project-structure"));

    let readme = read_to_string(&fixture_root.join("README.md"));
    assert!(readme.contains("<!-- docs-site:start -->"));
    assert!(readme.contains("https://wmzhai.github.io/alpaca-data-rs/"));
}

fn unique_temp_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    path.push(format!(
        "alpaca-data-doc-site-{stamp}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn copy_path(from: &Path, to: &Path) {
    if from.is_dir() {
        fs::create_dir_all(to).expect("destination directory should be created");
        for entry in fs::read_dir(from).expect("source directory should be readable") {
            let entry = entry.expect("directory entry should be readable");
            copy_path(&entry.path(), &to.join(entry.file_name()));
        }
        return;
    }

    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::copy(from, to).expect("file should be copied");
}

fn read_to_string(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}
