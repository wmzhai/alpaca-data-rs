use alpaca_data::{Client, Error, stocks};
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
