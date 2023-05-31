use reqwest::{Client};
use std::env;
use regex::Regex;

pub fn is_domain(target: &str) -> bool {
    let domain_regex = Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(target)
}

pub async fn query_projectdiscovery(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("PROJECTDISCOVERY_API").expect("PROJECTDISCOVERY_API not found");

    let client = Client::new();

    let url = format!("https://dns.projectdiscovery.io/dns/{}/subdomains", domain);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    let response_body = response.text().await?;

    Ok(response_body)
}
