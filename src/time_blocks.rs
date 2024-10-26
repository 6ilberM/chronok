use chrono::NaiveTime;
use serde::{Deserialize, Deserializer};
use std::fs;

#[derive(Deserialize)]
pub struct TimeBlockConfig {
    pub time_blocks: Vec<TimeBlock>,
}

#[derive(Deserialize)]
pub struct TimeBlock {
    pub name: String,
    #[serde(deserialize_with = "deserialize_naive_time")]
    pub start_time: NaiveTime,
    #[serde(deserialize_with = "deserialize_naive_time")]
    pub end_time: NaiveTime,
}

fn deserialize_naive_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveTime::parse_from_str(&s, "%H:%M").map_err(serde::de::Error::custom)
}

pub fn load_time_blocks(path: &str) -> Result<Vec<TimeBlock>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: TimeBlockConfig = toml::from_str(&contents)?;
    Ok(config.time_blocks)
}
