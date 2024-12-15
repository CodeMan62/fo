use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

#[derive(Deserialize)]
pub struct Config {
    pub categories: HashMap<String, Vec<String>>,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_data = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

