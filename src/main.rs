use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use lib::{parse_input, parse_input_welcome, 
    spawn_stdin_channel, pick_last_value, 
    sleep, Msg, WelcomeRsp};

mod render;
use render::renderer::{welcome_message};

fn main() {
    let stdin_channel = spawn_stdin_channel();

    // Welcome message
    welcome_message();
    let mut game = false;
    loop {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let latest_keys = stdin_channel.try_iter();
        let final_val = pick_last_value(latest_keys);

        match parse_input_welcome(final_val) {
            WelcomeRsp::Exit => {
                break;
            },
            WelcomeRsp::Play => {
                game = true;
                break;
            },
            WelcomeRsp::Null => {
                {}
            }
        }
        sleep(500)
    }
    

    // Game Loop
    match game {
        true => {
            loop {
                let mut stdout = io::stdout().into_raw_mode().unwrap();
                let latest_keys = stdin_channel.try_iter();
                let final_val = pick_last_value(latest_keys);
                match parse_input(final_val) {
                    Msg::End => break,
                    _=> {}
                }
                sleep(500)
            }
        },
        _=> {}
    }

    // Restore terminal state
    println!("{show}", show=termion::cursor::Show);
}

