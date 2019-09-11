use termion;
use super::visuals::{GAME_WELCOME, GAME_GOODBYE, GAME_FIELD};

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

pub fn render_board() {

}
