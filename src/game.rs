use std::io::stdout;
use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::event::Event::{Key, Mouse, Resize};
use crossterm::event::KeyCode::Char;
use crossterm::event::{Event, EventStream, KeyCode};

mod input;
use input::Input;

mod draw;
use draw::draw;

pub async fn game_loop() {
    let mut input = Input::default();
    let mut reader = EventStream::new();
    let mut stdout = stdout();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            _ = delay => {
                draw(&mut stdout, &input).expect("[!] Draw failed");
                input.reset();
            }
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if update_input(&mut input, &event) {
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error: {:?}\r", e);
                        break;
                    },
                    None => break,
                }
            }
        };
    }
}

fn update_input(input: &mut Input, event: &Event) -> bool {
    match event {
        Key(ke) => {
            if ke.code == KeyCode::Esc {
                return true;
            }

            if let Char(c) = ke.code {
                if c >= '0' && c <= '9' {
                    input.num_key_bitwise |= 0x01 << ((c as u8) - 0x30);
                } else if c == 'r' {
                    input.run_prog = true;
                } else if c == 's' {
                    input.stop_prog = true;
                }
            }
        }
        Mouse(_me) => {}
        Resize(_x, _y) => {}
    }

    false
}
