use crate::config;
use crate::constants;
use crate::payload;

use log::{debug, info};
use std::error::Error;
use std::net::ToSocketAddrs;
use std::time::Duration;

pub fn build_client(timeout_sec: u64) -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let timeout = Duration::from_secs(timeout_sec);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-Clacks-Overhead",
        reqwest::header::HeaderValue::from_static("GNU Terry Pratchett"),
    );
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let http_client_builder = reqwest::blocking::ClientBuilder::new()
        .user_agent(constants::generate_user_agent())
        .default_headers(headers)
        .timeout(timeout);

    let http_client = match http_client_builder.build() {
        Ok(v) => v,
        Err(e) => bail!("can't create HTTP client: {}", e),
    };

    Ok(http_client)
}

pub fn post(
    cfg: &config::Configuration,
    http_client: &mut reqwest::blocking::Client,
    url: &str,
    list: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    debug!("POST {}", &url);

    /*    let post_payload = IndexNowData{
        host: config.host,
        key:
    }
    */
    //    let payload = payload::
    //    let reply = http_client.post(url).
    Ok(())
}

/*
pub fn get(
    http_client: &mut reqwest::blocking::Client,
    url: &str,
) -> Result<String, Box<dyn Error>> {
    debug!("GET {}", &url);

    let response = http_client.get(url).send()?;
    if response.status() != reqwest::StatusCode::OK {
        bail!(
            "HTTP connection returned HTTP status code \"{}\" instead of \"200 OK\"",
            response.status()
        );
    }

    let reply = response.text()?;
    Ok(reply)
}
*/
