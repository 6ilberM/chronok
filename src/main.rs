mod input;
mod config;
mod view;

use crossterm::{execute, terminal, cursor};
use std::io::{self, Write};
use std::time::Duration;
use config::Config;
use view::{View, render_view};
use input::handle_input;

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
    let mut current_view = View::Main;

    loop {
        // Render the current view
        render_view(stdout, &current_view)?;

        // Handle input and update the current view
        if handle_input(&mut current_view)? {
            break;
        }

        // Sleep for the configured refresh rate
        std::thread::sleep(Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}
