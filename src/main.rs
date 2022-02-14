use crossterm::event::EventStream;
use crossterm::Result;
use std::io::{stdout, Stdout};
use std::{thread, time};

mod draw;
use draw::draw;

mod event;
use event::update_keys;

mod input;
use input::Input;

const TICKRATE_MS: u64 = 100;

fn main() -> Result<()> {
    let mut t = time::Instant::now();
    let mut input = Input::default();
    let mut eventstream = EventStream::new();
    let mut stdout: Stdout = stdout();

    loop {
        if t.elapsed() < time::Duration::from_millis(TICKRATE_MS) {
            thread::sleep(t.elapsed() - time::Duration::from_millis(TICKRATE_MS));
        }

        draw(&mut stdout)?;

        t = time::Instant::now();
    }

    Ok(())
}
