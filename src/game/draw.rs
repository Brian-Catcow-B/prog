use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{Stdout, Write};

pub fn draw(stdout: &mut Stdout) -> Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".red()))?;
            }
        }
    }
    stdout.flush()?;

    Ok(())
}
