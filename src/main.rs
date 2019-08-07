use lib::{other};
use actix::sync::SyncArbiter;
use actix::{Actor, Addr, System, Message, Arbiter};
use futures::{future, Future};
use failure::Fail;
use std::{thread, time};

mod actors;

use actors::renderer::{Rendact, Commands, Command, Coordinate, GameBoard, Id};

fn main() {
       
    let sys = System::new("test");

    // Set up Render Actor
    let mut coos = SyncArbiter::start(2, || Rendact::new());
        
    let msg = Command {
        command: Commands::Welcome,
    };
    let fut1 = coos.send(msg)
        .and_then(|x| { future::ok(()) })
        .map_err(|_| ());

    sys.run();
}
