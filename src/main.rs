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

    loop {
        // Render the current view
        render_view(stdout, &app_state)?;

        // Handle input and update the current view
        if handle_input(&mut app_state)? {
            break; // Exit the loop if quit command is detected
        }

        std::thread::sleep(Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}
