use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

use super::components::Velocity;

pub fn sleep(millis: u64) {
     let duration = time::Duration::from_millis(millis);
     thread::sleep(duration);
} 
  

pub fn spawn_stdin_channel() -> Receiver<termion::event::Key> {

    let mut stdin = termion::async_stdin().keys();
    let (tx, rx) = mpsc::channel::<termion::event::Key>();

    thread::spawn(move || {
        loop {
            let input = stdin.next();

            if let Some(Ok(key)) = input {
                tx.send(key).unwrap()
            }
        }
    });
    rx
}

fn pick_last_value(latest_keys: std::sync::mpsc::TryIter<termion::event::Key>) -> termion::event::Key {
    let mut final_val = termion::event::Key::Null;
    for x in latest_keys {
        final_val = x;
    }
    final_val
}

pub fn input_screen(rx: &Receiver<termion::event::Key>) -> bool {

    let mut game = false;

    loop {
        
        let latest_keys = rx.try_iter();
        let final_val = pick_last_value(latest_keys);
    
        match final_val {
            termion::event::Key::Char('p') => {
                game = true;
                break;
            },
            termion::event::Key::Char('q') => {
                break;
            },
            _ => {},
        }

        sleep(500)
    }
    game
}


pub fn take_user_input(rx: &Receiver<termion::event::Key>) -> Action {
        
    let latest_keys = rx.try_iter();
    let final_val = pick_last_value(latest_keys);
    parse_game_input(final_val)

}

#[derive(PartialEq, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Shot,
    Break,
    End,
    Nothing,
}

pub fn parse_game_input(key: termion::event::Key) -> Action {
    match key {
        termion::event::Key::Char('q') => Action::End,
        termion::event::Key::Char('b') => Action::Break,
        termion::event::Key::Char('A') => Action::Up,
        termion::event::Key::Char('B') => Action::Down,
        termion::event::Key::Char('D') => Action::Left,
        termion::event::Key::Char('C') => Action::Right,
        termion::event::Key::Char('s') => Action::Shot,
        _=> Action::Nothing,
    }
}


pub fn translate_user_input(user_input: &Action) -> Velocity {
    match user_input {
        Action::Up => Velocity {x: 0, y: -1},
        Action::Down => Velocity {x: 0, y: 1},
        Action::Left => Velocity {x: -1, y: 0},
        Action::Right => Velocity {x:1, y: 0},
        _ => Velocity {x: 0, y: 0},
    }
}
