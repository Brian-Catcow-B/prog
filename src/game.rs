use std::io::{stdout, Stdout};
use std::time::Duration;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::event::Event::{Key, Mouse, Resize};
use crossterm::event::KeyCode::Char;
use crossterm::{
    cursor::position,
    event::{Event, EventStream, KeyCode, MouseEvent},
};

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
                draw(&mut stdout);
            }
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if update_input(&mut input, &event) {
                            break;
                        }
                    }
                    Some(Err(e)) => {println!("Error: {:?}\r", e);
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
                if c <= '9' && c >= '0' {}
            }
        }
        Mouse(me) => {}
        Resize(x, y) => {}
    }

    false
}
