use serde_json::Value;
use std::fs::File;
use std::io::Write;
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

pub async fn run_single_search_hunterio(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_domain(target) {
        let hunterio_result = query_hunterio(target).await?;
        let parsed_result = serde_json::from_str::<Value>(&hunterio_result)?;

        if let Some(output_file) = output_file {
            let mut file = File::create(output_file)?;
            writeln!(file, "HunterIO: \n{}", serde_json::to_string_pretty(&parsed_result)?)?;
        } else {
            println!("HunterIO: \n{}", serde_json::to_string_pretty(&parsed_result)?);
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}
