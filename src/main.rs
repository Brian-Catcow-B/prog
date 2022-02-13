use crossterm::event::EventStream;
use crossterm::Result;
use std::io::{stdout, Stdout};

mod draw;
use draw::draw;

mod event;
use event::print_events;

fn main() -> Result<()> {
    let mut eventstream = EventStream::new();
    let mut stdout: Stdout = stdout();

    draw(&mut stdout)?;
    print_events()?;

    Ok(())
}
