use std::error::Error;
use std::time::Duration;

use alpaca_data::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let reqwest_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let mut builder = Client::builder()
        .reqwest_client(reqwest_client)
        .max_retries(2)
        .retry_on_429(true)
        .respect_retry_after(true)
        .base_backoff(Duration::from_millis(100))
        .max_backoff(Duration::from_millis(500))
        .max_in_flight(32);

    if let Ok(api_key) = std::env::var("APCA_API_KEY_ID") {
        builder = builder.api_key(api_key);
    }

    if let Ok(secret_key) = std::env::var("APCA_API_SECRET_KEY") {
        builder = builder.secret_key(secret_key);
    }

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    let client = builder.build()?;

    let _ = client.stocks();
    let _ = client.options();
    let _ = client.crypto();
    let _ = client.news();
    let _ = client.corporate_actions();

    println!("client configured");
    Ok(())
}
