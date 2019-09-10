use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


pub fn spawn_stdin_channel() -> Receiver<termion::event::Key> {

    let mut stdin = termion::async_stdin().keys();
    let (tx, rx) = mpsc::channel::<termion::event::Key>();

    thread::spawn(move || {
        loop {
            let tx = tx.clone();
            let input = stdin.next();

            if let Some(Ok(key)) = input {
                    tx.send(key).unwrap()
            }
        }
    });
    rx
}

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub fn pick_last_value(latest_keys: std::sync::mpsc::TryIter<termion::event::Key>) -> termion::event::Key {
    let mut final_val = termion::event::Key::Null;
    for x in latest_keys {
        final_val = x;
    }
    final_val
}

pub enum Msg {
    End,
    Continue,
}

pub fn parse_input(val: termion::event::Key) -> Msg {
    match val {
        termion::event::Key::Char('q') => {Msg::End},
        termion::event::Key::Char(c) => {
            println!("{clear}{goto}{c}",
                     clear=termion::clear::All,
                     goto=termion::cursor::Goto(1,1),
                     c=c);
            Msg::Continue
        }
        _=> {
            println!("{clear}{goto}No News",
                     clear=termion::clear::All,
                     goto=termion::cursor::Goto(1,1));
            Msg::Continue
        }
    }
}
