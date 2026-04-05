use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use alpaca_data::{Client, ObservedResponseMeta, TransportObserver};

struct LoggingObserver;

impl TransportObserver for LoggingObserver {
    fn on_response(&self, meta: &ObservedResponseMeta) {
        println!(
            "observed response: endpoint={}, status={}, request_id={:?}",
            meta.endpoint_name, meta.status, meta.request_id
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let reqwest_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let builder = Client::builder()
        .reqwest_client(reqwest_client)
        .observer(Arc::new(LoggingObserver))
        .max_retries(2)
        .retry_on_429(true)
        .respect_retry_after(true)
        .base_backoff(Duration::from_millis(100))
        .max_backoff(Duration::from_millis(500))
        .max_in_flight(32)
        .credentials_from_env()?;

    let builder = if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder.base_url(base_url)
    } else {
        builder
    };

    let client = builder.build()?;

    let _ = client.stocks();
    let _ = client.options();
    let _ = client.crypto();
    let _ = client.news();
    let _ = client.corporate_actions();

    println!("client configured");
    Ok(())
}
