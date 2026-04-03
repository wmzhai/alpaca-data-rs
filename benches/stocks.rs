use alpaca_data::{Client, stocks};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_stocks_latest_quote_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/stocks/AAPL/quotes/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"symbol":"AAPL","quote":{"t":"2026-04-03T12:00:00Z","ax":"V","ap":200.1,"as":1,"bx":"V","bp":200.0,"bs":1},"currency":"USD"}"#,
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
    let stocks_client = client.stocks();
    let request = stocks::LatestQuoteRequest {
        symbol: "AAPL".into(),
        feed: None,
        currency: None,
    };

    c.bench_function("stocks/latest_quote_local", |b| {
        b.to_async(&runtime).iter(|| {
            let stocks_client = stocks_client.clone();
            let request = request.clone();

            async move {
                let _ = stocks_client
                    .latest_quote(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(stocks, bench_stocks_latest_quote_local);
criterion_main!(stocks);
