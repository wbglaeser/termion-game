use termion;
use super::visuals::{GAME_WELCOME, GAME_GOODBYE, GAME_FIELD};
use crate::components::humanoid::HumanoidState;

use lib::GameState;

pub enum test {
    this            
}

pub struct GameField{
    new_gamefield: String,
}

impl GameField {
    pub fn set_game_field(dims: (u16, u16)) -> Self {
        
        let mut new_gamefield = String::from("╔"); 
        let top_end: &str = "╗\n\r";
        let middle_start: &str = "║";
        let middle_end: &str = "║\n\r";
        let bottom_start: &str = "╚";
        let bottom_end: &str = "╝";
        let extension: &str = "═";

        // Top row
        for i in (1..dims.0-1) {
            new_gamefield.push_str(extension);
        }
        new_gamefield.push_str(top_end);

        // Inbetween row
        for i in (1..dims.1-2) {
            new_gamefield.push_str(middle_start);
            for y in (1..dims.0-1) {
                new_gamefield.push_str(" ");
            }
            new_gamefield.push_str(middle_end);
        }

        // Bottom row
        new_gamefield.push_str(bottom_start);
        for i in (1..dims.0-1) {
            new_gamefield.push_str(extension);
        }
        new_gamefield.push_str(bottom_end);

        Self { new_gamefield }
    }
}

pub fn welcome_message() {
    println!("{clear}{goto}{vis}{hide}",
             clear=termion::clear::All,
             goto=termion::cursor::Goto(1,1),
             vis=GAME_WELCOME,
             hide=termion::cursor::Hide);
}

pub fn goodbye_message() {
    println!("{clear}{goto}{vis}{hide}",
             clear=termion::clear::All,
             goto=termion::cursor::Goto(1,1),
             vis=GAME_GOODBYE,
             hide=termion::cursor::Hide);
}

pub fn render_game(gamestate: &GameState, gamefield: &GameField) {
    println!("{clear}{goto}{vis}{hide}",
             clear=termion::clear::All,
             goto=termion::cursor::Goto(1,1),
             vis=gamefield.new_gamefield,
             hide=termion::cursor::Hide);

    let mut index = 0;
    for f in &gamestate.physics {
        
        if let Some(i) = f {
            
            let pos = i.accessPositionOuter();
            
            if let Some(c) =  gamestate.humanoid.get(index).unwrap() {
                    match c {
                        HumanoidState::Human => {
                            println!("{goto}😃{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        },
                        HumanoidState::Monster => {
                            println!("{goto}👿{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        },
                        HumanoidState::Weapon => {
                            println!("{goto}🔫{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        },
                    }
            }   
        }

        index = index + 1;
    }
}

