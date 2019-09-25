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
use render::renderer::{welcome_message, goodbye_message, render_game, GameField};

fn main() {
    let stdin_channel = spawn_stdin_channel();

    println!("{:?}", termion::terminal_size());
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
   
    // Set up game GameField
    let gamefield = GameField::set_game_field(termion::terminal_size().unwrap());

    // Set up some players
    let mut gamestate = GameState::new();
    gamestate.create_player();
    gamestate.create_monster();
    gamestate.create_monster();
    gamestate.create_monster();

    // Game Loop
    match game {
        true => {
            loop {

                // render game
                render_game(&gamestate, &gamefield);

                // receive next moves
                
                // A) take user input
                let mut stdout = io::stdout().into_raw_mode().unwrap();
                let latest_keys = stdin_channel.try_iter();
                let final_val = pick_last_value(latest_keys);
                let user_input = parse_input(final_val); 
               
                // check whether use wants to end game
                let mut user_move = NextMove::NoMove;
                match user_input {
                    Msg::End => {
                        break;        
                    },
                    Msg::Continue => {},
                    _=> {user_move = translate_user_input(user_input);}

                }
                
                // B) computer monster move
                gamestate = intelligence_system(user_move, gamestate); 
                gamestate = update_system(gamestate);

                sleep(150);
            }
            goodbye_message();
            sleep(2000)
        },
        _=> {
            goodbye_message();
            sleep(2000)
        }
    }

    // Restore terminal state
    println!("{show}", show=termion::cursor::Show);
}

