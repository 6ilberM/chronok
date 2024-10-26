use std::io::Write;

use chrono::{Datelike, Local, TimeZone, Timelike, DateTime};
use colored::Colorize;
use crossterm::{cursor, terminal, ExecutableCommand};

use crate::timer::load_timer_config;
use crate::time_blocks::TimeBlock;
pub enum View {
    Main,
    TimeLimit,
    TimeBlocks,

}

pub struct AppState {
    pub current_view: View,
    pub show_remaining: bool,
}

pub fn render_view(stdout: &mut impl Write, app_state: &AppState, last_buffer: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    match app_state.current_view {
        View::Main => {
            render_main_view(&mut buffer, app_state.show_remaining);
            render_time_blocks_view(&mut buffer);
        }
        View::TimeLimit => render_time_limit_view(&mut buffer)?,
        View::TimeBlocks => render_time_blocks_view(&mut buffer)?,  // New view rendering
    }

    if buffer != *last_buffer {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        write!(stdout, "{}", buffer)?;
        stdout.flush()?;
        *last_buffer = buffer;
    }

    Ok(())
}

fn render_main_view(buffer: &mut String, show_remaining: bool) {
    let now = Local::now();

    render_time_and_date(buffer, &now);

    render_day_progress(buffer, &now, show_remaining);
    render_week_progress(buffer, &now, show_remaining);
    render_year_progress(buffer, &now, show_remaining);
}
fn render_time_and_date(buffer: &mut String, now: &chrono::DateTime<Local>) {
    let time_text = format!("TIME: {:02}:{:02}", now.hour(), now.minute());
    let date_text = format!("DATE: {:02}/{:02}/{:04}", now.day(), now.month(), now.year());

    buffer.push_str(&format!("{}\n", time_text.red().bold()));
    buffer.push_str(&format!("{}\n", date_text.blue().bold()));
}

fn render_time_blocks_view(buffer: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    buffer.push_str(&format!("{}\n", "Time Blocks".blue().bold()));
    buffer.push_str(&format!("{}\n", "═".repeat(50).blue()));

    let time_blocks = crate::time_blocks::load_time_blocks("time_blocks.toml")?;

    for block in time_blocks {
        let (percentage, time_left_text) = calculate_time_block_progress(&now, &block);
        let progress_bar = ProgressBar::new(percentage);
        let block_text = format!(
            "{}: [{:02.0}%][{}][{}]",
            block.name,
            percentage,
            time_left_text,
            progress_bar.render()
        );
        buffer.push_str(&format!("{}\n", block_text));
    }

    Ok(())
}

pub fn calculate_time_block_progress(now: &DateTime<Local>, block: &TimeBlock) -> (f32, String) {
    let current_time = now.time();
    let total_minutes = (block.end_time - block.start_time).num_minutes();
    let elapsed_minutes = (current_time - block.start_time).num_minutes();
    let remaining_minutes = total_minutes - elapsed_minutes;

    let percentage = (elapsed_minutes as f32 / total_minutes as f32) * 100.0;
    let time_left_text = format!("{}h {}m left", remaining_minutes / 60, remaining_minutes % 60);

    (percentage, time_left_text)
}

fn render_day_progress(buffer: &mut String, now: &chrono::DateTime<Local>, show_remaining: bool) {
    let weekday = now.weekday();
    let total_minutes_day = 24 * 60;
    let current_minutes_day = now.hour() * 60 + now.minute();
    let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;
    let remaining_day = 100.0 - percentage_day;
    let day_progress_bar = ProgressBar::new(if show_remaining { remaining_day } else { percentage_day });
    let day_process_text = if show_remaining {
        format!(
            "Day Missing: [{}][{:02.0}%][{}]",
            weekday,
            remaining_day,
            day_progress_bar.render()
        )
    } else {
        format!(
            "Day Progress: [{}][{:02.0}%][{}]",
            weekday,
            percentage_day,
            day_progress_bar.render()
        )
    };

    buffer.push_str(&format!("{}\n", day_process_text.green().bold()));
}

