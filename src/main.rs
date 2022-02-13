use crossterm::event::EventStream;
use crossterm::Result;
use std::io::{stdout, Stdout};

mod draw;
use draw::draw;

mod event;
use event::print_events;

mod input;
use input::Input;

fn main() -> Result<()> {
    let mut input = Input::default();
    let mut eventstream = EventStream::new();
    let mut stdout: Stdout = stdout();

    draw(&mut stdout)?;

    Ok(())
}
