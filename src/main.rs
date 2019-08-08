use lib::{other};
use actix::sync::SyncArbiter;
use actix::{Actor, Addr, System, Message, Arbiter};
use futures::{future, Future};
use failure::Fail;
use std::{thread, time};

mod actors;

use actors::renderer::{Rendact, Commands, Command, Coordinate, GameBoard, Id};

fn main() {
       
    let sys = System::new("newSys");

    // Set up Render Actor
    let mut coos = SyncArbiter::start(2, || Rendact::new());
            

    let fut = coos.send(Command(Commands::Welcome))
        .map_err(|err| ())
        .and_then(|x| { future::ok(()) });
    
    Arbiter::spawn(fut);
    
    let fut2 = coos.send(Command(Commands::Goodbye))
        .map_err(|err| ())
        .and_then(|x| { future::ok(()) });
    
    Arbiter::spawn(fut2);

    sys.run();
}
