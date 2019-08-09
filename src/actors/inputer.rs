use actix::{Actor, Handler, Message, SyncContext, System};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use std::io::{Write, stdin, stdout};

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
        let mut stdout = stdout().into_raw_mode().unwrap();

        for c in stdin.keys() {
           
            write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

            // Print the key we type...
            match c.unwrap() {
                // Exit.
                Key::Char('q') => {
                    println!("Well goodbye then\r");    
                    System::current().stop();
                    break;
                },
                Key::Char(c)   => println!("{}", c),
                Key::Alt(c)    => println!("Alt-{}", c),
                Key::Ctrl(c)   => println!("Ctrl-{}", c),
                Key::Left      => println!("<left>"),
                Key::Right     => println!("<right>"),
                Key::Up        => println!("<up>"),
                Key::Down      => println!("<down>"),
                _              => println!("Other"),
            }
            stdout.flush().unwrap();
        }
        // Show the cursor again before we exit.
        write!(stdout, "{}", termion::cursor::Show).unwrap();
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

    fn handle(&mut self, ICMD(msg): ICMD, _ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            CMDEN::SETUP => { 
                self.set_up(); 
            },
            _ => {println!("did not work");}
        }
        0
    }
}
