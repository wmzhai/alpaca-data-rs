use alpaca_data::{Client, crypto};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_crypto_latest_quotes_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1beta3/crypto/us/latest/quotes"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"quotes":{"BTC/USD":{"t":"2026-04-03T12:00:00Z","bp":1.0,"bs":1.0,"ap":1.1,"as":1.0}}}"#,
                "application/json",
            ))
            .mount(&server)
            .await;
        server
    });
    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");
    let crypto_client = client.crypto();
    let request = crypto::LatestQuotesRequest {
        symbols: vec!["BTC/USD".into()],
        loc: Some(crypto::Loc::Us),
    };

    c.bench_function("shared_core/crypto_latest_quotes_local", |b| {
        b.to_async(&runtime).iter(|| {
            let crypto_client = crypto_client.clone();
            let request = request.clone();

            async move {
                let _ = crypto_client
                    .latest_quotes(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(shared_core, bench_crypto_latest_quotes_local);
criterion_main!(shared_core);
