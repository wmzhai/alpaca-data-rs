use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn temp_fixture_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let dir =
        std::env::temp_dir().join(format!("alpaca-data-{name}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&dir).expect("temp fixture dir should be created");
    dir
}

#[test]
fn api_sync_openapi_script_help_is_available() {
    let output = Command::new("bash")
        .arg(repo_root().join("scripts/api-sync-openapi"))
        .arg("--help")
        .output()
        .expect("script should run");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Fetch the latest official Alpaca Market Data OpenAPI"));
    assert!(stdout.contains("--json"));
}

#[test]
fn api_sync_audit_script_help_is_available() {
    let output = Command::new("bash")
        .arg(repo_root().join("scripts/api-sync-audit"))
        .arg("--help")
        .output()
        .expect("script should run");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Run a read-only Alpaca Market Data API parity audit"));
    assert!(stdout.contains("--strict"));
}

#[test]
fn api_sync_audit_reports_crypto_loc_schema_ref_gap() {
    let fixture_dir = temp_fixture_dir("api-sync-audit");
    let openapi_path = fixture_dir.join("market-data-api.json");
    let manifest_path = fixture_dir.join("market-data-api-manifest.json");
    let docs_path = fixture_dir.join("api-coverage.md");

    fs::write(
        &openapi_path,
        r##"{
  "info": {
    "title": "Fixture Market Data API",
    "version": "1.1"
  },
  "paths": {
    "/v1beta3/crypto/{loc}/bars": {
      "get": {
        "tags": ["Crypto"],
        "operationId": "CryptoBars"
      }
    }
  },
  "components": {
    "parameters": {
      "crypto_historical_loc": {
        "name": "loc",
        "in": "path",
        "schema": {
          "$ref": "#/components/schemas/crypto_historical_loc"
        }
      },
      "crypto_latest_loc": {
        "name": "loc",
        "in": "path",
        "schema": {
          "$ref": "#/components/schemas/crypto_latest_loc"
        }
      }
    },
    "schemas": {
      "crypto_historical_loc": {
        "type": "string",
        "enum": ["us", "us-1", "us-2", "eu-1", "bs-1"]
      },
      "crypto_latest_loc": {
        "type": "string",
        "enum": ["us", "us-1", "us-2", "eu-1", "bs-1"]
      }
    }
  }
}"##,
    )
    .expect("fixture openapi should be written");

    fs::write(
        &manifest_path,
        r##"{
  "generated_at": "2026-04-04",
  "summary": {
    "official_total_paths": 1,
    "adopted_family_total_paths": 1,
    "implemented_mirror_paths": 1,
    "adopted_family_open_gaps": 0
  },
  "mirror_layer": [
    {
      "tag": "Crypto",
      "operation_id": "CryptoBars",
      "path": "/v1beta3/crypto/{loc}/bars",
      "local_resource": "crypto",
      "mirror_method": "bars",
      "request_type": "crypto::BarsRequest",
      "response_type": "crypto::BarsResponse"
    }
  ],
  "gaps_in_adopted_families": [],
  "not_adopted_resource_families": [],
  "known_parity_gaps": [
    {
      "kind": "parameter_enum",
      "resource": "crypto",
      "subject": "crypto::Loc",
      "local_values": ["us", "us-1", "eu-1"]
    }
  ]
}"##,
    )
    .expect("fixture manifest should be written");

    fs::write(&docs_path, "# Fixture API coverage\n").expect("fixture docs should be written");

    let openapi_url = format!("file://{}", openapi_path.display());
    let output = Command::new("bash")
        .arg(repo_root().join("scripts/api-sync-audit"))
        .env("ALPACA_MARKET_DATA_OPENAPI_URL", openapi_url)
        .env("ALPACA_MARKET_DATA_MANIFEST_PATH", &manifest_path)
        .env("ALPACA_MARKET_DATA_DOCS_PATH", &docs_path)
        .output()
        .expect("script should run");

    let _ = fs::remove_dir_all(&fixture_dir);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Open parity note: official crypto loc values missing from local coverage"),
        "{stdout}"
    );
    assert!(
        stdout.contains("Extend crypto::Loc with the missing official values reported above."),
        "{stdout}"
    );
}
