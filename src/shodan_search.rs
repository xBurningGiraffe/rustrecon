use reqwest;
use std::env;
use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use regex::Regex;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct ShodanSearchError {
    message: String,
}

impl ShodanSearchError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for ShodanSearchError {}

impl fmt::Display for ShodanSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub fn is_domain(target: &str) -> bool {
    let domain_regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();

    domain_regex.is_match(target)
}

pub async fn query_shodan(target: &str) -> Result<String, Box<dyn Error>> {
    let shodan_api_key = get_shodan_api_key().expect("SHODAN_API not found");

    let url = if is_ip(target) {
        format!("https://api.shodan.io/shodan/host/{}?key={}", target, shodan_api_key)
    } else {
        format!("https://api.shodan.io/dns/domain/{}?key={}", target, shodan_api_key)
    };

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|err| ShodanSearchError::new(err.to_string()))?;

    let response_body = response
        .text()
        .await
        .map_err(|err| ShodanSearchError::new(err.to_string()))?;

    Ok(response_body)
}

pub fn get_shodan_api_key() -> Option<String> {
    env::var("SHODAN_API").ok()
}

pub async fn run_single_search_shodan(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_ip(target) || is_domain(target) {
        let shodan_result = query_shodan(target).await?;
        let parsed_result = serde_json::from_str::<Value>(&shodan_result)?;

        match output_file {
            Some(file_path) => {
                let mut file = File::create(file_path)?;
                write!(file, "{}", serde_json::to_string_pretty(&parsed_result)?)?;
            }
            None => {
                println!("Shodan:");
                println!("{}", serde_json::to_string_pretty(&parsed_result)?);
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

