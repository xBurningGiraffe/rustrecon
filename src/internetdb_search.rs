#![allow(unused)]
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
pub struct InternetDBSearchError {
    message: String,
}

impl InternetDBSearchError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for InternetDBSearchError {}

impl fmt::Display for InternetDBSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub async fn query_internetdb(target: &str) -> Result<String, Box<dyn Error>> {
    let url = if is_ip(target) {
        format!("https://internetdb.shodan.io/{}", target)
    } else {
        format!("https://internetdb.shodan.io/{}", target)
    };

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|err| InternetDBSearchError::new(err.to_string()))?;

    let response_body = response
        .text()
        .await
        .map_err(|err| InternetDBSearchError::new(err.to_string()))?;

    Ok(response_body)
}

pub async fn run_single_search_internetdb(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_ip(target) {
        let internetdb_result = query_internetdb(target).await?;
        let parsed_result = serde_json::from_str::<Value>(&internetdb_result)?;

        match output_file {
            Some(file_path) => {
                let mut file = File::create(file_path)?;
                write!(file, "{}", serde_json::to_string_pretty(&parsed_result)?)?;
            }
            None => {
                println!("InternetDB:");
                println!("{}", serde_json::to_string_pretty(&parsed_result)?);
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}