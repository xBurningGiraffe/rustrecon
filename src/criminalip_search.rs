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
pub struct CriminalIpSearchError {
    message: String,
}

impl CriminalIpSearchError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for CriminalIpSearchError {}

impl fmt::Display for CriminalIpSearchError {
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



pub async fn query_criminalip_domain_scan(target: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("CRIMINALIP_API").expect("CRIMINALIP_API not found");

    let url = format!("https://api.criminalip.io/v1/domain/lite/scan?query={}", target);

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("x-api-key", api_key.clone())
        .send()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    let response_body = response
        .text()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    let parsed_result: Value = serde_json::from_str(&response_body)?;
    let scan_id = parsed_result["data"]["scan_id"].as_str().unwrap_or("");

    let report_url = format!("https://api.criminalip.io/v1/domain/lite/report/{}", scan_id);

    let report_response = client
        .get(&report_url)
        .header("x-api-key", api_key)
        .send()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    let report_body = report_response
        .text()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    Ok(report_body)
}

pub async fn query_criminalip_ip_data(target: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("CRIMINALIP_API").expect("CRIMINALIP_API not found");

    let url = format!("https://api.criminalip.io/v1/ip/data?ip={}", target);

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("x-api-key", api_key.clone())
        .send()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    let response_body = response
        .text()
        .await
        .map_err(|err| CriminalIpSearchError::new(err.to_string()))?;

    Ok(response_body)
}

// Updated run_single_search_criminalip function
pub async fn run_single_search_criminalip(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_ip(target) || is_domain(target) {
        let criminalip_result = if is_domain(target) {
            query_criminalip_domain_scan(target).await?
        } else {
            query_criminalip_ip_data(target).await?
        };

        let parsed_result = serde_json::from_str::<Value>(&criminalip_result)?;

        match output_file {
            Some(file_path) => {
                let mut file = File::create(file_path)?;
                write!(file, "{}", serde_json::to_string_pretty(&parsed_result)?)?;
            }
            None => {
                println!("CriminalIP:");
                println!("{}", serde_json::to_string_pretty(&parsed_result)?);
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}
