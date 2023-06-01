#![allow(unused)]
use clap::{App, Arg};
use std::io::Write;
mod censys_search;
mod cisco_investigate_search;
mod criminalip_search;
mod fullhunt_search;
mod helper;
mod hunterio_search;
mod netlas_search;
mod projectdiscovery_search;
mod shodan_search;
mod zoomeye_search;
mod internetdb_search;
use shodan_search::run_single_search_shodan;
use censys_search::run_single_search_censys;
use fullhunt_search::run_single_search_fullhunt;
use hunterio_search::run_single_search_hunterio;
use projectdiscovery_search::run_single_search_projectdiscovery;
use criminalip_search::run_single_search_criminalip;
use netlas_search::run_single_search_netlas_ip;
use zoomeye_search::run_single_search_zoomeye;
use cisco_investigate_search::run_single_search_cisco_investigate;
use internetdb_search::run_single_search_internetdb;

async fn run_all_searches(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(ip) = target.parse::<std::net::IpAddr>() {
        // IP target, include relevant search types
        let search_types = vec!["shodan", "censys", "criminalip", "netlas", "zoomeye", "internetdb"];
        for search_type in search_types {
            match search_type {
                "shodan" => run_single_search_shodan(&ip.to_string(), output_file).await?,
                "censys" => run_single_search_censys(&ip.to_string(), output_file).await?,
                "criminalip" => run_single_search_criminalip(&ip.to_string(), output_file).await?,
                "netlas" => run_single_search_netlas_ip(&ip.to_string(), output_file).await?,
                "zoomeye" => run_single_search_zoomeye(&ip.to_string(), output_file).await?,
                _ => println!("Invalid search type: {}", search_type),
            }
        }
    } else if cisco_investigate_search::is_domain(target) {
        // Domain target, include relevant search types
        let search_types = vec![
            "shodan",
            "cisco_investigate",
            "fullhunt",
            "projectdiscovery",
            "hunterio",
        ];
        for search_type in search_types {
            match search_type {
                "shodan" => run_single_search_shodan(target, output_file).await?,
                "cisco_investigate" => {
                    run_single_search_cisco_investigate(target, output_file).await?
                }
                "fullhunt" => run_single_search_fullhunt(target, output_file).await?,
                "projectdiscovery" => {
                    run_single_search_projectdiscovery(target, output_file).await?
                }
                "hunterio" => run_single_search_hunterio(target, output_file).await?,
                _ => println!("Invalid search type: {}", search_type),
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = App::new("Rust Recon")
        .arg(
            Arg::new("search_type")
                .long("search_type")
                .value_name("SEARCH_TYPE")
                .possible_values(&[
                    "shodan",
                    "censys",
                    "fullhunt",
                    "projectdiscovery",
                    "investigate",
                    "criminalip",
                    "hunterio",
                    "netlas",
                    "zoomeye",
                    "internetdb",
                ])
                .help("The type of search")
                .takes_value(true),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .value_name("TARGET")
                .help("The target IP address or domain")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output the results to a file")
                .takes_value(true),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Run all applicable search types on the target"),
        )
        .get_matches();

    let search_type = matches.value_of("search_type");
    let target = matches.value_of("target").unwrap();
    let output_file = matches.value_of("output");

    if let Some(help) = matches.value_of("help") {
        if help == "help" {
            helper::print_help();
            return;
        }
    }

    if let Some(search_type) = search_type {
        match search_type {
            "shodan" => { 
                if let Err(err) = run_single_search_shodan(target, output_file).await {
                println!("Error while running Shodan search: {}", err);
            }
        }
            "censys" => {
                if let Err(err) = run_single_search_censys(target, output_file).await {
                    println!("Error while running Censys search: {}", err);
            }
        }
            "fullhunt" => {
                if let Err(err) = run_single_search_fullhunt(target, output_file).await {
                println!("Error while running FullHunt search: {}", err);
            }
        }
            "projectdiscovery" => {
                if let Err(err) = run_single_search_projectdiscovery(target, output_file).await {
                println!("Error while running ProjectDiscovery search: {}", err);
            }
        }
            "investigate" => {
                if let Err(err) = run_single_search_cisco_investigate(target, output_file).await {
                    println!("Error while running Cisco Investigate search: {}", err);
                }
            }
            "criminalip" => {
                if let Err(err) = run_single_search_criminalip(target, output_file).await {
                    println!("Error while running CriminalIP search: {}", err);
                }
            }
            "hunterio" => {
                if let Err(err) = run_single_search_hunterio(target, output_file).await {
                    println!("Error while running Hunter.io search: {}", err);
                }
            }
            "netlas" => {
                if let Err(err) = run_single_search_netlas_ip(target, output_file).await {
                        println!("Error while running Netlas search: {}", err);
                    }
                }
            "zoomeye" => {
                if let Err(err) = run_single_search_zoomeye(target, output_file).await {
                        println!("Error while running ZoomEye search: {}", err);
                    }
                }
            "internetdb" => {
                if let Err(err) = run_single_search_internetdb(target, output_file).await {
                        println!("Error while running InternetDB search: {}", err);
                }
            }
                _ => println!("Invalid search type: {}", search_type),
            } 
        } else { 
                if matches.is_present("all") {
                    if let Err(err) = run_all_searches(target, output_file).await {
                        println!("Error while running all searches: {}", err);
                    }
                } else {
                    println!("Invalid search type");
                }
            }
}


fn print_result(
    search_type: &str,
    result: &str,
    output_file: Option<&str>,
) -> Result<(), std::io::Error> {
    println!("{}:\n{}", search_type, result);

    if let Some(file_name) = output_file {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_name)?;
        writeln!(file, "{}:\n{}", search_type, result)?;
    }

    Ok(())
}
