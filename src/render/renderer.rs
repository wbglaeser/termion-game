use termion;
use super::visuals::{GAME_WELCOME, GAME_GOODBYE, GAME_FIELD};
use lib::GameState;

pub enum test {
    this            
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

pub fn render_game(gamestate: GameState) {
    println!("{clear}{goto}{vis}{hide}",
             clear=termion::clear::All,
             goto=termion::cursor::Goto(1,1),
             vis=GAME_FIELD,
             hide=termion::cursor::Hide);

    for f in gamestate.physics {
        
        if let Some(i) = f {
            
            let pos = i.accessPositionOuter();
            
            println!("{goto}X{hide}",
                goto=termion::cursor::Goto(pos.0, pos.1),
                hide=termion::cursor::Hide);
        }


    }
    
}

