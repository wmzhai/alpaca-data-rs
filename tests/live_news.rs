use alpaca_data::{Client, news};
use futures_util::StreamExt;

fn live_test_client() -> Client {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"));

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    builder.build().expect("client should build")
}

#[tokio::test]
async fn news_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let list = client
        .news()
        .list(news::ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(news::Sort::Desc),
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(2),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: None,
        })
        .await
        .expect("real Alpaca news list request should succeed");
    assert!(
        !list.news.is_empty(),
        "news list should return at least one article"
    );
    let article = &list.news[0];
    assert!(article.id > 0, "news article id should be populated");
    assert!(
        !article.headline.is_empty(),
        "news article headline should be populated"
    );
    assert_eq!(
        article.content, "",
        "news article content should be present and empty when include_content=false"
    );

    let all = client
        .news()
        .list_all(news::ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(news::Sort::Desc),
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(1),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: None,
        })
        .await
        .expect("real Alpaca news list_all request should succeed");
    assert!(
        !all.news.is_empty(),
        "news list_all should collect at least one article"
    );
    assert_eq!(all.next_page_token, None);

    let pages = client
        .news()
        .list_stream(news::ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(news::Sort::Desc),
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(1),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: None,
        })
        .collect::<Vec<_>>()
        .await;
    assert!(!pages.is_empty(), "news list_stream should yield pages");
    assert!(pages.iter().all(|page| page.is_ok()));
}
