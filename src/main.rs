use chrono::{Local, Timelike, Datelike, TimeZone, DateTime};
use crossterm::{execute, terminal, event::{self, Event, KeyCode}};
use std::io::{self, stdout, Write};
use serde::Deserialize;
use std::time::Duration;
use colored::Colorize;

#[derive(Deserialize)]
struct Config {
    progress_char: String,
    refresh_rate_in_millis: u64,
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

fn queue_text(buffer: &mut String, x: u16, y: u16, text: &str, color: colored::Color) {
    buffer.push_str(&format!("\x1B[{};{}H", y + 1, x + 1)); // Move cursor to position
    buffer.push_str(&format!("{}", text.color(color))); // Add colored text
}

// Move the cursor up `n` lines
fn move_cursor_up(n: u16) {
    print!("\x1B[{}A", n);
    stdout().flush().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_toml = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let mut stdout = io::stdout();

    // Enable non-blocking reads
    execute!(stdout, event::EnableMouseCapture)?;
    // Get initial terminal size
    let (_, _terminal_height) = terminal::size()?;
    let text_x = 0;
    let position = crossterm::cursor::position()?;
    let text_y = position.1 as u16 + 1;

    let mut buffer = String::new(); // Buffer to hold terminal content

    loop {
        buffer.clear(); // Clear the buffer at the start of each loop iteration

        let now = Local::now();
        let weekday = now.weekday();
        let total_minutes_day = 24 * 60;
        let current_minutes_day = now.hour() * 60 + now.minute();
        let start_of_year = get_start_of_year(now);
        let duration_since_start_of_year = now.signed_duration_since(start_of_year);
        let current_week = (now - start_of_year).num_weeks() + 1;
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
            current_week,
            percentage_week,
            week_progress_bar.render()
        );

        // Year progress
        let total_minutes_year = 365 * total_minutes_day; // assuming non-leap year for simplicity
        let current_minutes_year = duration_since_start_of_year.num_minutes();
        let percentage_year = (current_minutes_year as f32 / total_minutes_year as f32) * 100.0;
        let year_progress_bar = ProgressBar::new(config.progress_char.clone(), percentage_year);
        let year_process_text = format!(
            "Year Process: [Y:{:04}][{:02.0}%][{}]",
            now.year(),
            percentage_year,
            year_progress_bar.render()
        );

        let time_text = format!("TIME: {:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let date_text = format!("DATE: {:02}/{:02}/{:02}", now.day(), now.month(), now.year() % 100);

        // Replace all queue_text calls with the updated function
        queue_text(&mut buffer, text_x, text_y, &time_text, colored::Color::Red);
        queue_text(&mut buffer, text_x, text_y + 1, &date_text, colored::Color::Blue);
        queue_text(&mut buffer, text_x, text_y + 2, &day_process_text, colored::Color::Green);
        queue_text(&mut buffer, text_x, text_y + 3, &week_process_text, colored::Color::Yellow);
        queue_text(&mut buffer, text_x, text_y + 4, &year_process_text, colored::Color::Magenta);

        // Print the buffer content
        print!("{}", buffer);

        if event::poll(Duration::from_secs(0))? {
            if let Ok(Event::Key(event)) = event::read() {
                if event.code == KeyCode::Char('q') {
                    // Disable non-blocking reads
                    execute!(stdout, event::DisableMouseCapture)?;
                    break;
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}

fn get_start_of_year(now: DateTime<Local>) -> DateTime<Local> {
    let start_of_year = Local.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();
    start_of_year
}