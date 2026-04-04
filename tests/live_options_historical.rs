use alpaca_data::{Client, options};
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

async fn active_option_contract(client: &Client) -> (String, String, String) {
    let base_url = std::env::var("APCA_API_DATA_URL")
        .unwrap_or_else(|_| "https://data.alpaca.markets".to_string());
    let response = reqwest::Client::new()
        .get(format!(
            "{}/v1beta1/options/snapshots/AAPL",
            base_url.trim_end_matches('/')
        ))
        .header(
            "APCA-API-KEY-ID",
            std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"),
        )
        .header(
            "APCA-API-SECRET-KEY",
            std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"),
        )
        .query(&[("limit", "1")])
        .send()
        .await
        .expect("seed options chain request should succeed")
        .error_for_status()
        .expect("seed options chain response should be successful")
        .json::<serde_json::Value>()
        .await
        .expect("seed options chain response should decode");

    let (symbol, snapshot) = response["snapshots"]
        .as_object()
        .and_then(|items| items.iter().next())
        .expect("seed options chain should return at least one snapshot");
    let trade_time = snapshot["latestTrade"]["t"]
        .as_str()
        .or_else(|| snapshot["minuteBar"]["t"].as_str())
        .expect("seed snapshot should include latestTrade.t or minuteBar.t")
        .to_string();
    let start = format!("{}T00:00:00Z", &trade_time[..10]);
    let end = format!("{}T23:59:59Z", &trade_time[..10]);

    let _ = client;
    (symbol.to_string(), start, end)
}

#[tokio::test]
async fn options_historical_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let (symbol, start, end) = active_option_contract(&client).await;

    let bars = client
        .options()
        .bars(options::BarsRequest {
            symbols: vec![symbol.clone()],
            timeframe: options::TimeFrame::from("1Day"),
            start: Some(start.clone()),
            end: Some(end.clone()),
            limit: Some(10),
            sort: Some(options::Sort::Asc),
            page_token: None,
        })
        .await
        .expect("real Alpaca options bars request should succeed");
    assert!(
        bars.bars.contains_key(&symbol),
        "bars response should include the seeded symbol"
    );
    let bar = bars
        .bars
        .get(&symbol)
        .and_then(|items| items.first())
        .expect("bars response should include at least one bar");
    assert!(bar.t.is_some(), "decoded bar timestamp should be populated");
    assert!(bar.c.is_some(), "decoded bar close should be populated");

    let trades = client
        .options()
        .trades(options::TradesRequest {
            symbols: vec![symbol.clone()],
            start: Some(start.clone()),
            end: Some(end.clone()),
            limit: Some(10),
            sort: Some(options::Sort::Asc),
            page_token: None,
        })
        .await
        .expect("real Alpaca options trades request should succeed");
    assert!(
        trades.trades.contains_key(&symbol),
        "trades response should include the seeded symbol"
    );
    let trade = trades
        .trades
        .get(&symbol)
        .and_then(|items| items.first())
        .expect("trades response should include at least one trade");
    assert!(
        trade.t.is_some(),
        "decoded trade timestamp should be populated"
    );
    assert!(trade.p.is_some(), "decoded trade price should be populated");

    let bars_all = client
        .options()
        .bars_all(options::BarsRequest {
            symbols: vec![symbol.clone()],
            timeframe: options::TimeFrame::from("1Day"),
            start: Some(start.clone()),
            end: Some(end.clone()),
            limit: Some(1),
            sort: Some(options::Sort::Asc),
            page_token: None,
        })
        .await
        .expect("real Alpaca options bars_all request should succeed");
    assert_eq!(bars_all.next_page_token, None);
    assert!(
        bars_all.bars.contains_key(&symbol),
        "bars_all response should include the seeded symbol"
    );

    let trade_pages = client
        .options()
        .trades_stream(options::TradesRequest {
            symbols: vec![symbol.clone()],
            start: Some(start),
            end: Some(end),
            limit: Some(1),
            sort: Some(options::Sort::Asc),
            page_token: None,
        })
        .collect::<Vec<_>>()
        .await;
    assert!(
        !trade_pages.is_empty(),
        "trades_stream should yield at least one page"
    );
    assert!(trade_pages.iter().all(|page| page.is_ok()));
}
