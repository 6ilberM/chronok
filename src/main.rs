use chrono::{Local, Timelike, Datelike, Weekday};
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

fn queue_text(stdout: &mut std::io::Stdout, x: u16, y: u16, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    queue!(stdout, cursor::MoveTo(x, y))?;
    queue!(stdout, style::PrintStyledContent(text.white().on_black()))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_toml = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let mut stdout = io::stdout();

    // Enable non-blocking reads
    execute!(stdout, event::EnableMouseCapture)?;
    // Get initial terminal size
    let (_, terminal_height) = terminal::size()?;
    let text_x = 0;
    let text_y = terminal_height / 2;

    loop {
        let now = Local::now();
        let weekday = now.weekday();
        let total_minutes_day = 24 * 60;
        let current_minutes_day = now.hour() * 60 + now.minute();
        let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;
        let day_progress_bar = ProgressBar::new(config.progress_char.clone(), percentage_day);
        let day_process_text = format!(
            "Day Process: [{}][{:02.0}%][{}]",
            weekday,
            percentage_day,
            day_progress_bar.render()
        );

        let total_minutes_week = 7 * 24 * 60; // Total number of minutes in a week
        let current_day_of_week = now.weekday().num_days_from_sunday(); // Current day of the week (0 for Sunday, 1 for Monday, etc.)
        let current_minutes_week = current_day_of_week * total_minutes_day + current_minutes_day; // Current number of minutes in the week
        let percentage_week = (current_minutes_week as f32 / total_minutes_week as f32) * 100.0; // Progress of the week in percent
        let week_progress_bar = ProgressBar::new(config.progress_char.clone(), percentage_week);
        let week_process_text = format!(
            "Week Process: [W:{:02}][{:02.0}%][{}]",
            now.iso_week().week(),
            percentage_week,
            week_progress_bar.render()
        );

        let time_text = format!("TIME: {:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let date_text = format!("DATE: {:02}/{:02}/{:02}", now.day(), now.month(), now.year() % 100);

        queue_text(&mut stdout, text_x, text_y, &time_text)?;
        queue_text(&mut stdout, text_x, text_y + 2, &date_text)?;
        queue_text(&mut stdout, text_x, text_y + 4, &day_process_text)?;
        queue_text(&mut stdout, text_x, text_y + 6, &week_process_text)?;

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
