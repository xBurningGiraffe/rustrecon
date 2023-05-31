use reqwest::header::{HeaderValue, AUTHORIZATION};



use std::env;
use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use base64;

#[derive(Debug)]
pub struct CensysSearchError {
    message: String,
}

impl CensysSearchError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for CensysSearchError {}

impl fmt::Display for CensysSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn is_ip(target: &str) -> bool {
    target.parse::<IpAddr>().is_ok()
}

pub async fn query_censys(ip: &str) -> Result<String, Box<dyn Error>> {
    let censys_id = get_censys_id().expect("CENSYS_ID not found");
    let censys_secret = get_censys_secret().expect("CENSYS_SECRET not found");

    let url = format!("https://search.censys.io/api/v2/hosts/search?q={}", ip);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    let auth_value = format!("{}:{}", censys_id, censys_secret);
    let encoded_auth = base64::encode(auth_value);
    let auth_header_value = format!("Basic {}", encoded_auth);
    let auth_header = HeaderValue::from_str(&auth_header_value)
        .map_err(|err| CensysSearchError::new(err.to_string()))?;
    headers.insert(AUTHORIZATION, auth_header);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|err| CensysSearchError::new(err.to_string()))?;

    let response_body = response
        .text()
        .await
        .map_err(|err| CensysSearchError::new(err.to_string()))?;

    Ok(response_body)
}

pub fn get_censys_id() -> Option<String> {
    env::var("CENSYS_ID").ok()
}

pub fn get_censys_secret() -> Option<String> {
    env::var("CENSYS_SECRET").ok()
}
