use alpaca_data::{Client, options};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_options_chain_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1beta1/options/snapshots/AAPL"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"snapshots":{"AAPL260406C00265000":{"dailyBar":{"c":0.04,"h":0.12,"l":0.02,"n":211,"o":0.12,"t":"2026-04-02T04:00:00Z","v":971,"vw":0.045664},"greeks":{"delta":0.0232,"gamma":0.0118,"rho":0.0005,"theta":-0.043,"vega":0.0127},"impliedVolatility":0.2006,"latestQuote":{"ap":0.05,"as":209,"ax":"A","bp":0.03,"bs":265,"bx":"A","c":"A","t":"2026-04-02T19:59:59.991421537Z"},"latestTrade":{"c":"a","p":0.04,"s":1,"t":"2026-04-02T19:59:50.72338658Z","x":"I"},"minuteBar":{"c":0.04,"h":0.05,"l":0.04,"n":4,"o":0.05,"t":"2026-04-02T19:59:00Z","v":10,"vw":0.049},"prevDailyBar":{"c":0.15,"h":0.32,"l":0.08,"n":170,"o":0.22,"t":"2026-04-01T04:00:00Z","v":842,"vw":0.154917}}},"next_page_token":null}"#,
                "application/json",
            ))
            .mount(&server)
            .await;
        server
    });

    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url(server.uri())
        .build()
        .expect("client should build");
    let options_client = client.options();
    let request = options::ChainRequest {
        underlying_symbol: "AAPL".into(),
        feed: None,
        r#type: Some(options::ContractType::Call),
        strike_price_gte: Some(200.0),
        strike_price_lte: Some(230.0),
        expiration_date: Some("2026-04-06".into()),
        expiration_date_gte: None,
        expiration_date_lte: None,
        root_symbol: None,
        updated_since: None,
        limit: Some(3),
        page_token: None,
    };

    c.bench_function("options/chain_local", |b| {
        b.to_async(&runtime).iter(|| {
            let options_client = options_client.clone();
            let request = request.clone();

            async move {
                let _ = options_client
                    .chain(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(options, bench_options_chain_local);
criterion_main!(options);
