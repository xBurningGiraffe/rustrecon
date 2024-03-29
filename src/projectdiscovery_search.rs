use reqwest::Client;
use std::env;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::process::Command;
use which::which;

// Function to check if the target is a valid domain
pub fn is_domain(target: &str) -> bool {
    let domain_regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();
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

// Function to perform a single search using ProjectDiscovery
pub async fn run_single_search_projectdiscovery(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_domain(target) {
        let check_chaos = Command::new("chaos")
            .arg("--help")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        if let Ok(_) = check_chaos {
            let api_key = env::var("PROJECTDISCOVERY_API").expect("PROJECTDISCOVERY_API not found");
            let chaos_output = Command::new("chaos")
                .arg("-d")
                .arg(target)
                .arg("-key")
                .arg(api_key)
                .output()?;

            let chaos_stdout = String::from_utf8_lossy(&chaos_output.stdout);
            println!("Chaos:\n{}", chaos_stdout);  // Print to terminal

            if let Some(output_file) = output_file {
                let mut file = File::create(output_file)?;
                writeln!(file, "Chaos:\n{}", chaos_stdout)?;
            }
        } else {
            let projectdiscovery_result = query_projectdiscovery(target).await?;
            let parsed_result = serde_json::from_str::<Value>(&projectdiscovery_result)?;

            if let Some(output_file) = output_file {
                let mut file = File::create(output_file)?;
                writeln!(file, "ProjectDiscovery:\n{}", serde_json::to_string_pretty(&parsed_result)?)?;
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}



