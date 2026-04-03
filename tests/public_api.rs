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
    let _ = options::ChainRequest::default();
    let _ = options::ChainResponse::default();
    let _ = crypto::LatestOrderbooksRequest::default();
    let _ = crypto::LatestOrderbooksResponse::default();
    let _ = news::ListRequest::default();
    let _ = news::ListResponse::default();
    let _ = corporate_actions::ListRequest::default();
    let _ = corporate_actions::ListResponse::default();
}

#[test]
fn stocks_module_exposes_batch_and_single_type_names() {
    let _ = stocks::BarsRequest::default();
    let _ = stocks::BarsSingleRequest::default();
    let _ = stocks::LatestQuoteRequest::default();
    let _ = stocks::SnapshotRequest::default();
    let _ = stocks::ConditionCodesRequest::default();
    let _ = stocks::BarsSingleResponse::default();
    let _ = stocks::LatestBarResponse::default();
    let _ = stocks::SnapshotResponse::default();
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
    let _ = client.stocks().bars_all(stocks::BarsRequest::default());
    let _ = client.stocks().bars_stream(stocks::BarsRequest::default());
    let _ = client
        .stocks()
        .latest_quotes(stocks::LatestQuotesRequest::default());
    let _ = client
        .stocks()
        .latest_quote(stocks::LatestQuoteRequest::default());
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

    let _ = client.options().chain(options::ChainRequest::default());
    let _ = client.options().chain_all(options::ChainRequest::default());
    let _ = client
        .options()
        .chain_stream(options::ChainRequest::default());

    let _ = client
        .crypto()
        .latest_orderbooks(crypto::LatestOrderbooksRequest::default());
    let _ = client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest::default());
    let _ = client.news().list(news::ListRequest::default());
    let _ = client
        .corporate_actions()
        .list(corporate_actions::ListRequest::default());
}
