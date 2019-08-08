use lib::{other};
use actix::sync::SyncArbiter;
use actix::{Actor, Addr, System, Message, Arbiter};
use futures::{future, Future};
use failure::Fail;
use std::{thread, time};
use std::io::{stdin, Stdin};


mod actors;

use actors::renderer::{Rendact, Commands, Command, Coordinate, GameBoard, Id};
use actors::inputer::{Inputact, ICMD, CMDEN};

fn main() {
       
    let sys = System::new("newSys");

    // Set up Render Actor
    let mut coos = SyncArbiter::start(2, || Rendact::new());
    let fut = coos.send(Command(Commands::Welcome))
        .map_err(|err| ())
        .and_then(|x| { future::ok(()) });
    
    Arbiter::spawn(fut);    

    // Set up Input Actor
    let mut inps = SyncArbiter::start(2, || Inputact::new());
    let stdin = stdin();
    let fut2 = inps.send(ICMD(CMDEN::SETUP))
        .map_err(|err| ())
        .and_then(|x| { future::ok(()) });

    Arbiter::spawn(fut2);

    // Run this stuff
    sys.run();
}
