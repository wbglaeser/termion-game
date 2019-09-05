use actix::{Actor, Handler, Message, SyncContext, System};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use std::io::{Write, stdin, stdout};

use lib::take_input;

// initialise coordinates
pub struct Inputact {}

impl Actor for Inputact {
    type Context = SyncContext<Self>;
}

impl Inputact {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CallInput;

impl Message for CallInput {
    type Result = u16;
}

impl Handler<CallInput> for Inputact {
    type Result = u16;

    fn handle(&mut self, call: CallInput, _ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("ok there");
        take_input()
    }
}
