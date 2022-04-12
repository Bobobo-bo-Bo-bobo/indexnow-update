use crate::config;
use crate::constants;
use log::{debug, info};
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Clone, Debug)]
pub struct Payload {
    pub host: String,
    pub key: String,
    #[serde(rename = "keyLocation")]
    pub key_location: String,
    #[serde(rename = "urlList")]
    pub url_list: Vec<String>,
}

fn build_post_payload(
    index_now_host: &str,
    index_now_key: &str,
    index_now_key_location: &str,
    list: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    let raw_payload = Payload {
        host: index_now_host.to_string(),
        key: index_now_key.to_string(),
        key_location: index_now_key_location.to_string(),
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

pub fn process_payload(cfg: config::Configuration, list: Vec<String>) {
    // The list of URLs per submit is limited to 10000 - https://www.indexnow.org/documentation
    let iter = list.len() / constants::BATCH_SIZE;
    let remain = list.len() % constants::BATCH_SIZE;
    debug!("List contains {} elements, {} iterations with fill batch size of {} + {} remaining elements", list.len(), iter, constants::BATCH_SIZE, remain);

    let engines = cfg.engines.clone();
    for (engine, engine_data) in engines {
        info!("Submitting data to {}", engine);
        if iter > 0 {
            // XXX
        }
        let payload = build_post_payload(
            &cfg.host,
            &engine_data.key_value,
            &engine_data.key_url,
            list[iter * constants::BATCH_SIZE..].to_vec(),
        )
        .unwrap();
        debug!("-> {}", payload);
    }
}
