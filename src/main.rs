use chrono::{Local, Timelike, Datelike};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, event::{self, Event, KeyCode},
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "█".magenta()))?;
            }
        }
    }

    let progress_char = "🌕"; // Change this to any character or emoji
    let now = Local::now();
    let weekday = now.weekday();
    let total_minutes_day = 24 * 60;
    let current_minutes_day = now.hour() * 60 + now.minute();
    let percentage_day = (current_minutes_day as f32 / total_minutes_day as f32) * 100.0;
    let progress_bar_length = (percentage_day / 2.0).round() as usize;
    let progress_bar = format!("{}{}", progress_char.repeat(progress_bar_length), " ".repeat(50 - progress_bar_length));
    let text = format!("Day Process: [{:?}][{:02.0}%][{}]", weekday, percentage_day, progress_bar);

    queue!(stdout, cursor::MoveTo(2, 2), style::PrintStyledContent(text.as_str().blue()))?;

    stdout.flush()?;

    // Listen for events
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}