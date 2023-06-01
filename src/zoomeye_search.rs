use std::env;
use std::net::IpAddr;
use reqwest::{Client, Error};
use serde_json::Value;
use serde_json::json;
use std::fs::File;
use std::io::Write;

pub async fn query_zoom_eye(ip: &str) -> Result<String, Error> {
    let api_key = get_zoom_eye_api_key().expect("ZOOMEYE_API not found");

    let query = format!("ip:{}", ip);
    let url = format!("https://api.zoomeye.org/host/search?query={}", query);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("API-KEY", api_key)
        .send()
        .await?;

    let response_body = response.text().await?;

    Ok(response_body)
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub fn get_zoom_eye_api_key() -> Option<String> {
    env::var("ZOOMEYE_API").ok()
}

pub async fn run_single_search_zoomeye(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let zoomeye_result = query_zoom_eye(target).await?;

    let json: Value = serde_json::from_str(&zoomeye_result)?;

    if let Some(hits) = json["matches"].as_array() {
        let filtered_hits: Vec<serde_json::Map<String, Value>> = hits
            .iter()
            .filter_map(|hit| match hit {
                Value::Object(map) => {
                    if !contains_chinese(map) {
                        Some(map.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        let filtered_json = json!({
            "matches": filtered_hits,
        });

        parsed_result(
            "ZoomEye",
            &serde_json::to_string_pretty(&filtered_json)?,
            output_file,
        )?;
    } else {
        println!("ZoomEye:\nNo results found");
    }

    Ok(())
}

fn contains_chinese(hit: &serde_json::Map<String, Value>) -> bool {
    if let Some(zh_cn) = hit.get("zh-CN") {
        if let Some(text) = zh_cn.as_str() {
            !text.is_empty()
        } else {
            true
        }
    } else {
        false
    }
}

fn parsed_result(
    search_engine: &str,
    result: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    match output_file {
        Some(file) => {
            let mut file = File::create(file)?;
            writeln!(file, "{}:\n{}", search_engine, result)?;
        }
        None => {
            println!("{}:\n{}", search_engine, result);
        }
    }
    Ok(())
}
