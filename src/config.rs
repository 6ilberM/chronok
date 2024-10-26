use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub progress_char: String,
    pub refresh_rate_in_millis: u64,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_toml = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_toml)?;
    Ok(config)
}
