use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::Result;

pub fn print_events() -> Result<()> {
    // `poll()` waits for an `Event` for a given time period
    if poll(Duration::from_millis(5000))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match read()? {
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    } else {
        // Timeout expired and no `Event` is available
    }
    Ok(())
}
