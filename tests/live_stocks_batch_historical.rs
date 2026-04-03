use alpaca_data::{Client, stocks};

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
async fn stocks_batch_historical_uses_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let bars = client
        .stocks()
        .bars(stocks::BarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            timeframe: stocks::TimeFrame::from("1Day"),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-06T00:00:00Z".into()),
            limit: Some(100),
            adjustment: Some(stocks::Adjustment::raw()),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await
        .expect("real Alpaca stocks bars request should succeed");
    assert!(
        bars.bars.contains_key("AAPL"),
        "bars response should include AAPL"
    );
    assert!(
        bars.bars.contains_key("MSFT"),
        "bars response should include MSFT"
    );
    let aapl_bar = bars
        .bars
        .get("AAPL")
        .and_then(|items| items.first())
        .expect("AAPL bars response should include at least one bar");
    assert!(
        aapl_bar.t.is_some(),
        "decoded bar timestamp should be populated"
    );
    assert!(
        aapl_bar.c.is_some(),
        "decoded bar close should be populated"
    );

    let quotes = client
        .stocks()
        .quotes(stocks::QuotesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T14:30:00Z".into()),
            end: Some("2024-03-01T14:31:00Z".into()),
            limit: Some(10_000),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await
        .expect("real Alpaca stocks quotes request should succeed");
    assert!(
        quotes.quotes.contains_key("AAPL"),
        "quotes response should include AAPL"
    );
    assert!(
        quotes.quotes.contains_key("MSFT"),
        "quotes response should include MSFT"
    );
    let aapl_quote = quotes
        .quotes
        .get("AAPL")
        .and_then(|items| items.first())
        .expect("AAPL quotes response should include at least one quote");
    assert!(
        aapl_quote.t.is_some(),
        "decoded quote timestamp should be populated"
    );
    assert!(
        aapl_quote.bp.is_some(),
        "decoded quote bid price should be populated"
    );

    let trades = client
        .stocks()
        .trades(stocks::TradesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T14:30:00Z".into()),
            end: Some("2024-03-01T14:31:00Z".into()),
            limit: Some(500),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await
        .expect("real Alpaca stocks trades request should succeed");
    assert!(
        trades.trades.contains_key("AAPL"),
        "trades response should include AAPL"
    );
    assert!(
        trades.trades.contains_key("MSFT"),
        "trades response should include MSFT"
    );
    let aapl_trade = trades
        .trades
        .get("AAPL")
        .and_then(|items| items.first())
        .expect("AAPL trades response should include at least one trade");
    assert!(
        aapl_trade.t.is_some(),
        "decoded trade timestamp should be populated"
    );
    assert!(
        aapl_trade.p.is_some(),
        "decoded trade price should be populated"
    );
}
