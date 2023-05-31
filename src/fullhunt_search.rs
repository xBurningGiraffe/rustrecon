use reqwest::header::HeaderMap;
use reqwest::{Client};
use serde_json::Value;
use std::env;
use regex::Regex;

pub async fn query_fullhunt(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("FULLHUNT_API").expect("FULLHUNT_API not found");

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("X-API-KEY", api_key.parse().unwrap());

    let url = format!("https://fullhunt.io/api/v1/domain/{}/details", domain);

    let response = client.get(&url).headers(headers).send().await?;

    let response_body = response.text().await?;

    let json_value: Value = serde_json::from_str(&response_body)?;
    let pretty_response = serde_json::to_string_pretty(&json_value)?;

    Ok(pretty_response)
}

pub fn is_domain(target: &str) -> bool {
    // Regular expression for domain validation
    let domain_regex = Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();
    
    domain_regex.is_match(target)
}
