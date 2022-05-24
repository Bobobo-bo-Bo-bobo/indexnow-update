use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use url::Url;

#[derive(Serialize, Clone, Deserialize)]
pub struct Configuration {
    pub database: String,
    pub file_extensions: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub base_url: String,
    pub engines: HashMap<String, EngineConfiguration>,
    #[serde(skip)]
    pub host: String,
}

#[derive(Serialize, Clone, Deserialize)]
pub struct EngineConfiguration {
    pub indexnow_url: String,
    pub key_value: String,
    pub key_url: String,
}

impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Configuration")
            .field("database", &self.database)
            .field("exclude", &self.exclude)
            .field("file_extensions", &self.file_extensions)
            .field("base_url", &self.base_url)
            .field("host", &self.host)
            .field("engines", &self.engines)
            .finish()
    }
}

impl std::fmt::Debug for EngineConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EngineConfiguration")
            .field("indexnow_url", &self.indexnow_url)
            .field("key_value", &self.key_value)
            .field("key_url", &self.key_url)
            .finish()
    }
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let mut result: Configuration = match serde_yaml::from_str(&raw.as_str()) {
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
