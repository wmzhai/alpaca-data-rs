use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
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

fn run_audit(
    openapi_path: &Path,
    manifest_path: &Path,
    docs_path: &Path,
    extra_args: &[&str],
) -> Output {
    let openapi_url = format!("file://{}", openapi_path.display());
    let mut command = Command::new("bash");
    command.arg(repo_root().join("scripts/api-sync-audit"));
    command.env("ALPACA_MARKET_DATA_OPENAPI_URL", openapi_url);
    command.env("ALPACA_MARKET_DATA_MANIFEST_PATH", manifest_path);
    command.env("ALPACA_MARKET_DATA_DOCS_PATH", docs_path);
    command.args(extra_args);
    command.output().expect("script should run")
}

#[test]
fn api_sync_audit_rejects_unexpected_arguments() {
    let fixture_dir = temp_fixture_dir("api-sync-audit-args");
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
  "paths": {},
  "components": {
    "parameters": {},
    "schemas": {}
  }
}"##,
    )
    .expect("fixture openapi should be written");

    fs::write(
        &manifest_path,
        r##"{
  "generated_at": "2026-04-04",
  "summary": {
    "official_total_paths": 0,
    "adopted_family_total_paths": 0,
    "implemented_mirror_paths": 0,
    "adopted_family_open_gaps": 0
  },
  "mirror_layer": [],
  "convenience_layer": [],
  "gaps_in_adopted_families": [],
  "not_adopted_resource_families": [],
  "known_parity_gaps": []
}"##,
    )
    .expect("fixture manifest should be written");

    fs::write(&docs_path, "# Fixture API coverage\n").expect("fixture docs should be written");

    let output = run_audit(&openapi_path, &manifest_path, &docs_path, &["--help"]);

    let _ = fs::remove_dir_all(&fixture_dir);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("does not accept command-line arguments"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn api_sync_audit_reports_crypto_loc_schema_ref_gap() {
    let fixture_dir = temp_fixture_dir("api-sync-audit-loc-gap");
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
        "operationId": "CryptoBars",
        "parameters": [
          { "$ref": "#/components/parameters/crypto_historical_loc" }
        ],
        "responses": {
          "200": {
            "description": "ok",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "bars": {
                      "type": "object",
                      "additionalProperties": {
                        "type": "array",
                        "items": {
                          "type": "object",
                          "properties": {
                            "c": { "type": "number" },
                            "t": { "type": "string" }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
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
      }
    },
    "schemas": {
      "crypto_historical_loc": {
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
      "response_type": "crypto::BarsResponse",
      "parameter_contract": {
        "signatures": [
          "loc|path|optional|string|enum=bs-1,eu-1,us,us-1,us-2"
        ]
      },
      "response_contract": {
        "status": "200",
        "field_signatures": [
          "bars|object",
          "bars.*|array",
          "bars.*[]|object",
          "bars.*[].c|number",
          "bars.*[].t|string"
        ]
      }
    }
  ],
  "convenience_layer": [],
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

    let output = run_audit(&openapi_path, &manifest_path, &docs_path, &[]);

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

#[test]
fn api_sync_audit_reports_parameter_and_response_drift() {
    let fixture_dir = temp_fixture_dir("api-sync-audit-drift");
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
        "operationId": "CryptoBars",
        "parameters": [
          { "$ref": "#/components/parameters/crypto_historical_loc" },
          {
            "name": "symbols",
            "in": "query",
            "required": true,
            "schema": { "type": "string" }
          },
          {
            "name": "limit",
            "in": "query",
            "required": false,
            "schema": { "type": "integer", "default": 500 }
          },
          {
            "name": "feed",
            "in": "query",
            "required": false,
            "schema": { "type": "string", "enum": ["us", "global"] }
          }
        ],
        "responses": {
          "200": {
            "description": "ok",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "bars": {
                      "type": "object",
                      "additionalProperties": {
                        "type": "array",
                        "items": {
                          "type": "object",
                          "properties": {
                            "c": { "type": "number" },
                            "o": { "type": "number" }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  },
  "components": {
    "parameters": {
      "crypto_historical_loc": {
        "name": "loc",
        "in": "path",
        "required": true,
        "schema": {
          "$ref": "#/components/schemas/crypto_historical_loc"
        }
      }
    },
    "schemas": {
      "crypto_historical_loc": {
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
      "response_type": "crypto::BarsResponse",
      "parameter_contract": {
        "signatures": [
          "limit|query|optional|integer|default=1000",
          "loc|path|required|string|enum=bs-1,eu-1,us,us-1",
          "symbols|query|required|string"
        ]
      },
      "response_contract": {
        "status": "200",
        "field_signatures": [
          "bars|object",
          "bars.*|array",
          "bars.*[]|object",
          "bars.*[].c|number",
          "bars.*[].t|string"
        ]
      }
    }
  ],
  "convenience_layer": [
    {
      "resource": "crypto",
      "base_mirror_method": "bars",
      "helpers": ["bars_all", "bars_stream"]
    }
  ],
  "gaps_in_adopted_families": [],
  "not_adopted_resource_families": [],
  "known_parity_gaps": []
}"##,
    )
    .expect("fixture manifest should be written");

    fs::write(&docs_path, "# Fixture API coverage\n").expect("fixture docs should be written");

    let output = run_audit(&openapi_path, &manifest_path, &docs_path, &[]);

    let _ = fs::remove_dir_all(&fixture_dir);

    assert!(
        !output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Blocking: parameter drift in adopted-family mirror contracts"),
        "{stdout}"
    );
    assert!(
        stdout.contains("Blocking: response-field drift in adopted-family mirror contracts"),
        "{stdout}"
    );
    assert!(
        stdout.contains("added parameter signature: feed|query|optional|string|enum=global,us"),
        "{stdout}"
    );
    assert!(
        stdout.contains("missing response field signature: bars.*[].t|string"),
        "{stdout}"
    );
    assert!(
        stdout.contains("added response field signature: bars.*[].o|number"),
        "{stdout}"
    );
    assert!(
        stdout.contains("Convenience compatibility notes"),
        "{stdout}"
    );
    assert!(
        stdout.contains("crypto.bars helpers require re-validation: bars_all, bars_stream"),
        "{stdout}"
    );
}
