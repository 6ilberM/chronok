mod input;
mod config;
mod view;
mod timer;

use crate::view::AppState;
use config::Config;
use crossterm::{cursor, execute, terminal};
use input::handle_input;
use std::io::{self, Write};
use std::time::Duration;
use view::{render_view, View};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::load_config("config.toml")?;

    let mut stdout = io::stdout();
    let mut last_buffer = String::new(); // Initialize the last buffer

    // Enable raw mode and enter alternate screen
    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    // Run the application
    let result = run_app(&mut stdout, &config, &mut last_buffer);

    // Restore terminal settings
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    result
}

fn run_app(stdout: &mut impl Write, config: &Config, last_buffer: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = AppState {
        current_view: View::Main,
        show_remaining: false,
    };

    loop {
        // Render the current view with double buffering
        render_view(stdout, &app_state, last_buffer)?;

        // Handle input and update the current view
        if handle_input(&mut app_state)? {
            break; // Exit the loop if quit command is detected
        }

        std::thread::sleep(Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}
