

use std::env;
use std::error::Error;
use regex::Regex;

pub fn is_domain(target: &str) -> bool {
    let domain_regex = Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(target)
}

pub async fn query_hunterio(domain: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("HUNTERIO_API").expect("HUNTERIO_API not found");

    let client = reqwest::Client::new();

    let url = format!("https://api.hunter.io/v2/domain-search?domain={}&api_key={}", domain, api_key);

    let response = client.get(&url).send().await?;
    let response_body = response.text().await?;

    Ok(response_body)
}
