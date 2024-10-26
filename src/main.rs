use chrono::{Local, Timelike, Datelike, TimeZone, DateTime};
use crossterm::{
    execute, terminal::{self, ClearType}, cursor::{self}, event::{self, Event, KeyCode},
};
use std::io::{self, Write};
use std::time::Duration;
use serde::Deserialize;
use colored::Colorize;

#[derive(Deserialize)]
struct Config {
    progress_char: String,
    refresh_rate_in_millis: u64,
}

struct ProgressBar {
    length: usize,
}

impl ProgressBar {
    fn new(percentage: f32) -> Self {
        let length = (percentage / 2.0).round() as usize; // Assuming a 50-character bar
        ProgressBar { length }
    }

    fn render(&self) -> String {
        format!("{}{}", self.get_repeat_path_for_length(), self.get_repeat_path_for_end())
    }

    fn get_repeat_path_for_end(&self) -> String {
        "░".repeat(50 - self.length)
    }

    fn get_repeat_path_for_length(&self) -> String {
        "█".repeat(self.length)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration
    let config_toml = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    let mut stdout = io::stdout();

    // Enable raw mode and enter alternate screen
    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All)
    )?;

    // Make sure to disable raw mode and leave alternate screen on panic or exit
    let result = run_app(&mut stdout, &config);

    // Restore terminal settings
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    result
}

fn run_app(stdout: &mut impl Write, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut previous_time = String::new();
    let mut previous_date = String::new();
    let mut previous_day_progress = String::new();
    let mut previous_week_progress = String::new();
    let mut previous_year_progress = String::new();

    loop {
        let mut buffer = String::new(); // Use a buffer to accumulate output

        // Move cursor to the top-left corner without clearing the screen
        buffer.push_str(&format!("{}", cursor::MoveTo(0, 0)));

        // Get current time and calculate progress percentages
        let now = Local::now();
        let weekday = now.weekday();

        // Calculate day progress
        let total_minutes_day = 24 * 60;
        let current_minutes_day = now.hour() * 60 + now.minute();
        let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;
        let day_progress_bar = ProgressBar::new(percentage_day);
        let day_process_text = format!(
            "Day Progress: [{}][{:02.0}%][{}]",
            weekday,
            percentage_day,
            day_progress_bar.render()
        );

        // Calculate week progress
        let total_minutes_week = 7 * 24 * 60;
        let current_day_of_week = now.weekday().num_days_from_sunday();
        let current_minutes_week = current_day_of_week * total_minutes_day + current_minutes_day;
        let percentage_week = (current_minutes_week as f32 / total_minutes_week as f32) * 100.0;
        let week_progress_bar = ProgressBar::new(percentage_week);
        let current_week = now.iso_week().week();
        let week_process_text = format!(
            "Week Progress: [W:{:02}][{:02.0}%][{}]",
            current_week,
            percentage_week,
            week_progress_bar.render()
        );

        // Calculate year progress
        let start_of_year = get_start_of_year(now);
        let duration_since_start_of_year = now.signed_duration_since(start_of_year);
        let total_minutes_year = 365 * total_minutes_day; // You might adjust for leap years
        let current_minutes_year = duration_since_start_of_year.num_minutes();
        let percentage_year = (current_minutes_year as f32 / total_minutes_year as f32) * 100.0;
        let year_progress_bar = ProgressBar::new(percentage_year);
        let year_process_text = format!(
            "Year Progress: [Y:{:04}][{:02.0}%][{}]",
            now.year(),
            percentage_year,
            year_progress_bar.render()
        );

        // Format time and date
        let time_text = format!("TIME: {:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let date_text = format!("DATE: {:02}/{:02}/{:04}", now.day(), now.month(), now.year());

        // Check if any of the displayed values have changed
        if time_text != previous_time || date_text != previous_date ||
            day_process_text != previous_day_progress || week_process_text != previous_week_progress ||
            year_process_text != previous_year_progress {

            // Accumulate output in the buffer
            buffer.push_str(&format!("{}\n", time_text.red().bold()));
            buffer.push_str(&format!("{}\n", date_text.blue().bold()));
            buffer.push_str(&format!("{}\n", day_process_text.green().bold()));
            buffer.push_str(&format!("{}\n", week_process_text.yellow().bold()));
            buffer.push_str(&format!("{}\n", year_process_text.magenta().bold()));

            // Write the buffer to the terminal
            write!(stdout, "{}", buffer)?;
            stdout.flush()?;

            // Update previous state
            previous_time = time_text;
            previous_date = date_text;
            previous_day_progress = day_process_text;
            previous_week_progress = week_process_text;
            previous_year_progress = year_process_text;
        }

        // Check for 'q' or 'Ctrl + C' key press to exit
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                if should_exit(&key_event) {
                    break;
                }
            }
        }

        // Sleep for the configured refresh rate
        std::thread::sleep(Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}

fn should_exit(key_event: &event::KeyEvent) -> bool {
    key_event.code == KeyCode::Char('q') ||
        (key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(event::KeyModifiers::CONTROL))
}

fn get_start_of_year(now: DateTime<Local>) -> DateTime<Local> {
    Local.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap()
}
