#![allow(unused)]
use clap::{App, Arg};
use std::io::Write;
use regex::Regex;
mod censys_search;
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
use netlas_search::run_single_search_netlas;
use zoomeye_search::run_single_search_zoomeye;
use internetdb_search::run_single_search_internetdb;

async fn run_all_searches(
    search_types: Vec<&str>,
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}$").unwrap();

    if let Ok(ip) = target.parse::<std::net::IpAddr>() {
        for search_type in &search_types {
            match *search_type {
                "shodan" => run_single_search_shodan(&ip.to_string(), output_file).await?,
                "censys" => run_single_search_censys(&ip.to_string(), output_file).await?,
                "criminalip" => run_single_search_criminalip(&ip.to_string(), output_file).await?,
                "netlas" => run_single_search_netlas(&ip.to_string(), output_file).await?,
                "zoomeye" => run_single_search_zoomeye(&ip.to_string(), output_file).await?,
                "internetdb" => run_single_search_internetdb(&ip.to_string(), output_file).await?,
                _ => println!("Invalid search type for IP: {}", search_type),
            }
        }
    } else if re.is_match(target) {
        for search_type in &search_types {
            match *search_type {
                "shodan" => run_single_search_shodan(target, output_file).await?,
                "censys" => run_single_search_censys(target, output_file).await?,
                "fullhunt" => run_single_search_fullhunt(target, output_file).await?,
                "projectdiscovery" => run_single_search_projectdiscovery(target, output_file).await?,
                "hunterio" => run_single_search_hunterio(target, output_file).await?,
                "netlas" => run_single_search_netlas(target, output_file).await?,
                _ => println!("Invalid search type for domain: {}", search_type),
            }
        }
    } else {
        println!("Invalid target: {}", target);
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    // Banner
    println!("
    ███████████                       █████    ███████████                                        
    ░░███░░░░░███                     ░░███    ░░███░░░░░███                                       
     ░███    ░███  █████ ████  █████  ███████   ░███    ░███   ██████   ██████   ██████  ████████  
     ░██████████  ░░███ ░███  ███░░  ░░░███░    ░██████████   ███░░███ ███░░███ ███░░███░░███░░███ 
     ░███░░░░░███  ░███ ░███ ░░█████   ░███     ░███░░░░░███ ░███████ ░███ ░░░ ░███ ░███ ░███ ░███ 
     ░███    ░███  ░███ ░███  ░░░░███  ░███ ███ ░███    ░███ ░███░░░  ░███  ███░███ ░███ ░███ ░███ 
     █████   █████ ░░████████ ██████   ░░█████  █████   █████░░██████ ░░██████ ░░██████  ████ █████
    ░░░░░   ░░░░░   ░░░░░░░░ ░░░░░░     ░░░░░  ░░░░░   ░░░░░  ░░░░░░   ░░░░░░   ░░░░░░  ░░░░ ░░░░░ 

    Created by xBurningGiraffe                                                                                   
    ");
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
                    "criminalip",
                    "hunterio",
                    "netlas",
                    "zoomeye",
                    "internetdb",
                ])
                .help("The type(s) of search, separated by commas")
                .takes_value(true)
                .multiple_occurrences(true)
                .use_delimiter(true)
                .value_delimiter(','),
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

    let search_types: Vec<&str> = matches.values_of("search_type").unwrap_or_default().collect();
    let target = matches.value_of("target").unwrap();
    let output_file = matches.value_of("output");

    if !search_types.is_empty() {
        if let Err(err) = run_all_searches(search_types, target, output_file).await {
            println!("Error while running specified searches: {}", err);
        }
    } else if matches.is_present("all") {
        let all_search_types = vec![
            "shodan",
            "censys",
            "fullhunt",
            "projectdiscovery",
            "criminalip",
            "hunterio",
            "netlas",
            "zoomeye",
            "internetdb",
        ];
        if let Err(err) = run_all_searches(all_search_types, target, output_file).await {
            println!("Error while running all searches: {}", err);
        }
    } else {
        println!("Please specify a search type or use --all to run all search types.");
    }
}
