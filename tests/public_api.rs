use alpaca_data::{Client, corporate_actions, crypto, news, options, stocks};

#[test]
fn client_exposes_resource_accessors() {
    let client = Client::new();

    let _ = client.stocks();
    let _ = client.options();
    let _ = client.crypto();
    let _ = client.news();
    let _ = client.corporate_actions();
}

#[test]
fn client_builder_builds_client() {
    let client = Client::builder().build().expect("builder should succeed");

    let _ = client.stocks();
    let _ = client.options();
    let _ = client.crypto();
    let _ = client.news();
    let _ = client.corporate_actions();
}

#[test]
fn resource_modules_expose_short_type_names() {
    let _ = stocks::BarsRequest::default();
    let _ = stocks::BarsResponse::default();
    let _ = stocks::QuotesSingleRequest::default();
    let _ = stocks::QuotesSingleResponse::default();
    let _ = stocks::TradesSingleRequest::default();
    let _ = stocks::TradesSingleResponse::default();
    let _ = options::ChainRequest::default();
    let _ = options::ChainResponse::default();
    let _ = crypto::BarsRequest::default();
    let _ = crypto::BarsResponse::default();
    let _ = crypto::QuotesRequest::default();
    let _ = crypto::QuotesResponse::default();
    let _ = crypto::OrderbookLevel::default();
    let _ = crypto::TradesRequest::default();
    let _ = crypto::TradesResponse::default();
    let _ = crypto::LatestOrderbooksRequest::default();
    let _ = crypto::LatestOrderbooksResponse::default();
    let _ = crypto::SnapshotsResponse::default();
    let _ = news::ListRequest::default();
    let _ = news::ListResponse::default();
    let _ = news::NewsItem::default();
    let _ = news::NewsImage::default();
    let _ = corporate_actions::ListRequest::default();
    let _ = corporate_actions::ListResponse::default();
}

#[test]
fn stocks_module_exposes_batch_and_single_type_names() {
    let _ = stocks::BarsRequest::default();
    let _ = stocks::BarsSingleRequest::default();
    let _ = stocks::Tape::A;
    let _ = stocks::LatestBarRequest::default();
    let _ = stocks::LatestBarsRequest::default();
    let _ = stocks::LatestQuoteRequest::default();
    let _ = stocks::LatestQuotesRequest::default();
    let _ = stocks::LatestTradeRequest::default();
    let _ = stocks::LatestTradesRequest::default();
    let _ = stocks::QuotesSingleRequest::default();
    let _ = stocks::SnapshotRequest::default();
    let _ = stocks::SnapshotsRequest::default();
    let _ = stocks::TradesSingleRequest::default();
    let _ = stocks::ConditionCodesRequest::default();
    let _ = stocks::LatestBarResponse::default();
    let _ = stocks::LatestBarsResponse::default();
    let _ = stocks::LatestQuoteResponse::default();
    let _ = stocks::LatestQuotesResponse::default();
    let _ = stocks::LatestTradeResponse::default();
    let _ = stocks::LatestTradesResponse::default();
    let _ = stocks::BarsSingleResponse::default();
    let _ = stocks::QuotesSingleResponse::default();
    let _ = stocks::ConditionCodesResponse::default();
    let _ = stocks::ExchangeCodesResponse::default();
    let _ = stocks::SnapshotResponse::default();
    let _ = stocks::SnapshotsResponse::default();
    let _ = stocks::TradesSingleResponse::default();
}

