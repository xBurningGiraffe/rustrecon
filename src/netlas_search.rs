use reqwest::{Client, Error};
use std::env;
use std::net::IpAddr;

pub fn is_domain(target: &str) -> bool {
    let domain_regex = regex::Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(target)
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub async fn query_netlas_domain(target: &str) -> Result<String, Error> {
    let api_key = env::var("NETLAS_API").expect("NETLAS_API_KEY not found");

    let client = Client::new();

    let url = format!("https://app.netlas.io/api/host/{}/?fields=*&source_type=include", target);

    let response = client
        .get(&url)
        .header("accept", "application/json")
        .header("X-API-Key", api_key)
        .send()
        .await?;

    Ok(response.text().await?)
}

pub async fn query_netlas_ip(target: &str) -> Result<String, Error> {
    let api_key = env::var("NETLAS_API").expect("NETLAS_API_KEY not found");

    let client = Client::new();

    let url = format!("https://app.netlas.io/api/host/{}/?fields=*&source_type=include", target);

    let response = client
        .get(&url)
        .header("accept", "application/json")
        .header("X-API-Key", api_key)
        .send()
        .await?;

    Ok(response.text().await?)



}
