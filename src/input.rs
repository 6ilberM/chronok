use crossterm::event::{self, Event, KeyCode};
use crate::view::View;

pub fn handle_input(current_view: &mut View) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(0))? {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => {
                    *current_view = match current_view {
                        View::Main => View::TimeLimit,
                        View::TimeLimit => View::Main,
                    };
                }
                KeyCode::Right => {
                    // Implement logic for right arrow key
                }
                KeyCode::Left => {
                    // Implement logic for left arrow key
                }
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('c') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => return Ok(true),
                _ => {}
            }
        }
    }
    Ok(false)
}
