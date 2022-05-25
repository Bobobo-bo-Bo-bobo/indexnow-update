use crate::constants;

use log::debug;
use reqwest::StatusCode;
use simple_error::bail;
use std::error::Error;
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
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json; charset=utf-8"),
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
    http_client: &mut reqwest::blocking::Client,
    url: &str,
    data: String,
) -> Result<(), Box<dyn Error>> {
    debug!("Sending HTTP POST request to {}", &url);
    let response = http_client.post(url).body(data).send()?;
    match response.status() {
        StatusCode::OK | StatusCode::ACCEPTED => Ok(()),
        StatusCode::BAD_REQUEST => {
            bail!("invalid format");
        }
        StatusCode::FORBIDDEN => {
            bail!("key not valid, e.g. key not found, file found but key not in the file");
        }
        StatusCode::UNPROCESSABLE_ENTITY => {
            bail!("URLs which don’t belong to the host or the key is not matching the schema in the protocol");
        }
        StatusCode::TOO_MANY_REQUESTS => {
            bail!("too many requests (potential Spam)")
        }
        _ => {
            let reason = response.status().canonical_reason().unwrap_or("???");
            bail!("unexpected HTTP status {} {}", response.status(), reason);
        }
    }
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
