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
            limit: Some(2),
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
        !bars.bars.is_empty(),
        "bars response should include symbols"
    );

    let quotes = client
        .stocks()
        .quotes(stocks::QuotesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T14:30:00Z".into()),
            end: Some("2024-03-01T14:35:00Z".into()),
            limit: Some(2),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await
        .expect("real Alpaca stocks quotes request should succeed");
    assert!(
        !quotes.quotes.is_empty(),
        "quotes response should include symbols"
    );

    let trades = client
        .stocks()
        .trades(stocks::TradesRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            start: Some("2024-03-01T14:30:00Z".into()),
            end: Some("2024-03-01T14:35:00Z".into()),
            limit: Some(2),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await
        .expect("real Alpaca stocks trades request should succeed");
    assert!(
        !trades.trades.is_empty(),
        "trades response should include symbols"
    );
}
