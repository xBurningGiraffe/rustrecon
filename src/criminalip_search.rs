use reqwest::header::HeaderMap;
use reqwest::{Client, Error};
use std::env;
use regex::Regex;

pub fn is_ip(target: &str) -> bool {
    // Implement the logic to check if the target is a valid IP address
    // Return true if it is a valid IP, false otherwise
    let ip_regex = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
    ip_regex.is_match(target)
}

pub async fn query_criminalip(ip: &str) -> Result<String, Error> {
    let api_key = env::var("CRIMINALIP_API").expect("CRIMINALIP_API not found");

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("x-api-key", api_key.parse().unwrap());

    let url = format!("https://api.criminalip.io/v1/ip/data?ip={}", ip);

    let response = client.get(&url).headers(headers).send().await?;

    Ok(response.text().await?)
}