use alpaca_data::{Client, Error, corporate_actions, crypto, news, options, stocks};

fn auth_client() -> Client {
    Client::builder()
        .api_key("test-key")
        .secret_key("test-secret")
        .base_url("http://127.0.0.1:9")
        .build()
        .expect("test client should build")
}

fn crypto_client() -> Client {
    Client::builder()
        .base_url("http://127.0.0.1:9")
        .build()
        .expect("test client should build")
}

#[tokio::test]
async fn stocks_batch_requests_reject_empty_symbols() {
    let error = auth_client()
        .stocks()
        .bars(stocks::BarsRequest {
            symbols: Vec::new(),
            timeframe: stocks::TimeFrame::from("1Min"),
            ..stocks::BarsRequest::default()
        })
        .await
        .expect_err("empty stock symbols must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message) if message.contains("symbols") && message.contains("empty")
    ));
}

#[tokio::test]
async fn stocks_single_latest_bar_rejects_empty_symbol() {
    let error = auth_client()
        .stocks()
        .latest_bar(stocks::LatestBarRequest {
            symbol: String::new(),
            ..stocks::LatestBarRequest::default()
        })
        .await
        .expect_err("empty stock symbol must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message) if message.contains("symbol") && message.contains("invalid")
    ));
}

#[tokio::test]
async fn stocks_snapshot_rejects_whitespace_only_symbol() {
    let error = auth_client()
        .stocks()
        .snapshot(stocks::SnapshotRequest {
            symbol: "   ".into(),
            ..stocks::SnapshotRequest::default()
        })
        .await
        .expect_err("blank stock symbol must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message) if message.contains("symbol") && message.contains("invalid")
    ));
}

#[tokio::test]
async fn options_requests_reject_symbol_lists_over_one_hundred() {
    let symbols = (0..101)
        .map(|index| format!("AAPL260406C{:08}", index))
        .collect();

    let error = auth_client()
        .options()
        .bars(options::BarsRequest {
            symbols,
            timeframe: options::TimeFrame::from("1Day"),
            ..options::BarsRequest::default()
        })
        .await
        .expect_err("oversized option symbol lists must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message)
            if message.contains("symbols") && message.contains("100")
    ));
}

#[tokio::test]
async fn options_chain_rejects_empty_underlying_symbol() {
    let error = auth_client()
        .options()
        .chain(options::ChainRequest {
            underlying_symbol: String::new(),
            ..options::ChainRequest::default()
        })
        .await
        .expect_err("empty underlying symbol must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message) if message.contains("symbol") && message.contains("invalid")
    ));
}

#[tokio::test]
async fn crypto_requests_reject_empty_symbols() {
    let error = crypto_client()
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: Vec::new(),
            loc: None,
        })
        .await
        .expect_err("empty crypto symbols must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message) if message.contains("symbols") && message.contains("empty")
    ));
}

#[tokio::test]
async fn news_requests_reject_limits_outside_documented_range() {
    let error = auth_client()
        .news()
        .list(news::ListRequest {
            limit: Some(51),
            ..news::ListRequest::default()
        })
        .await
        .expect_err("out-of-range news limit must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message)
            if message.contains("limit") && message.contains("50")
    ));
}

#[tokio::test]
async fn corporate_actions_ids_reject_other_filters() {
    let error = auth_client()
        .corporate_actions()
        .list(corporate_actions::ListRequest {
            symbols: Some(vec!["AAPL".into()]),
            ids: Some(vec!["ca-1".into()]),
            ..corporate_actions::ListRequest::default()
        })
        .await
        .expect_err("ids plus other filters must fail before transport");

    assert!(matches!(
        error,
        Error::InvalidRequest(message)
            if message.contains("ids") && message.contains("filters")
    ));
}
