mod input;
mod config;
mod view;

use crossterm::{execute, terminal, cursor};
use std::io::{self, Write};
use std::time::Duration;
use chrono::{Local, Timelike};
use config::Config;
use view::{View, render_view};
use input::handle_input;
use crate::view::AppState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::load_config("config.toml")?;

    let mut stdout = io::stdout();

    // Enable raw mode and enter alternate screen
    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    // Run the application
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
    let mut app_state = AppState {
        current_view: View::Main,
        show_remaining: false,
    };

    let mut last_minute = Local::now().minute();

    // Render the initial state
    render_view(stdout, &app_state)?;

    loop {
        let now = Local::now();
        let current_minute = now.minute();

        // Check for input and update the display if necessary
        let input_action_occurred = handle_input(&mut app_state)?;

        // Update the display if the minute has changed or an input action occurred
        if current_minute != last_minute || input_action_occurred {
            render_view(stdout, &app_state)?;
            last_minute = current_minute;
        }

        // Sleep for a short duration to reduce CPU usage
        std::thread::sleep(Duration::from_millis(100));
    }
}



