use lib::{set_up};
use actix::sync::SyncArbiter;
use actix::{Actor, Addr, System, Message};
use futures::{future, Future};

mod actors;

use actors::renderer::{Coordinates, Commands, Command};

fn main() {

    let sys = System::new("test");

    let mut coos = SyncArbiter::start(2, || Coordinates::new());
    


    let msg = Command {
        command: Commands::Welcome,
    };
    coos.try_send(msg);

    // set_up();

    println!("Hi there");
    
    
    sys.run();

}
