use std::io::{Error, ErrorKind};
use failure::Fail;
use std::fmt;
use actix::{Actor,Addr, Handler, Message, SyncContext, System, Context};
use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;

pub type Id = i16;
pub type EntityIndex = i16;

//
// Actors
//

// Render Engine
pub struct Rendact {}

// Design style elements
const GAME_WELCOME: &'static str = "╔══════════════════════════════════════╗\n\r\
                                    ║───┬Welcome to this beautiful Game────║\n\r\
                                    ║ p ┆ play                             ║\n\r\
                                    ║ q ┆ quit                             ║\n\r\
                                    ╚═══╧══════════════════════════════════╝";

const GAME_FIELD: &'static str = "╔══════════════════════════════════════╗\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ║                                      ║\n\r\
                                  ╚══════════════════════════════════════╝";


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
        println!("{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1,1),
            GAME_WELCOME,
            termion::cursor::Hide);
    }

    pub fn goodbye_message(&mut self) {
        
        // Welcome message
        println!("{}Well that was a lot of fun. See you next time.{}",
            termion::cursor::Goto(1,4),
            termion::cursor::Hide);

    }

    pub fn place_player(&mut self, pos: &Position) {

        println!("{}O",
            termion::cursor::Goto(pos.x as u16, pos.y as u16));

    }
}

// Message Sectioni
pub enum Commands {
    Welcome,
    Goodbye,
}

pub struct Command (pub Commands);

impl Message for Command {
    type Result = Value;
}

impl Handler<Command> for Rendact {
    type Result = Value;

    fn handle(&mut self, Command(msg): Command, _: &mut  SyncContext<Self>) -> Self::Result {
        let reaction = match msg {
            Commands::Welcome => {
                self.welcome_message(); 
            },
            Commands::Goodbye => {
                self.goodbye_message();
                System::current().stop();
            }
        };
        0    
    }
}

// Player
pub struct Player {
    pub id: Id,
    pub physics: Physics,
    pub health: i16,
    pub entity_type: EntityType,
}

type Value = u64;

impl Actor for Player {
    type Context = SyncContext<Self>;
}

impl Player {
    pub fn new(id: Id, x: i16, y: i16) -> Self {
        Self {
            id: id,
            physics: Physics::new(x,y),
            health: 100,
            entity_type: EntityType::Humanoid,
        }
    }

    pub fn expose(self) -> (Id, Physics) {
        (self.id, self.physics)
    }
}

// Monster
pub struct Monster {
    pub id: Id,
    pub physics: Physics,
    pub health: i16,
    pub entity_type: EntityType,
}

impl Actor for Monster {
     type Context = SyncContext<Self>;
 }

 impl Monster {
     pub fn new(id: Id, x: i16, y: i16) -> Self {
         Self {
             id: id,
             physics: Physics::new(x,y),
             health: 100,
             entity_type: EntityType::Monster,
         }
     }

     pub fn expose(self) -> (Id, Physics) {
         (self.id, self.physics)
     }
 }

// COMPONENTS
#[derive(Clone)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(_x: i16, _y: i16) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Physics {
    position: Position
}

impl Physics {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            position: Position::new(x, y),
        }
    }

    pub fn accessPosition(&self) -> &Position {
        &self.position
    }
}

#[derive(Clone)]
pub enum EntityType {
    Humanoid,
    Monster,
    Other
}

#[derive(Clone)]
pub struct GameState {
    pub physics_components: Vec<Option<Physics>>,
    pub health_components: Vec<Option<i16>>,
    pub humanoid_state: Vec<EntityType>,

    pub players: Vec<EntityIndex>,
}

impl GameState {
    pub fn current_state(&self) -> Self {
        Self {
            physics_components: self.physics_components.clone(),
            health_components: self.health_components.clone(),
            humanoid_state: self.humanoid_state.clone(),
            players: self.players.clone(),
        }
    }
}


//
// Systems
//

pub fn physics_system(game: &GameState) {
    
    

}

pub fn render_welcome() -> u16 {
            
    // Welcome message
    println!("{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1,1),
        GAME_WELCOME,
        termion::cursor::Hide);

    game_choice()
}

pub fn clean_goodbye() {
    println!("{}{}{}",
    termion::clear::All,
    termion::cursor::Goto(1,1),
    termion::cursor::Show);
}

pub fn clean_field() {

    println!("{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1,1),
        GAME_FIELD,
        termion::cursor::Hide);
}

fn game_choice() -> u16 {
    
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    
    let mut stdin = stdin().keys();
    let mut game_code = 0 as u16;

    loop {
    
        let b = stdin.next();
        if let Some(Ok(key)) = b {
            match key {
                Key::Char('q') => {
                    break;
                },
                Key::Char('p') => {
                    game_code = 1;
                    break;
                },
                _ => {}
            }
        }
    }    
    game_code
}

pub fn take_input() -> u16 {
   
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut stdin = stdin().keys();
    let mut game_code = 3 as u16;

    loop {
    
        let b = stdin.next();
        if let Some(Ok(key)) = b {
            match key {
                Key::Char('q') => {
                    game_code = 0;
                    break;
                },
                Key::Right => {
                    game_code = 1;
                    break;
                },
                _ => {break;}
            }
        }
    }    
    game_code
}

pub fn render_game(game: GameState) {
   
    for player in game.players {
        if let Some(physics) = game.physics_components.get(player as usize) {
            match physics {
                Some(physics) => {
                    match game.humanoid_state.get(player as usize) {
                        Some(EntityType::Humanoid) => {

                            println!("{}O{}",
                                termion::cursor::Goto(physics.position.x as u16, physics.position.y as u16),
                                termion::cursor::Hide);
                        },
                        Some(EntityType::Monster) => {

                            println!("{}X{}",
                                termion::cursor::Goto(physics.position.x as u16, physics.position.y as u16),
                                termion::cursor::Hide);
                        },
                        _=> {},
                    }
                },
                _=> {}
            }
        }    
    }
}

