use chrono::{DateTime, Local, NaiveTime, Timelike};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct TimerConfig {
    pub timers: Vec<Timer>,
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Timer {
    pub name: String,
    pub time: String,
    pub message: String,
    pub repeat: String,
}

impl Timer {
    pub fn is_active(&self, current_time: &DateTime<Local>) -> bool {
        let timer_time = NaiveTime::parse_from_str(&self.time, "%H:%M").unwrap();
        let current_time = current_time.time();
        current_time < timer_time
    }

    pub fn progress(&self, current_time: &DateTime<Local>) -> f32 {
        let timer_time = NaiveTime::parse_from_str(&self.time, "%H:%M").unwrap();
        let current_minutes = current_time.time().hour() * 60 + current_time.time().minute();
        let target_minutes = timer_time.hour() * 60 + timer_time.minute();

        if current_minutes >= target_minutes {
            100.0
        } else {
            (current_minutes as f32 / target_minutes as f32) * 100.0
        }
    }
}

pub fn load_timer_config(path: &str) -> Result<TimerConfig, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err(format!("Timer config file not found: {}", path).into());
    }

    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read timer config from {}: {}", path, e))?;

    let config: TimerConfig = toml::from_str(&contents)
        .map_err(|e| format!("Failed to parse timer config: {}", e))?;

    if config.timers.is_empty() {
        return Err("No timers configured in the file".into());
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_timer_is_active() {
        let timer = Timer {
            name: "Test Timer".to_string(),
            time: "14:00".to_string(),
            message: "Test Message".to_string(),
            repeat: "daily".to_string(),
        };

        // Test before timer time
        let before_time = Local.with_ymd_and_hms(2024, 1, 1, 13, 0, 0).unwrap();
        assert!(timer.is_active(&before_time));

        // Test after timer time
        let after_time = Local.with_ymd_and_hms(2024, 1, 1, 15, 0, 0).unwrap();
        assert!(!timer.is_active(&after_time));
    }

    #[test]
    fn test_timer_progress() {
        let timer = Timer {
            name: "Test Timer".to_string(),
            time: "14:00".to_string(),
            message: "Test Message".to_string(),
            repeat: "daily".to_string(),
        };

        // Test at start of day
        let start_of_day = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(timer.progress(&start_of_day), 0.0);

        // Test at half way
        let half_way = Local.with_ymd_and_hms(2024, 1, 1, 7, 0, 0).unwrap();
        assert!(timer.progress(&half_way) > 45.0 && timer.progress(&half_way) < 55.0);

        // Test after completion
        let after_completion = Local.with_ymd_and_hms(2024, 1, 1, 15, 0, 0).unwrap();
        assert_eq!(timer.progress(&after_completion), 100.0);
    }
}