#[test]
fn stocks_client_exposes_batch_and_single_method_names() {
    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .build()
        .expect("client should build");

    let _ = client.stocks().bars(stocks::BarsRequest::default());
    let _ = client
        .stocks()
        .bars_single(stocks::BarsSingleRequest::default());
    let _ = client
        .stocks()
        .bars_single_all(stocks::BarsSingleRequest::default());
    let _ = client
        .stocks()
        .bars_single_stream(stocks::BarsSingleRequest::default());
    let _ = client.stocks().bars_all(stocks::BarsRequest::default());
    let _ = client.stocks().bars_stream(stocks::BarsRequest::default());
    let _ = client
        .stocks()
        .quotes_single(stocks::QuotesSingleRequest::default());
    let _ = client
        .stocks()
        .quotes_single_all(stocks::QuotesSingleRequest::default());
    let _ = client
        .stocks()
        .quotes_single_stream(stocks::QuotesSingleRequest::default());
    let _ = client
        .stocks()
        .trades_single(stocks::TradesSingleRequest::default());
    let _ = client
        .stocks()
        .trades_single_all(stocks::TradesSingleRequest::default());
    let _ = client
        .stocks()
        .trades_single_stream(stocks::TradesSingleRequest::default());
    let _ = client
        .stocks()
        .latest_bars(stocks::LatestBarsRequest::default());
    let _ = client
        .stocks()
        .latest_bar(stocks::LatestBarRequest::default());
    let _ = client
        .stocks()
        .latest_quotes(stocks::LatestQuotesRequest::default());
    let _ = client
        .stocks()
        .latest_quote(stocks::LatestQuoteRequest::default());
    let _ = client
        .stocks()
        .latest_trades(stocks::LatestTradesRequest::default());
    let _ = client
        .stocks()
        .latest_trade(stocks::LatestTradeRequest::default());
    let _ = client
        .stocks()
        .snapshots(stocks::SnapshotsRequest::default());
    let _ = client.stocks().snapshot(stocks::SnapshotRequest::default());
    let _ = client
        .stocks()
        .condition_codes(stocks::ConditionCodesRequest::default());
    let _ = client.stocks().exchange_codes();
}

#[test]
fn resource_clients_expose_core_method_names() {
    let client = Client::new();

    let _ = client.options().bars(options::BarsRequest::default());
    let _ = client.options().bars_all(options::BarsRequest::default());
    let _ = client
        .options()
        .bars_stream(options::BarsRequest::default());
    let _ = client.options().trades(options::TradesRequest::default());
    let _ = client
        .options()
        .trades_all(options::TradesRequest::default());
    let _ = client
        .options()
        .trades_stream(options::TradesRequest::default());
    let _ = client
        .options()
        .latest_quotes(options::LatestQuotesRequest::default());
    let _ = client
        .options()
        .latest_trades(options::LatestTradesRequest::default());
    let _ = client.options().exchange_codes();
    let _ = client
        .options()
        .snapshots(options::SnapshotsRequest::default());
    let _ = client
        .options()
        .snapshots_all(options::SnapshotsRequest::default());
    let _ = client
        .options()
        .snapshots_stream(options::SnapshotsRequest::default());
    let _ = client.options().chain(options::ChainRequest::default());
    let _ = client.options().chain_all(options::ChainRequest::default());
    let _ = client
        .options()
        .chain_stream(options::ChainRequest::default());

    let _ = client.crypto().bars(crypto::BarsRequest::default());
    let _ = client.crypto().bars_all(crypto::BarsRequest::default());
    let _ = client.crypto().bars_stream(crypto::BarsRequest::default());
    let _ = client.crypto().quotes(crypto::QuotesRequest::default());
    let _ = client.crypto().quotes_all(crypto::QuotesRequest::default());
    let _ = client
        .crypto()
        .quotes_stream(crypto::QuotesRequest::default());
    let _ = client.crypto().trades(crypto::TradesRequest::default());
    let _ = client.crypto().trades_all(crypto::TradesRequest::default());
    let _ = client
        .crypto()
        .trades_stream(crypto::TradesRequest::default());
    let _ = client
        .crypto()
        .latest_bars(crypto::LatestBarsRequest::default());
    let _ = client
        .crypto()
        .latest_orderbooks(crypto::LatestOrderbooksRequest::default());
    let _ = client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest::default());
    let _ = client
        .crypto()
        .latest_trades(crypto::LatestTradesRequest::default());
    let _ = client
        .crypto()
        .snapshots(crypto::SnapshotsRequest::default());
    let _ = client.news().list(news::ListRequest::default());
    let _ = client.news().list_all(news::ListRequest::default());
    let _ = client.news().list_stream(news::ListRequest::default());
    let _ = client
        .corporate_actions()
        .list(corporate_actions::ListRequest::default());
}

#[test]
fn options_module_exposes_historical_type_names() {
    let _ = options::BarsRequest::default();
    let _ = options::TradesRequest::default();
    let _ = options::BarsResponse::default();
    let _ = options::TradesResponse::default();
    let _ = options::LatestQuotesRequest::default();
    let _ = options::LatestTradesRequest::default();
    let _ = options::LatestQuotesResponse::default();
    let _ = options::LatestTradesResponse::default();
    let _ = options::SnapshotsRequest::default();
    let _ = options::SnapshotsResponse::default();
    let _ = options::Greeks::default();
    let _ = options::ExchangeCodesResponse::default();
}
