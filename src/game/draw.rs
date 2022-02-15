use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{Stdout, Write};

use crate::game::Input;

const X_MAX: u16 = 80;
const Y_MAX: u16 = 40;

pub fn draw(stdout: &mut Stdout, input: &Input) -> Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..Y_MAX {
        for x in 0..X_MAX {
            if (y == 0 || y == Y_MAX - 1) || (x == 0 || x == X_MAX - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".grey()))?;
            }
        }
    }

    for i in 0..=9 {
        if (input.num_key_bitwise & (0x01 << i)) > 0 {
            stdout
                .queue(cursor::MoveTo(i + 1, 1))?
                .queue(style::Print(format!("{}", i)))?;
        }
    }

    stdout.flush()?;

    Ok(())
}