fn render_week_progress(buffer: &mut String, now: &chrono::DateTime<Local>, show_remaining: bool) {
    let total_minutes_day = 24 * 60;
    let total_minutes_week = 7 * total_minutes_day;
    let current_day_of_week = now.weekday().num_days_from_sunday();
    let current_minutes_week = current_day_of_week * total_minutes_day + now.hour() * 60 + now.minute();
    let percentage_week = (current_minutes_week as f32 / total_minutes_week as f32) * 100.0;
    let remaining_week = 100.0 - percentage_week;
    let week_progress_bar = ProgressBar::new(if show_remaining { remaining_week } else { percentage_week });
    let week_process_text = if show_remaining {
        format!(
            "Week Left: [W:{:02}][{:02.0}%][{}]",
            now.iso_week().week(),
            remaining_week,
            week_progress_bar.render()
        )
    } else {
        format!(
            "Week Progress: [W:{:02}][{:02.0}%][{}]",
            now.iso_week().week(),
            percentage_week,
            week_progress_bar.render()
        )
    };

    buffer.push_str(&format!("{}\n", week_process_text.yellow().bold()));
}

fn render_year_progress(buffer: &mut String, now: &chrono::DateTime<Local>, show_remaining: bool) {
    let start_of_year = Local.with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0).unwrap();
    let duration_since_start_of_year = now.signed_duration_since(start_of_year);
    let total_minutes_year = 365 * 24 * 60; // You might adjust for leap years
    let current_minutes_year = duration_since_start_of_year.num_minutes();
    let percentage_year = (current_minutes_year as f32 / total_minutes_year as f32) * 100.0;
    let remaining_year = 100.0 - percentage_year;
    let year_progress_bar = ProgressBar::new(if show_remaining { remaining_year } else { percentage_year });
    let year_process_text = if show_remaining {
        format!(
            "Year Left: [Y:{:04}][{:02.0}%][{}]",
            now.year(),
            remaining_year,
            year_progress_bar.render()
        )
    } else {
        format!(
            "Year Progress: [Y:{:04}][{:02.0}%][{}]",
            now.year(),
            percentage_year,
            year_progress_bar.render()
        )
    };

    buffer.push_str(&format!("{}\n", year_process_text.magenta().bold()));
}

pub fn render_time_limit_view(buffer: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let config = crate::config::load_config("config.toml")?;
    let timer_config = load_timer_config(&config.timer_config_path)?;

    let (active, completed): (Vec<_>, Vec<_>) = timer_config.timers
        .iter()
        .partition(|timer| timer.is_active(&now));

    buffer.push_str(&format!("{}\n", "Time Limits".blue().bold()));
    buffer.push_str(&format!("{}\n", "═".repeat(50).blue()));

    if active.is_empty() && completed.is_empty() {
        buffer.push_str(&format!("\n{}\n", "No timers configured.".yellow()));
        buffer.push_str("Add timers to timers.toml to get started.\n");
        return Ok(());
    }

    buffer.push_str(&format!("{}\n", "Active Timers:".green().bold()));
    for timer in &active {
        let progress = timer.progress(&now);
        let progress_bar = ProgressBar::new(progress);

        let timer_text = format!(
            "{}: {} - {}% [{}]",
            timer.name,
            timer.time,
            progress as u32,
            progress_bar.render()
        );
        buffer.push_str(&format!("{}\n", timer_text.yellow()));
        buffer.push_str(&format!("Message: {}\n", timer.message.white()));
    }

    if !completed.is_empty() {
        buffer.push_str(&format!("\n{}\n", "Completed Timers:".red().bold()));
        for timer in &completed {
            let timer_text = format!(
                "{}: {} - {}",
                timer.name,
                timer.time,
                timer.message
            );
            buffer.push_str(&format!("{}\n", timer_text.dimmed()));
        }
    }

    Ok(())
}

struct ProgressBar {
    length: usize,
    overflow: bool,
}

impl ProgressBar {
    fn new(percentage: f32) -> Self {
        let overflow = percentage > 100.0;
        // Clamp at 100% if overflowing
        let clamped_percentage = if overflow { 100.0 } else { percentage };
        let length = ((clamped_percentage / 2.0).round() as usize).min(50);
        ProgressBar { length, overflow }
    }

    fn render(&self) -> String {
        let filled = if self.overflow {
            "█".repeat(self.length).red()
        } else {
            "█".repeat(self.length).normal()
        };
        let empty = "░".repeat(50 - self.length);
        format!("{}{}", filled, empty)
    }
}
