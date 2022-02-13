use crossterm::Result;
use std::io::{stdout, Stdout};

mod draw;
use draw::draw;

fn main() -> Result<()> {
    let mut stdout: Stdout = stdout();

    draw(&mut stdout)?;

    Ok(())
}
