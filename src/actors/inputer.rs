use actix::{Actor, Handler, Message, SyncContext, System};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use std::io::{Write, stdin, stdout};

#[derive(Debug)]
pub enum Answer {
    Play,
    Quit,
    NoAction
}

// initialise coordinates
pub struct Inputact {}

impl Actor for Inputact {
    type Context = SyncContext<Self>;
}

impl Inputact {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_up(&mut self) -> Answer {        

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        let mut answer: Answer = Answer::NoAction;

        for c in stdin.keys() {
           
            write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

            // Print the key we type...
            match c.unwrap() {
                
                // Exit.
                Key::Char('q') => {
                System::current().stop();
                    answer = Answer::Play;
                    break;
                },
                Key::Char('p') =>{
                    answer = Answer::Quit;
                    break;
                }
                _ => {answer = Answer::NoAction;},
            };

            stdout.flush().unwrap();
        }
        // Show the cursor again before we exit.
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    
        answer
    }
}


// Message sending
pub enum CMDEN {
    SETUP
}

pub struct ICMD (pub CMDEN);

impl Message for ICMD {
    type Result = Result<Answer, ()>;
}

impl Handler<ICMD> for Inputact {
    type Result = Result<Answer, ()>;

    fn handle(&mut self, ICMD(msg): ICMD, _ctx: &mut SyncContext<Self>) -> Self::Result {
        
        let answer = match msg {
            CMDEN::SETUP => { 
                self.set_up() 
            },
        };
        Ok(answer)
    }
}
