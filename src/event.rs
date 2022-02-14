use std::time::Duration;

use crossterm::event::{poll, read, Event, EventStream, KeyCode, KeyEvent};
use crossterm::Result;

use crate::input::Input;

pub fn update_keys(estream: &mut EventStream, input: &mut Input) -> Result<()> {
    input.reset();

    let mut tst = EventStream::new();
    loop {
        // let mut event = estream.next().fuse();
        // let mut event = tst.next().fuse();
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = tst.next().fuse();
        match event {
            Some(Ok(ev)) => {
                println!("Event::{:?}\r", ev);

                if ev == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
            }
            Some(Err(e)) => println!("Error: {:?}\r", e),
            None => break,
        }
    }
    Ok(())
}

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
