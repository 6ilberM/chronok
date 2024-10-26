use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEventKind};
use std::time::Duration;
use crate::view::{AppState, View};

pub fn handle_input(app_state: &mut AppState) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            // Check if the key event is a key press (not a repeat)
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => return Ok(true), // Quit on 'q'
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => return Ok(true), // Quit on Ctrl+C
                    KeyCode::Tab => {
                        app_state.current_view = match app_state.current_view {
                            View::Main => View::TimeLimit,
                            View::TimeLimit => View::Main,
                        };
                    }
                    KeyCode::Char(' ') => {
                        app_state.show_remaining = !app_state.show_remaining; // Toggle the state
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}
