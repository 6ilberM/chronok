mod input;
mod config;
mod view;
mod timer;
mod time_blocks;

use config::Config;
use crossterm::{cursor, execute, terminal};
use input::handle_input;
use std::io::{self, Write};
use std::time::Duration;
use view::{render_view, View};
use crate::time_blocks::TimeBlock;
use crate::view::AppState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::load_config("config.toml")?;
    let time_blocks = time_blocks::load_time_blocks("time_blocks.toml")?;

    let mut stdout = io::stdout();
    let mut last_buffer = String::new();

    // Enable raw mode and enter alternate screen
    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    // Run the application with time_blocks
    let result = run_app(&mut stdout, &config, &mut last_buffer, time_blocks);

    // Restore terminal settings
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    result
}

fn run_app(
    stdout: &mut impl Write,
    config: &Config,
    last_buffer: &mut String,
    time_blocks: Vec<TimeBlock>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_state = AppState {
        current_view: View::Main,
        show_remaining: false,
        time_blocks,  // Initialize with the loaded time blocks
    };

    loop {
        render_view(stdout, &app_state, last_buffer)?;

        if handle_input(&mut app_state)? {
            break;
        }

        std::thread::sleep(Duration::from_millis(config.refresh_rate_in_millis));
    }

    Ok(())
}
