use actix::{Actor,Addr, Handler, Message, SyncContext, System};
use futures::{future, Future};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, stdin, Stdin, Stdout};
use std::collections::HashMap;
use std::{thread, time};

enum MoveX {
    Left,
    Right,
}

enum MoveY {
    Up,
    Down,
}

// initialise coordinates
pub struct Rendact {
    pub cursor_x: u16,
    pub cursor_y: u16,
}

impl Actor for Rendact {
    type Context = SyncContext<Self>;
}

impl Rendact {
    pub fn new() -> Self {
        Self {
            cursor_x: 1,
            cursor_y: 1,
        }
    }

    pub fn welcome_message(&mut self) {
                
        //let ten_millis = time::Duration::from_millis(2000);
        //thread::sleep(ten_millis);

        // Welcome message
        println!("{}{}Welcome to Game. To exit please type q.{}",
            termion::clear::All,
            termion::cursor::Goto(self.cursor_x, self.cursor_y),
            termion::cursor::Hide);
    }

    pub fn goodbye_message(&mut self) {
        
        println!("");
        // Welcome message
        println!("{}{}Well that was a lot of fun. See you next time.{}",
            termion::clear::All,
            termion::cursor::Goto(self.cursor_x, self.cursor_y),
            termion::cursor::Hide);

    }
}

pub enum Commands {
    Welcome,
    Goodbye,
}

type Value = u64;

pub struct Command (pub Commands);

impl Message for Command {
    type Result = Value;
}

impl Handler<Command> for Rendact {
    type Result = Value;

    fn handle(&mut self, Command(msg): Command, ctx: &mut  SyncContext<Self>) -> Self::Result {
        match msg {
            Commands::Welcome => {
                self.welcome_message(); 
            },
            Commands::Goodbye => {
                self.goodbye_message();
                System::current().stop();
            }
        }
    0
    }
}

// Game
#[derive(Clone)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    pub fn new(_x: u16, _y: u16) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

pub type Id = u16;

#[derive(Clone)]
pub struct GameBoard {
    pub state: HashMap<Id, Coordinate>,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

impl Message for GameBoard {
    type Result = Value;
}

impl Handler<GameBoard> for Rendact {
    type Result = Value;

    fn handle(&mut self, msg: GameBoard, ctx: &mut SyncContext<Self>) -> Self::Result {
        
        println!("hI HI");
        0
    }
}
