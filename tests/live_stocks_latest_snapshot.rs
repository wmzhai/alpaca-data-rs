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
async fn stocks_latest_and_snapshot_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let latest_bars = client
        .stocks()
        .latest_bars(stocks::LatestBarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_bars request should succeed");
    assert!(
        latest_bars.bars.contains_key("AAPL"),
        "latest_bars response should include AAPL"
    );
    assert!(
        latest_bars.bars.contains_key("MSFT"),
        "latest_bars response should include MSFT"
    );

    let latest_bar = client
        .stocks()
        .latest_bar(stocks::LatestBarRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_bar request should succeed");
    assert_eq!(latest_bar.symbol, "AAPL");
    assert!(
        latest_bar.bar.t.is_some(),
        "latest_bar response should decode a timestamp"
    );

    let latest_quotes = client
        .stocks()
        .latest_quotes(stocks::LatestQuotesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_quotes request should succeed");
    assert!(
        latest_quotes.quotes.contains_key("AAPL"),
        "latest_quotes response should include AAPL"
    );
    assert!(
        latest_quotes.quotes.contains_key("MSFT"),
        "latest_quotes response should include MSFT"
    );

    let latest_quote = client
        .stocks()
        .latest_quote(stocks::LatestQuoteRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_quote request should succeed");
    assert_eq!(latest_quote.symbol, "AAPL");
    assert!(
        latest_quote.quote.t.is_some(),
        "latest_quote response should decode a timestamp"
    );

    let latest_trades = client
        .stocks()
        .latest_trades(stocks::LatestTradesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_trades request should succeed");
    assert!(
        latest_trades.trades.contains_key("AAPL"),
        "latest_trades response should include AAPL"
    );
    assert!(
        latest_trades.trades.contains_key("MSFT"),
        "latest_trades response should include MSFT"
    );

    let latest_trade = client
        .stocks()
        .latest_trade(stocks::LatestTradeRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks latest_trade request should succeed");
    assert_eq!(latest_trade.symbol, "AAPL");
    assert!(
        latest_trade.trade.t.is_some(),
        "latest_trade response should decode a timestamp"
    );

    let snapshots = client
        .stocks()
        .snapshots(stocks::SnapshotsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks snapshots request should succeed");
    assert!(
        snapshots.snapshots.contains_key("AAPL"),
        "snapshots response should include AAPL"
    );
    assert!(
        snapshots.snapshots.contains_key("MSFT"),
        "snapshots response should include MSFT"
    );

    let snapshot = client
        .stocks()
        .snapshot(stocks::SnapshotRequest {
            symbol: "AAPL".into(),
            feed: Some(stocks::DataFeed::Iex),
            currency: Some(stocks::Currency::from("USD")),
        })
        .await
        .expect("real Alpaca stocks snapshot request should succeed");
    assert_eq!(snapshot.symbol, "AAPL");
    assert!(
        snapshot.snapshot.latestTrade.is_some()
            || snapshot.snapshot.latestQuote.is_some()
            || snapshot.snapshot.minuteBar.is_some()
            || snapshot.snapshot.dailyBar.is_some()
            || snapshot.snapshot.prevDailyBar.is_some(),
        "snapshot response should decode at least one official snapshot field"
    );
}
