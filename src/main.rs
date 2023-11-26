use chrono::{Local, Timelike, Datelike};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, event::{self, Event, KeyCode},
};
use std::io::{self, Write};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    progress_char: String,
}

struct ProgressBar {
    progress_char: String,
    length: usize,
}

impl ProgressBar {
    fn new(progress_char: String, percentage: f32) -> Self {
        let length = (percentage / 2.0).round() as usize;
        ProgressBar { progress_char, length }
    }

    fn render(&self) -> String {
        format!("{}{}", self.progress_char.repeat(self.length), " ".repeat(50 - self.length))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_toml = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                queue!(stdout, cursor::MoveTo(x, y), style::PrintStyledContent("█".magenta()))?;
            }
        }
    }

    let now = Local::now();
    let weekday = now.weekday();
    let total_minutes_day = 24 * 60;
    let current_minutes_day = now.hour() * 60 + now.minute();
    let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;

    let day_progress_bar = ProgressBar::new(config.progress_char, percentage_day);
    let text = format!("\n\nTIME: {:02}:{:02}:{:02}\n\nDATE: {:02}/{:02}/{:02}\n\nWeek Process: [{:?}][{:02.0}%][{}]\n\n",
                       now.hour(), now.minute(), now.second(),
                       now.day(), now.month(), now.year() % 100,
                       weekday, percentage_day, day_progress_bar.render());

    queue!(stdout, cursor::MoveTo(2, 2), style::PrintStyledContent(text.as_str().blue().on_white()))?;
    stdout.flush()?;

    // Listen for events
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}