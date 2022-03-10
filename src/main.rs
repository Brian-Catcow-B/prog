use std::io::stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

mod game;
use game::game_loop;

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    async_std::task::block_on(game_loop());

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
