use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::{self, Value};
use std::env;
use std::fs::File;
use std::io::Write;

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
    let domain_regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();

    domain_regex.is_match(target)
}

pub async fn run_single_search_fullhunt(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_domain(target) {
        let fullhunt_result = query_fullhunt(target).await?;
        let parsed_result = serde_json::from_str::<Value>(&fullhunt_result)?;
        if let Some(output_file) = output_file {
            let mut file = File::create(output_file)?;
            writeln!(
                file,
                "FullHunt: \n{}",
                serde_json::to_string_pretty(&parsed_result)?
            )?;
        } else {
            println!(
                "FullHunt: \n{}",
                serde_json::to_string_pretty(&parsed_result)?
            );
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}
