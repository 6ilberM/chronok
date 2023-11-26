use chrono::{Local, Timelike, Datelike};
use crossterm::{execute, queue, style::{self, Stylize}, cursor, terminal, event::{self, Event, KeyCode}};
use std::io::{self, Write};
use serde::Deserialize;
use std::time::Duration;
use std::thread;

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

    // Enable non-blocking reads
    execute!(stdout, event::EnableMouseCapture)?;

    // Get initial terminal size
    let (terminal_width, terminal_height) = terminal::size()?;
    let text_x = terminal_width / 2;
    let text_y = terminal_height / 2;

    loop {
        let now = Local::now();
        let weekday = now.weekday();
        let total_minutes_day = 24 * 60;
        let current_minutes_day = now.hour() * 60 + now.minute();
        let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;

        let day_progress_bar = ProgressBar::new(config.progress_char.clone(), percentage_day);
        let week_number = now.iso_week().week();
        let time_text = format!("TIME: {:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let date_text = format!("DATE: {:02}/{:02}/{:02}", now.day(), now.month(), now.year() % 100);
        let week_process_text = format!(
            "Week Process: [{}][W:{:02}][{:02.0}%][{}]",
            weekday,
            week_number,
            percentage_day,
            day_progress_bar.render()
        );

        // Update only the lines that change
        queue!(stdout, cursor::MoveTo(text_x, text_y))?;
        queue!(stdout, style::PrintStyledContent(time_text.as_str().white().on_black()))?;

        queue!(stdout, cursor::MoveTo(text_x, text_y + 2))?;
        queue!(stdout, style::PrintStyledContent(date_text.as_str().white().on_black()))?;

        queue!(stdout, cursor::MoveTo(text_x, text_y + 4))?;
        queue!(stdout, style::PrintStyledContent(week_process_text.as_str().white().on_black()))?;

        stdout.flush()?;

        if event::poll(Duration::from_secs(0))? {
            if let Ok(Event::Key(event)) = event::read() {
                if event.code == KeyCode::Char('q') {
                    // Disable non-blocking reads
                    execute!(stdout, event::DisableMouseCapture)?;
                    break;
                }
            }
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}