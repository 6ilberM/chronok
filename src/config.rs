use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    // Remove unused field
    // pub progress_char: String,
    pub refresh_rate_in_millis: u64,
    pub timer_config_path: String,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
