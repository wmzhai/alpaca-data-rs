use alpaca_data::{Client, Error, options};
use futures_util::StreamExt;
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn authed_client(base_url: String) -> Client {
    Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url(base_url)
        .build()
        .expect("client should build")
}

#[tokio::test]
async fn malformed_snapshots_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots"))
        .and(query_param("symbols", "AAPL260406C00180000"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .options()
        .snapshots(options::SnapshotsRequest {
            symbols: vec!["AAPL260406C00180000".into()],
            ..Default::default()
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn malformed_condition_codes_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/meta/conditions/trade"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .options()
        .condition_codes(options::ConditionCodesRequest {
            ticktype: options::TickType::Trade,
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn snapshots_all_rejects_duplicate_symbols_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots"))
        .and(query_param(
            "symbols",
            "AAPL260406C00180000,AAPL260406C00185000",
        ))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "snapshots": {
                "AAPL260406C00180000": {
                    "latestQuote": { "bp": 73.95 }
                }
            },
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots"))
        .and(query_param(
            "symbols",
            "AAPL260406C00180000,AAPL260406C00185000",
        ))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "snapshots": {
                "AAPL260406C00180000": {
                    "latestQuote": { "bp": 74.00 }
                }
            },
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .options()
        .snapshots_all(options::SnapshotsRequest {
            symbols: vec!["AAPL260406C00180000".into(), "AAPL260406C00185000".into()],
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject duplicate symbols");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn chain_stream_rejects_duplicate_symbols_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots/AAPL"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "snapshots": {
                "AAPL260406C00180000": {
                    "latestQuote": { "bp": 73.95 }
                }
            },
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots/AAPL"))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "snapshots": {
                "AAPL260406C00180000": {
                    "latestQuote": { "bp": 74.00 }
                }
            },
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let pages = authed_client(server.uri())
        .options()
        .chain_stream(options::ChainRequest {
            underlying_symbol: "AAPL".into(),
            limit: Some(1),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(pages.len(), 2);
    assert!(pages[0].as_ref().is_ok());
    assert!(matches!(pages[1], Err(Error::Pagination(_))));
}
