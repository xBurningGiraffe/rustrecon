use reqwest::{Client, Error};
use std::env;
use std::net::IpAddr;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

pub fn is_domain(target: &str) -> bool {
    let domain_regex = regex::Regex::new(r"^([a-zA-Z0-9]+(-[a-zA-Z0-9]+)*\.)+[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(target)
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

/* pub async fn query_netlas_domain(target: &str) -> Result<String, Error> {
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



}  */

pub async fn query_netlas(target: &str) -> Result<String, Error> {
    let api_key = env::var("NETLAS_API").expect("NETLAS_API not found");

    let client = Client::new();

    let url = format!("https://app.netlas.io/api/host/{}/?fields=*&source_type=include", target);

    let response = client
        .get(&url)
        .header("accept", "application/json")
        .header ("X-API-Key", api_key)
        .send()
        .await?;
    Ok(response.text().await?)


}

pub async fn run_single_search_netlas(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_ip(target) || is_domain(target) {
        let netlas_result = query_netlas(target).await?;
        let parsed_result = serde_json::from_str::<Value>(&netlas_result)?;

        match output_file {
            Some(file_path) => {
                let mut file = File::create(file_path)?;
                writeln!(file, "Netlas: \n{}", serde_json::to_string_pretty(&parsed_result)?)?;
            }
            None => {
                println!("Netlas:");
                println!("{}", serde_json::to_string_pretty(&parsed_result)?);
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}
