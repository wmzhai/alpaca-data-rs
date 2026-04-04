use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_latest_endpoints_use_real_api_without_credentials() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    let client = builder.build().expect("client should build");

    let latest_bars = client
        .crypto()
        .latest_bars(crypto::LatestBarsRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("real Alpaca crypto latest bars request should succeed without credentials");
    assert!(latest_bars.bars.contains_key("BTC/USD"));

    let latest_quotes = client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us1),
        })
        .await
        .expect("real Alpaca crypto latest quotes request should succeed without credentials");
    assert!(latest_quotes.quotes.contains_key("BTC/USD"));

    let latest_trades = client
        .crypto()
        .latest_trades(crypto::LatestTradesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Eu1),
        })
        .await
        .expect("real Alpaca crypto latest trades request should succeed without credentials");
    assert!(latest_trades.trades.contains_key("BTC/USD"));

    let latest_orderbooks = client
        .crypto()
        .latest_orderbooks(crypto::LatestOrderbooksRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("real Alpaca crypto latest orderbooks request should succeed without credentials");
    assert!(latest_orderbooks.orderbooks.contains_key("BTC/USD"));
}
