use chrono::{Datelike, Local, TimeZone, Timelike};
use colored::Colorize;
use crossterm::cursor;
use std::io::Write;

pub enum View {
    Main,
    TimeLimit,
}
pub struct AppState {
    pub current_view: View,
    pub show_remaining: bool, // Toggle state for showing remaining time
}

pub fn render_view(stdout: &mut impl Write, app_state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    buffer.push_str(&format!("{}", cursor::MoveTo(0, 0)));

    match app_state.current_view {
        View::Main => render_main_view(&mut buffer, app_state.show_remaining),
        View::TimeLimit => render_time_limit_view(&mut buffer),
    }

    // Flush once after all updates
    write!(stdout, "{}", buffer)?;
    stdout.flush()?;
    Ok(())
}


fn render_main_view(buffer: &mut String, show_remaining: bool) {
    let now = Local::now();
    let weekday = now.weekday();

    // Calculate day progress
    let total_minutes_day = 24 * 60;
    let current_minutes_day = now.hour() * 60 + now.minute();
    let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;
    let remaining_day = 100.0 - percentage_day;
    let day_progress_bar = ProgressBar::new(if show_remaining { remaining_day } else { percentage_day });
    let day_process_text = format!(
        "Day Progress: [{}][{:02.0}%][{}]",
        weekday,
        if show_remaining { remaining_day } else { percentage_day },
        day_progress_bar.render()
    );

    // Calculate week progress
    let total_minutes_week = 7 * 24 * 60;
    let current_day_of_week = now.weekday().num_days_from_sunday();
    let current_minutes_week = current_day_of_week * total_minutes_day + current_minutes_day;
    let percentage_week = (current_minutes_week as f32 / total_minutes_week as f32) * 100.0;
    let remaining_week = 100.0 - percentage_week;
    let week_progress_bar = ProgressBar::new(if show_remaining { remaining_week } else { percentage_week });
    let current_week = now.iso_week().week();
    let week_process_text = format!(
        "Week Progress: [W:{:02}][{:02.0}%][{}]",
        current_week,
        if show_remaining { remaining_week } else { percentage_week },
        week_progress_bar.render()
    );

    // Calculate year progress
    let start_of_year = Local.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();
    let duration_since_start_of_year = now.signed_duration_since(start_of_year);
    let total_minutes_year = 365 * total_minutes_day; // You might adjust for leap years
    let current_minutes_year = duration_since_start_of_year.num_minutes();
    let percentage_year = (current_minutes_year as f32 / total_minutes_year as f32) * 100.0;
    let remaining_year = 100.0 - percentage_year;
    let year_progress_bar = ProgressBar::new(if show_remaining { remaining_year } else { percentage_year });
    let year_process_text = format!(
        "Year Progress: [Y:{:04}][{:02.0}%][{}]",
        now.year(),
        if show_remaining { remaining_year } else { percentage_year },
        year_progress_bar.render()
    );

    // Format time and date without seconds
    let time_text = format!("TIME: {:02}:{:02}", now.hour(), now.minute());
    let date_text = format!("DATE: {:02}/{:02}/{:04}", now.day(), now.month(), now.year());

    // Add all texts to the buffer
    buffer.push_str(&format!("{}\n", time_text.red().bold()));
    buffer.push_str(&format!("{}\n", date_text.blue().bold()));
    buffer.push_str(&format!("{}\n", day_process_text.green().bold()));
    buffer.push_str(&format!("{}\n", week_process_text.yellow().bold()));
    buffer.push_str(&format!("{}\n", year_process_text.magenta().bold()));
}


fn render_time_limit_view(buffer: &mut String) {
    buffer.push_str("Time Limit View\n");
    buffer.push_str("Here you can manage your time limits.\n");
}

struct ProgressBar {
    length: usize,
}

impl ProgressBar {
    fn new(percentage: f32) -> Self {
        let length = (percentage / 2.0).round() as usize;
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
