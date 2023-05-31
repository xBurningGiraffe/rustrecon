use std::env;
mod censys_search;
mod criminalip_search;
mod fullhunt_search;
mod helper;
mod hunterio_search;
mod netlas_search;
mod projectdiscovery_search;
mod zoomeye_search;
mod cisco_investigate_search;
use serde_json::{Value, Map};
use serde_json::json;
use std::net::IpAddr;

async fn run_all_searches(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(ip) = target.parse::<IpAddr>() {
        // IP target, include relevant search types
        let search_types = vec!["censys", "criminalip", "netlas", "zoomeye"];
        for search_type in search_types {
            match search_type {
                "censys" => run_single_search_censys(&ip.to_string()).await?,
                "criminalip" => run_single_search_criminalip(&ip.to_string()).await?,
                "netlas" => run_single_search_netlas_ip(&ip.to_string()).await?,
                "zoomeye" => run_single_search_zoomeye(&ip.to_string()).await?,
                _ => println!("Invalid search type: {}", search_type),
            }
        }
    } else if cisco_investigate_search::is_domain(target) {
        // Domain target, include relevant search types
        let search_types = vec![
            "cisco_investigate",
            "fullhunt",
            "projectdiscovery",
            "hunterio",
        ];
        for search_type in search_types {
            match search_type {
                "cisco_investigate" => run_single_search_cisco_investigate(target).await?,
                "fullhunt" => run_single_search_fullhunt(target).await?,
                "projectdiscovery" => run_single_search_projectdiscovery(target).await?,
                "hunterio" => run_single_search_hunterio(target).await?,
                _ => println!("Invalid search type: {}", search_type),
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        helper::print_help();
        return Ok(());
    }

    let mut target_option: Option<String> = None;
    let mut should_run_all_searches = false;

    for (index, arg) in args.iter().enumerate() {
        if arg == "-t" && index + 1 < args.len() {
            target_option = Some(args[index + 1].clone());
        }
        if arg == "-all" {
            should_run_all_searches = true;
        }
    }

    if let Some(target) = target_option {
        if should_run_all_searches {
            run_all_searches(&target).await?;
        } else {
            run_single_search(&target).await?;
        }
    } else {
        helper::print_help();
    }

    Ok(())
}

async fn run_single_search(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if censys_search::is_ip(target) {
        let censys_result = censys_search::query_censys(target).await?;
        let parsed_result: Value = serde_json::from_str(&censys_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("Censys:\n{}", formatted_result);
    } else if fullhunt_search::is_domain(target) {
        let fullhunt_result = fullhunt_search::query_fullhunt(target).await?;
        let parsed_result: Value = serde_json::from_str(&fullhunt_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("FullHunt:\n{}", formatted_result);
    } else if projectdiscovery_search::is_domain(target) {
        let projectdiscovery_result =
            projectdiscovery_search::query_projectdiscovery(target).await?;
        let parsed_result: Value = serde_json::from_str(&projectdiscovery_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("ProjectDiscovery:\n{}", formatted_result);
    } else if criminalip_search::is_ip(target) {
        let criminalip_result = criminalip_search::query_criminalip(target).await?;
        let parsed_result: Value = serde_json::from_str(&criminalip_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("CriminalIP:\n{}", formatted_result);
    } else if hunterio_search::is_domain(target) {
        let hunterio_result = hunterio_search::query_hunterio(target).await?;
        let parsed_result: Value = serde_json::from_str(&hunterio_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("HunterIO:\n{}", formatted_result);
    } else if netlas_search::is_domain(target) {
        let netlas_result = netlas_search::query_netlas_domain(target).await?;
        let parsed_result: Value = serde_json::from_str(&netlas_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("Netlas (Domain):\n{}", formatted_result);
    } else if netlas_search::is_ip(target) {
        let netlas_result = netlas_search::query_netlas_ip(target).await?;
        let parsed_result: Value = serde_json::from_str(&netlas_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("Netlas (IP):\n{}", formatted_result);
    } else if zoomeye_search::is_ip(target) {
        let zoomeye_result = zoomeye_search::query_zoom_eye(target).await?;
        let parsed_result: Value = serde_json::from_str(&zoomeye_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("ZoomEye:\n{}", formatted_result);
    } else if cisco_investigate_search::is_domain(target) {
        match cisco_investigate_search::query_cisco_investigate(target).await {
            Ok(cisco_investigate_result) => {
                println!("Cisco Investigate:\n{:?}", cisco_investigate_result);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }
    
    Ok(())
}


async fn run_single_search_censys(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if censys_search::is_ip(target) {
        let censys_result = censys_search::query_censys(target).await?;
        let parsed_result: Value = serde_json::from_str(&censys_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("Censys:\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_fullhunt(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fullhunt_search::is_domain(target) {
        let fullhunt_result = fullhunt_search::query_fullhunt(target).await?;
        let parsed_result: Value = serde_json::from_str(&fullhunt_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("FullHunt:\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_projectdiscovery(
    target: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if projectdiscovery_search::is_domain(target) {
        let projectdiscovery_result =
            projectdiscovery_search::query_projectdiscovery(target).await?;
        let parsed_result: Value = serde_json::from_str(&projectdiscovery_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("ProjectDiscovery:\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_criminalip(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if criminalip_search::is_ip(target) {
        let criminalip_result = criminalip_search::query_criminalip(target).await?;
        let parsed_result: Value = serde_json::from_str(&criminalip_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("CriminalIP:\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_hunterio(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if hunterio_search::is_domain(target) {
        let hunterio_result = hunterio_search::query_hunterio(target).await?;
        let parsed_result: Value = serde_json::from_str(&hunterio_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("HunterIO:\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_netlas_ip(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if netlas_search::is_ip(target) {
        let netlas_result = netlas_search::query_netlas_ip(target).await?;
        let parsed_result: Value = serde_json::from_str(&netlas_result)?;
        let formatted_result = serde_json::to_string_pretty(&parsed_result)?;
        println!("Netlas (IP):\n{}", formatted_result);
    } else {
        println!("Invalid target: {}", target);
    }
    Ok(())
}

async fn run_single_search_zoomeye(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zoomeye_result = zoomeye_search::query_zoom_eye(target).await?;

    let json: Value = serde_json::from_str(&zoomeye_result)?;

    if let Some(hits) = json["matches"].as_array() {
        let filtered_hits: Vec<Map<String, Value>> = hits
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

        let formatted_result = serde_json::to_string_pretty(&filtered_json)?;

        println!("ZoomEye:\n{}", formatted_result);
    } else {
        println!("ZoomEye:\nNo results found");
    }

    Ok(())
}

fn contains_chinese(hit: &Map<String, Value>) -> bool {
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

async fn run_single_search_cisco_investigate(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cisco_investigate_search::is_domain(target) {
        let cisco_investigate_result =
            cisco_investigate_search::query_cisco_investigate(target).await?;
        println!("Cisco Investigate:\n{:?}", cisco_investigate_result);
        Ok(()) // Return Ok(()) to indicate success
    } else {
        println!("Invalid target: {}", target);
        Ok(()) // Return Ok(()) to indicate success
    }
}




