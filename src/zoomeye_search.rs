use std::env;
use std::net::IpAddr;
use reqwest::{Client, Error};

pub async fn query_zoom_eye(ip: &str) -> Result<String, Error> {
    let api_key = get_zoom_eye_api_key().expect("ZOOMEYE_API not found");

    let query = format!("ip:{}", ip);
    let url = format!("https://api.zoomeye.org/host/search?query={}", query);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("API-KEY", api_key)
        .send()
        .await?;

    let response_body = response.text().await?;

    Ok(response_body)
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub fn get_zoom_eye_api_key() -> Option<String> {
    env::var("ZOOMEYE_API").ok()
}
