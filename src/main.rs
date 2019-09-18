use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use lib::*;

mod render;
use render::renderer::{welcome_message, goodbye_message, render_game};

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
   
    // Set up some players
    let mut gamestate = GameState::new();
    gamestate.create_player();
    gamestate.create_monster();

    // Game Loop
    match game {
        true => {
            loop {

                let current_game = gamestate.clone();

                // render game
                render_game(current_game);

                // receive next moves
                // A) take user input
                let mut stdout = io::stdout().into_raw_mode().unwrap();
                let latest_keys = stdin_channel.try_iter();
                let final_val = pick_last_value(latest_keys);
                let user_move =  parse_input(final_val);

                // B) computer monster move
                sleep(500)
            }
        },
        _=> {
            goodbye_message();
            sleep(2000)
        }
    }

    // Restore terminal state
    println!("{show}", show=termion::cursor::Show);
}

