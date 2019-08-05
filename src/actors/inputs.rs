use actix::{Actor,Addr, Handler, Message, SyncContext};
use futures::{future, Future};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, stdin, Stdin, Stdout};
use std::collections::HashMap;

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

    fn move_x(&mut self, d: MoveX) {
        match d {
            MoveX::Left => {
                if self.cursor_x == 1 {}
                else { self.cursor_x -= 1 }
            },
            MoveX::Right => {
                self.cursor_x += 1
            },
        }
    }

    fn move_y(&mut self, d: MoveY) {
        match d {
            MoveY::Up => {
                if self.cursor_y == 1 {}
                else { self.cursor_y -= 1 }
            },
            MoveY::Down => {
                self.cursor_y += 1
            },
        }
    }

    pub fn set_up(&mut self) {

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Welcome message
        write!(stdout, "{}{}Welcome to Game. To exit please type q.{}",
            termion::clear::All,
            termion::cursor::Goto(self.cursor_x, self.cursor_y),
            termion::cursor::Hide).unwrap();

        stdout.flush().unwrap();

        // Accept Input
        for c in stdin.keys() {

            // Clear the current line.
            for i in 1..100 {
                write!(stdout, "{}{}", termion::cursor::Goto(1, i), termion::clear::CurrentLine).unwrap();
            }

            // Print the key we type...
            match c.unwrap() {
                
                // Exit.
                Key::Char('q') => break,
                Key::Char(c)   => println!("{}", c),
                Key::Alt(c)    => println!("Alt-{}", c),

                // Move around game
                Key::Left => {
                    self.move_x(MoveX::Left);
                    println!("{}o", termion::cursor::Goto(self.cursor_x, self.cursor_y));
                },
                Key::Right => {
                    self.move_x(MoveX::Right);
                    println!("{}o", termion::cursor::Goto(self.cursor_x, self.cursor_y));
                },
                Key::Up => {
                    self.move_y(MoveY::Up);
                    println!("{}o", termion::cursor::Goto(self.cursor_x, self.cursor_y));
                },
                Key::Down => {
                    self.move_y(MoveY::Down);
                    println!("{}o", termion::cursor::Goto(self.cursor_x, self.cursor_y));
                },
                _  => println!("Other"),
            }

            // Flush again.
            stdout.flush().unwrap();
        }

        // Show the cursor again before we exit
        write!(stdout, "{}", termion::cursor::Show).unwrap();
            
        // Flush again.
        stdout.flush().unwrap();
    }
}

pub enum Commands {
    Welcome,
    Goodbye,
}

type Value = u64;

pub struct Command {
    pub command: Commands,
}

impl Message for Command {
    type Result = Value;
}

impl Handler<Command> for Rendact {
    type Result = Value;

    fn handle(&mut self, msg: Command, ctx: &mut  SyncContext<Self>) -> Self::Result {
        match msg.command {
            Commands::Welcome => {
                self.set_up();
            },
            Commands::Goodbye => {}
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
