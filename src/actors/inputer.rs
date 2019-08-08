use actix::{Actor,Addr, Handler, Message, SyncContext, System};
use futures::{future, Future};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{stdin, Stdin};

type Value = u64;

// initialise coordinates
pub struct Inputact {}

impl Actor for Inputact {
    type Context = SyncContext<Self>;
}

impl Inputact {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_up(&mut self) {        

        let stdin = stdin();

        for c in stdin.keys() {
            
            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char('q') => break,
                Key::Char(c)   => println!("{}", c),
                Key::Alt(c)    => println!("Alt-{}", c),
                Key::Ctrl(c)   => println!("Ctrl-{}", c),
                Key::Left      => println!("<left>"),
                Key::Right     => println!("<right>"),
                Key::Up        => println!("<up>"),
                Key::Down      => println!("<down>"),
                _              => println!("Other"),
            }
        }
    }
}

pub enum CMDEN {
    SETUP
}

// Send stdin
pub struct ICMD (pub CMDEN);

impl Message for ICMD {
    type Result = Value;
}

impl Handler<ICMD> for Inputact {
    type Result = Value;

    fn handle(&mut self, ICMD(msg): ICMD, ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            CMDEN::SETUP => { 
                self.set_up(); 
            },
            _ => {println!("did not work");}
        }
        0
    }
}
