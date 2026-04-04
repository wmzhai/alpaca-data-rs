use alpaca_data::{Client, Error, corporate_actions, news};
use futures_util::StreamExt;
use serde_json::json;
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

fn news_item(id: i64, symbol: &str) -> serde_json::Value {
    json!({
        "id": id,
        "headline": format!("headline-{id}"),
        "author": "Author",
        "created_at": "2026-04-04T00:00:00Z",
        "updated_at": "2026-04-04T00:00:01Z",
        "summary": "Summary",
        "content": "",
        "url": format!("https://example.com/{id}"),
        "images": [],
        "symbols": [symbol],
        "source": "benzinga"
    })
}

fn cash_dividend(id: &str, symbol: &str) -> serde_json::Value {
    json!({
        "id": id,
        "symbol": symbol,
        "cusip": "037833100",
        "rate": 0.25,
        "special": false,
        "foreign": false,
        "process_date": "2026-04-04",
        "ex_date": "2026-04-04",
        "record_date": "2026-04-04",
        "payable_date": "2026-04-05"
    })
}

fn name_change(id: &str, old_symbol: &str, new_symbol: &str) -> serde_json::Value {
    json!({
        "id": id,
        "old_symbol": old_symbol,
        "old_cusip": "111111111",
        "new_symbol": new_symbol,
        "new_cusip": "222222222",
        "process_date": "2026-04-04"
    })
}

#[tokio::test]
async fn malformed_news_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(query_param("symbols", "AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .news()
        .list(news::ListRequest {
            symbols: Some(vec!["AAPL".into()]),
            ..Default::default()
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn news_list_all_merges_pages_and_clears_next_page_token() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(query_param("symbols", "AAPL,MSFT"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "news": [news_item(1, "AAPL")],
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(query_param("symbols", "AAPL,MSFT"))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "news": [news_item(2, "MSFT")],
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let response = authed_client(server.uri())
        .news()
        .list_all(news::ListRequest {
            symbols: Some(vec!["AAPL".into(), "MSFT".into()]),
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect("pagination should succeed");

    assert_eq!(response.news.len(), 2);
    assert_eq!(response.next_page_token, None);
}

#[tokio::test]
async fn news_list_all_rejects_repeated_next_page_token() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(query_param("symbols", "AAPL"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "news": [news_item(1, "AAPL")],
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(query_param("symbols", "AAPL"))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "news": [news_item(2, "AAPL")],
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .news()
        .list_all(news::ListRequest {
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect_err("repeated next_page_token should fail");

    assert!(matches!(error, Error::Pagination(_)));
}

#[tokio::test]
async fn malformed_corporate_actions_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = authed_client(server.uri())
        .corporate_actions()
        .list(corporate_actions::ListRequest {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn corporate_actions_list_all_merges_bucketed_pages_and_clears_next_page_token() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "corporate_actions": {
                "cash_dividends": [cash_dividend("ca-1", "AAPL")],
                "contract_adjustments": [{"id": "undoc-1", "memo": "first"}]
            },
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "corporate_actions": {
                "name_changes": [name_change("ca-2", "META1", "META")],
                "partial_calls": [{"id": "undoc-2", "memo": "second"}],
                "mystery_bucket": [{"id": "mystery-1"}]
            },
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let response = authed_client(server.uri())
        .corporate_actions()
        .list_all(corporate_actions::ListRequest {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .expect("pagination should succeed");

    assert_eq!(response.corporate_actions.cash_dividends.len(), 1);
    assert_eq!(response.corporate_actions.name_changes.len(), 1);
    assert_eq!(response.corporate_actions.contract_adjustments.len(), 1);
    assert_eq!(response.corporate_actions.partial_calls.len(), 1);
    assert_eq!(
        response
            .corporate_actions
            .other
            .get("mystery_bucket")
            .map(Vec::len),
        Some(1)
    );
    assert_eq!(response.next_page_token, None);
}

#[tokio::test]
async fn corporate_actions_list_stream_rejects_repeated_next_page_token() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("limit", "1"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "corporate_actions": {
                "cash_dividends": [cash_dividend("ca-1", "AAPL")]
            },
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("limit", "1"))
        .and(query_param("page_token", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "corporate_actions": {
                "name_changes": [name_change("ca-2", "OLD", "NEW")]
            },
            "next_page_token": "page-2"
        })))
        .mount(&server)
        .await;

    let pages = authed_client(server.uri())
        .corporate_actions()
        .list_stream(corporate_actions::ListRequest {
            limit: Some(1),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(pages.len(), 2);
    assert!(pages[0].as_ref().is_ok());
    assert!(matches!(pages[1], Err(Error::Pagination(_))));
}
