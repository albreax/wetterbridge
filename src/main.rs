use chrono::{DateTime, Utc};
use config::Config;
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ApiConfig {
    username: String,
    password: String,
    coordinates: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Konfiguration laden

    let c = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();
    //settings.merge(config::File::with_name("config"))?;
    let api_config: ApiConfig = c.get("api").unwrap();
    // let parameters = "t_2m:C"; // Temperatur in Grad Celsius

    let now: DateTime<Utc> = Utc::now();
    let start_time = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let end_time = (now + chrono::Duration::days(1))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let interval = "PT1H"; // Stündlich
    let units = "t_2m:C"; // Temperatur in Grad Celsius
    let base_url = format!(
        "https://api.meteomatics.com/{}--{}:{}/{}/{}/json",
        start_time, end_time, interval, units, api_config.coordinates
    );
    println!("URL: {}", base_url);
    let client = reqwest::Client::new();
    let res = client
        .get(&base_url)
        .basic_auth(api_config.username, Some(api_config.password))
        .send()
        .await?;

    if res.status().is_success() {
        let data: Value = res.json().await?;
        println!("Wetterdaten: {}", data);
    } else {
        eprintln!("Fehler beim Abrufen der Wetterdaten: {}", res.status());
    }

    Ok(())
}
