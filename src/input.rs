use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::view::{AppState, View};

pub fn handle_input(app_state: &mut AppState) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(150))? {
        if let Event::Key(key_event) = event::read()? {
            // Only handle the key press event
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Tab => {
                        app_state.current_view = match app_state.current_view {
                            View::Main => View::TimeLimit,
                            View::TimeLimit => View::Main,
                        };
                        return Ok(true); // Force update
                    }
                    KeyCode::Right => {
                        // Implement logic for right arrow key
                    }
                    KeyCode::Left => {
                        // Implement logic for left arrow key
                    }
                    KeyCode::Char(' ') => {
                        app_state.show_remaining = !app_state.show_remaining; // Toggle the state
                        return Ok(true); // Force update
                    }
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('c') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => return Ok(true),
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}
