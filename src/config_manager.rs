use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub monitors: Vec<String>,
    pub pipelines: Vec<String>,
    pub base_url: String,
    pub brokers: Vec<String>,
    pub topic: String,
    pub request_sleep: u64
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn read_config(file_path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: AppConfig = serde_yaml::from_str(&content)?;

        Ok(config)
    }
}