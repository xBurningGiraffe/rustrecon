extern crate virustotal;
use virustotal::VtClient;
use std::env;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn is_ip(target: &str) -> bool {
    target.parse::<std::net::IpAddr>().is_ok()
}

pub fn is_domain(target: &str) -> bool {
    let domain_regex = regex::Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(target)
}

pub fn get_vt_api_key() -> Option<String> {
    env::var("VT_API").ok()
}

pub async fn run_single_search_virustotal(
    target: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let api_key = get_vt_api_key().expect("VT_API not found");
    let vt = VtClient::new(&api_key);

    let vt_result: String = if is_ip(target) {
        let ip_report = vt.report_ip_address(target);
        format!("IP Report: {:?}", ip_report)  // Replace with actual fields
    } else if is_domain(target) {
        let domain_report = vt.repot_domain(target);  // Corrected the typo
        format!("Domain Report: {:?}", domain_report)  // Replace with actual fields
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid target")));
    };
    

    match output_file {
        Some(file_path) => {
            let mut file = File::create(file_path)?;
            write!(file, "{}", vt_result)?;
        },
        None => {
            println!("VirusTotal:");
            println!("{}", vt_result);
        }
    }

    Ok(())
}
