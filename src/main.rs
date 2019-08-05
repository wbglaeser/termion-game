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

    let mut coos = SyncArbiter::start(2, || Rendact::new());
    
    // Draw some new stuff
    let mut game_new = GameBoard::new();
    game_new.state.insert(
        1,
        Coordinate::new(10, 10),
    );

    // Set up board
    let msg = Command {
        command: Commands::Welcome,
    };
    
    coos.do_send(game_new);

    let ten_millis = time::Duration::from_millis(500);
    thread::sleep(ten_millis);

    let res = coos.send(msg)
        .map_err(|err| other(err.compat()))
        .and_then(|x| {
            Ok(())
        });
 

    

    sys.run();

}
