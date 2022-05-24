use crate::config;
use crate::constants;
use crate::http;

use log::{debug, info};
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Clone, Debug)]
struct IndexNowData {
    pub host: String,
    pub key: String,
    #[serde(rename = "keyLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_location: Option<String>,
    #[serde(rename = "urlList")]
    pub url_list: Vec<String>,
}

fn build_post_payload(
    cfg: &config::Configuration,
    list: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let raw_payload = IndexNowData {
        host: cfg.host.clone(),
        key: cfg.key.clone(),
        key_location: cfg.key_location.clone(),
        url_list: list,
    };
    let payload = serde_json::to_string(&raw_payload)?;
    Ok(payload)
}

pub fn massage_payload(base_url: &str, html_dir: &str, list: Vec<String>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for entry in list {
        result.push(entry.replacen(html_dir, base_url, 1));
    }
    result
}

pub fn process_payload(
    cfg: config::Configuration,
    list: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    // The list of URLs per submit is limited to 10000 - https://www.indexnow.org/documentation
    let iter = list.len() / constants::BATCH_SIZE;
    let remain = list.len() % constants::BATCH_SIZE;
    debug!("List contains {} elements, {} iterations with fill batch size of {} + {} remaining elements", list.len(), iter, constants::BATCH_SIZE, remain);

    info!("Submitting data to {}", cfg.submit);
    if iter > 0 {
        // XXX
    }
    let payload = build_post_payload(&cfg, list[iter * constants::BATCH_SIZE..].to_vec()).unwrap();
    debug!("-> {}", payload);
    let mut http_client = http::build_client(constants::DEFAULT_TIMEOUT)?;
    http::post(&mut http_client, &cfg.submit, payload)?;
    Ok(())
}
