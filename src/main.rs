use actix::sync::SyncArbiter;
use actix::{System, Arbiter};
use futures::{future, Future};

mod actors;

use actors::renderer::{Rendact, Commands, Command, GameBoard};
use actors::inputer::{Inputact, ICMD, CMDEN};
use actors::opponent::{Opponact};

fn main() {
       
    let sys = System::new("newSys");
    
    let opp = Opponact::new(1, 5, 10);
    let opp_state = opp.expose();

    let opp2 = Opponact::new(2, 4, 8);
    let opp2_state = opp2.expose();

    let mut game = GameBoard::new();
    game.state.insert(opp_state.0, opp_state.1);
    game.state.insert(opp2_state.0, opp2_state.1);


    // Set up Render Actor
    let rend = SyncArbiter::start(2, || Rendact::new());
    let fut = rend.send(Command(Commands::Welcome))
        .map_err(|_| ())
        .and_then(|_x| { future::ok(()) });
    Arbiter::spawn(fut);    

    //let fut2 = rend.send(game)
    //    .map_err(|_| ())
    //    .and_then(|_x| {future::ok(())});
    //Arbiter::spawn(fut2);

    // Set up Input Actor
    let inps = SyncArbiter::start(2, || Inputact::new());
    let fut3 = inps.send(ICMD(CMDEN::SETUP))
        .map_err(|_| ())
        .and_then(|_x| { future::ok(()) });

    Arbiter::spawn(fut3);

    // Run this stuff
    sys.run();
}
