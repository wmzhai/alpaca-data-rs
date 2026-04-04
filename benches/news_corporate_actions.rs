use alpaca_data::{Client, corporate_actions, news};
use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn bench_news_list_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1beta1/news"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"news":[{"id":1,"headline":"headline-1","author":"Author","created_at":"2026-04-04T00:00:00Z","updated_at":"2026-04-04T00:00:01Z","summary":"Summary","content":"","url":"https://example.com/1","images":[],"symbols":["AAPL"],"source":"benzinga"}],"next_page_token":null}"#,
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
    let news_client = client.news();
    let request = news::ListRequest {
        symbols: Some(vec!["AAPL".into()]),
        include_content: Some(false),
        ..Default::default()
    };

    c.bench_function("news/list_local", |b| {
        b.to_async(&runtime).iter(|| {
            let news_client = news_client.clone();
            let request = request.clone();

            async move {
                let _ = news_client
                    .list(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

fn bench_corporate_actions_list_local(c: &mut Criterion) {
    let runtime = Runtime::new().expect("runtime should build");
    let server = runtime.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/corporate-actions"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"corporate_actions":{"cash_dividends":[{"id":"ca-1","symbol":"AAPL","cusip":"037833100","rate":0.25,"special":false,"foreign":false,"process_date":"2026-04-04","ex_date":"2026-04-04","record_date":"2026-04-04","payable_date":"2026-04-05"}]},"next_page_token":null}"#,
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
    let corporate_actions_client = client.corporate_actions();
    let request = corporate_actions::ListRequest {
        types: Some(vec![corporate_actions::CorporateActionType::CashDividend]),
        limit: Some(1),
        ..Default::default()
    };

    c.bench_function("corporate_actions/list_local", |b| {
        b.to_async(&runtime).iter(|| {
            let corporate_actions_client = corporate_actions_client.clone();
            let request = request.clone();

            async move {
                let _ = corporate_actions_client
                    .list(request)
                    .await
                    .expect("request should succeed");
            }
        })
    });
}

criterion_group!(
    news_corporate_actions,
    bench_news_list_local,
    bench_corporate_actions_list_local
);
criterion_main!(news_corporate_actions);
