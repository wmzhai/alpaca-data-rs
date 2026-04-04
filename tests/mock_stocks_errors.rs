use alpaca_data::{Client, Error, stocks};
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
async fn malformed_single_historical_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .bars_single(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::from("1Day"),
            ..Default::default()
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn malformed_auctions_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/auctions"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .auctions(stocks::AuctionsRequest {
            symbols: vec!["AAPL".into()],
            ..Default::default()
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn auctions_single_all_rejects_mismatched_symbol_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/auctions"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "auctions": [],
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/auctions"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "MSFT",
            "auctions": [],
            "next_page_token": null,
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .auctions_single_all(stocks::AuctionsSingleRequest {
            symbol: "AAPL".into(),
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject mismatched symbol");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn auctions_all_rejects_mismatched_batch_currency_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/auctions"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "auctions": {
                "AAPL": [
                    { "d": "2024-03-01", "o": [], "c": [] }
                ]
            },
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/auctions"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "auctions": {
                "AAPL": [
                    { "d": "2024-03-04", "o": [], "c": [] }
                ]
            },
            "next_page_token": null,
            "currency": "CAD"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .auctions_all(stocks::AuctionsRequest {
            symbols: vec!["AAPL".into()],
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject mismatched currency");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn bars_single_all_rejects_mismatched_symbol_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "MSFT",
            "bars": [],
            "next_page_token": null,
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .bars_single_all(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::from("1Day"),
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject mismatched symbol");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn bars_single_all_rejects_mismatched_currency_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": null,
            "currency": "CAD"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .bars_single_all(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::from("1Day"),
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject mismatched currency");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn bars_single_stream_rejects_mismatched_symbol_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "MSFT",
            "bars": [],
            "next_page_token": null,
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    let pages = authed_client(server.uri())
        .stocks()
        .bars_single_stream(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::from("1Day"),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(pages.len(), 2);
    assert!(pages[0].as_ref().is_ok());
    assert!(matches!(pages[1], Err(Error::Pagination(_))));
}

#[tokio::test]
async fn bars_single_stream_rejects_mismatched_currency_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/AAPL/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "symbol": "AAPL",
            "bars": [],
            "next_page_token": null,
            "currency": "CAD"
        })))
        .mount(&server)
        .await;

    let pages = authed_client(server.uri())
        .stocks()
        .bars_single_stream(stocks::BarsSingleRequest {
            symbol: "AAPL".into(),
            timeframe: stocks::TimeFrame::from("1Day"),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(pages.len(), 2);
    assert!(pages[0].as_ref().is_ok());
    assert!(matches!(pages[1], Err(Error::Pagination(_))));
}

#[tokio::test]
async fn malformed_metadata_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/meta/conditions/trade"))
        .and(query_param("tape", "A"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .condition_codes(stocks::ConditionCodesRequest {
            ticktype: stocks::TickType::Trade,
            tape: stocks::Tape::A,
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn bars_all_rejects_mismatched_batch_currency_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bars": {
                "AAPL": [
                    { "t": "2024-03-01T20:00:00Z", "c": 179.66 }
                ]
            },
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bars": {
                "AAPL": [
                    { "t": "2024-03-04T20:00:00Z", "c": 175.10 }
                ]
            },
            "next_page_token": null,
            "currency": "CAD"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .stocks()
        .bars_all(stocks::BarsRequest {
            symbols: vec!["AAPL".into()],
            timeframe: stocks::TimeFrame::from("1Day"),
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect_err("pagination should reject mismatched currency");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn bars_stream_rejects_mismatched_batch_currency_across_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bars": {
                "AAPL": [
                    { "t": "2024-03-01T20:00:00Z", "c": 179.66 }
                ]
            },
            "next_page_token": "page-2",
            "currency": "USD"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bars": {
                "AAPL": [
                    { "t": "2024-03-04T20:00:00Z", "c": 175.10 }
                ]
            },
            "next_page_token": null,
            "currency": "CAD"
        })))
        .mount(&server)
        .await;

    let pages = authed_client(server.uri())
        .stocks()
        .bars_stream(stocks::BarsRequest {
            symbols: vec!["AAPL".into()],
            timeframe: stocks::TimeFrame::from("1Day"),
            limit: Some(1),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(pages.len(), 2);
    assert!(pages[0].as_ref().is_ok());
    assert!(matches!(pages[1], Err(Error::Pagination(_))));
}
