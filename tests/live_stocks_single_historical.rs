use alpaca_data::{Client, stocks};
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
async fn stocks_single_bars_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let request = stocks::BarsSingleRequest {
        symbol: "AAPL".into(),
        timeframe: stocks::TimeFrame::from("1Day"),
        start: Some("2024-03-01T00:00:00Z".into()),
        end: Some("2024-03-06T00:00:00Z".into()),
        limit: Some(2),
        adjustment: Some(stocks::Adjustment::raw()),
        feed: Some(stocks::DataFeed::Iex),
        sort: Some(stocks::Sort::Asc),
        asof: None,
        currency: Some(stocks::Currency::from("USD")),
        page_token: None,
    };

    let first = client
        .stocks()
        .bars_single(request.clone())
        .await
        .expect("real Alpaca stocks bars_single request should succeed");
    assert_eq!(first.symbol, "AAPL");
    assert!(
        !first.bars.is_empty(),
        "bars_single response should include at least one bar"
    );
    assert!(
        first.bars[0].t.is_some(),
        "decoded single bar timestamp should be populated"
    );

    let all = client
        .stocks()
        .bars_single_all(request.clone())
        .await
        .expect("real Alpaca stocks bars_single_all request should succeed");
    assert_eq!(all.symbol, "AAPL");
    assert!(all.next_page_token.is_none());
    assert!(
        all.bars.len() >= first.bars.len(),
        "bars_single_all should collect at least the first page"
    );

    let pages = client
        .stocks()
        .bars_single_stream(request)
        .collect::<Vec<_>>()
        .await;
    assert!(
        !pages.is_empty(),
        "bars_single_stream should yield at least one page"
    );
    let stream_first = pages[0]
        .as_ref()
        .expect("first bars_single_stream page should succeed");
    assert_eq!(stream_first.symbol, "AAPL");
}

#[tokio::test]
async fn stocks_single_quotes_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let request = stocks::QuotesSingleRequest {
        symbol: "AAPL".into(),
        start: Some("2024-03-01T15:00:00Z".into()),
        end: Some("2024-03-01T15:00:05Z".into()),
        limit: Some(500),
        feed: Some(stocks::DataFeed::Iex),
        sort: Some(stocks::Sort::Asc),
        asof: None,
        currency: Some(stocks::Currency::from("USD")),
        page_token: None,
    };

    let first = client
        .stocks()
        .quotes_single(request.clone())
        .await
        .expect("real Alpaca stocks quotes_single request should succeed");
    assert_eq!(first.symbol, "AAPL");
    assert!(
        !first.quotes.is_empty(),
        "quotes_single response should include at least one quote"
    );
    assert!(
        first.quotes[0].t.is_some(),
        "decoded single quote timestamp should be populated"
    );

    let all = client
        .stocks()
        .quotes_single_all(request.clone())
        .await
        .expect("real Alpaca stocks quotes_single_all request should succeed");
    assert_eq!(all.symbol, "AAPL");
    assert!(all.next_page_token.is_none());
    assert!(
        all.quotes.len() >= first.quotes.len(),
        "quotes_single_all should collect at least the first page"
    );

    let pages = client
        .stocks()
        .quotes_single_stream(request)
        .collect::<Vec<_>>()
        .await;
    assert!(
        !pages.is_empty(),
        "quotes_single_stream should yield at least one page"
    );
    let stream_first = pages[0]
        .as_ref()
        .expect("first quotes_single_stream page should succeed");
    assert_eq!(stream_first.symbol, "AAPL");
}

#[tokio::test]
async fn stocks_single_trades_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let request = stocks::TradesSingleRequest {
        symbol: "AAPL".into(),
        start: Some("2024-03-01T15:00:00Z".into()),
        end: Some("2024-03-01T15:00:05Z".into()),
        limit: Some(500),
        feed: Some(stocks::DataFeed::Iex),
        sort: Some(stocks::Sort::Asc),
        asof: None,
        currency: Some(stocks::Currency::from("USD")),
        page_token: None,
    };

    let first = client
        .stocks()
        .trades_single(request.clone())
        .await
        .expect("real Alpaca stocks trades_single request should succeed");
    assert_eq!(first.symbol, "AAPL");
    assert!(
        !first.trades.is_empty(),
        "trades_single response should include at least one trade"
    );
    assert!(
        first.trades[0].t.is_some(),
        "decoded single trade timestamp should be populated"
    );

    let all = client
        .stocks()
        .trades_single_all(request.clone())
        .await
        .expect("real Alpaca stocks trades_single_all request should succeed");
    assert_eq!(all.symbol, "AAPL");
    assert!(all.next_page_token.is_none());
    assert!(
        all.trades.len() >= first.trades.len(),
        "trades_single_all should collect at least the first page"
    );

    let pages = client
        .stocks()
        .trades_single_stream(request)
        .collect::<Vec<_>>()
        .await;
    assert!(
        !pages.is_empty(),
        "trades_single_stream should yield at least one page"
    );
    let stream_first = pages[0]
        .as_ref()
        .expect("first trades_single_stream page should succeed");
    assert_eq!(stream_first.symbol, "AAPL");
}
