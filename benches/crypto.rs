use alpaca_data::{Client, crypto};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_crypto_snapshots_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1beta3/crypto/us/snapshots"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"snapshots":{"BTC/USD":{"dailyBar":{"c":66800.79,"h":66975.1,"l":66772.66,"n":87,"o":66942.46,"t":"2026-04-04T00:00:00Z","v":0.029938953,"vw":66854.9651939408},"latestQuote":{"ap":66819.4,"as":1.28052,"bp":66763.431,"bs":1.272,"t":"2026-04-04T04:14:35.580241652Z"},"latestTrade":{"i":7456836641300628344,"p":66832.6,"s":0.000946,"t":"2026-04-04T04:14:32.161121311Z","tks":"B"},"minuteBar":{"c":66800.79,"h":66817.1675,"l":66800.79,"n":0,"o":66812.172,"t":"2026-04-04T04:13:00Z","v":0.0,"vw":66808.97875},"prevDailyBar":{"c":66961.45,"h":67293.2523,"l":66252.479,"n":549,"o":66887.805,"t":"2026-04-03T00:00:00Z","v":1.117036142,"vw":66779.3688392417}}}}"#,
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
    let request = crypto::SnapshotsRequest {
        symbols: vec!["BTC/USD".into()],
        loc: Some(crypto::Loc::Us),
    };

    c.bench_function("crypto/snapshots_local", |b| {
        b.to_async(&runtime).iter(|| {
            let crypto_client = crypto_client.clone();
            let request = request.clone();

            async move {
                let _ = crypto_client
                    .snapshots(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(crypto, bench_crypto_snapshots_local);
criterion_main!(crypto);
