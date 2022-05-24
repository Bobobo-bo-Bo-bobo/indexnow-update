use serde::Deserialize;
use simple_error::bail;
use std::error::Error;
use std::fs;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub database: String,
    pub file_extensions: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub base_url: String,
    pub submit: String,
    pub key: String,
    pub key_location: Option<String>,
    #[serde(skip)]
    pub host: String,
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let mut result: Configuration = match serde_yaml::from_str(raw.as_str()) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    while result.base_url.ends_with('/') {
        result.base_url.pop();
    }
    result.base_url.push('/');

    let parsed = Url::parse(&result.base_url)?;
    result.host = match parsed.host_str() {
        Some(v) => v.to_string(),
        None => {
            bail!("Can't extract hostname from base_url {}", result.base_url);
        }
    };
    Ok(result)
}
