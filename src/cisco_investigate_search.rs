use reqwest::{header, Client};
use std::env;
use regex::Regex;

pub fn is_domain(target: &str) -> bool {
    // Regular expression for domain validation
    let domain_regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();

    domain_regex.is_match(target)
}

pub async fn query_cisco_investigate(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://investigate.api.umbrella.com/security/name/{}", domain);

    let access_token = get_cisco_investigate_access_token().expect("CISCO_INVESTIGATE_API not found");

    let client = Client::new();
    let response = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?; // Await the response

    let response_body = response.text().await?; // Await the response body

    Ok(response_body)
}

fn get_cisco_investigate_access_token() -> Option<String> {
    env::var("CISCO_INVESTIGATE_API").ok()
}
