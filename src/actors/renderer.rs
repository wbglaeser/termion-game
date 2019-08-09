use actix::{Actor, Handler, Message, SyncContext, System};
use std::collections::HashMap;

use crate::actors::opponent::{Id, Coordinates};

// initialise coordinates
pub struct Rendact {
}

impl Actor for Rendact {
    type Context = SyncContext<Self>;
}

impl Rendact {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn welcome_message(&mut self) {
                
        // Welcome message
        println!("{}{}Welcome to Game. To exit please type q.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Hide);
    }

    pub fn goodbye_message(&mut self) {
        
        // Welcome message
        println!("{}{}Well that was a lot of fun. See you next time.{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            termion::cursor::Hide);

    }

    pub fn place_player(&mut self, coos: &Coordinates) {
        
        println!("{}O",
            termion::cursor::Goto(coos.x as u16, coos.y as u16));
        
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

    fn handle(&mut self, Command(msg): Command, _: &mut  SyncContext<Self>) -> Self::Result {
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


#[derive(Clone)]
pub struct GameBoard {
    pub state: HashMap<Id, Coordinates>,
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

    fn handle(&mut self, msg: GameBoard, _ctx: &mut SyncContext<Self>) -> Self::Result {
        
        for (id, coos) in &msg.state {
            self.place_player(coos);
        }
        0
    }
}
