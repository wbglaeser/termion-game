use lib::{other};
use actix::sync::SyncArbiter;
use actix::{Actor, Addr, System, Message, Arbiter};
use futures::{future, Future};
use failure::Fail;
use std::{thread, time};

mod actors;

use actors::renderer::{Rendact, Commands, Command, Coordinate, GameBoard, Id};

fn main() {
       
    let sys = System::new("NewSys");    

        // Set up Render Actor
        let mut coos = SyncArbiter::start(2, || Rendact::new());
        
        // Set up board
        let msg = Command {
            command: Commands::Welcome,
        };
        
        let res1 = coos.send(msg)
            .map_err(|err| other(err.compat()))
            .and_then(|x| {
                match x {
                    0 => println!("Hi there"),
                    _ => println!("Upsi")
                }
                future::ok(x)
            });

        // Draw some new stuff
        let mut game_new = GameBoard::new();
        game_new.state.insert(
            1,
            Coordinate::new(10, 10),
        );

        let res2 = coos.send(game_new)
            .map_err(|err| other(err.compat()))
            .and_then(|x| {
                Ok(0)
            });

    sys.run();
    //System::current().stop(); why does it not stop
}
