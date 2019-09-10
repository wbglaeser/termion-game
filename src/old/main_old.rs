use actix::sync::SyncArbiter;
use actix::{System, Arbiter, Context, Actor, Addr};
use futures::{future, Future};
use std::{thread, time};
use std::io::{Write, stdin, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use lib::{GameState, Player, Monster, take_input, render_welcome, render_game, clean_goodbye, clean_field};

mod actors;
use actors::inputer::{Inputact, CallInput};

fn main() {
  
    let fut = actix::run(|| {

        let inputer = SyncArbiter::start(2, || Inputact::new());
 
        let play_bool = render_welcome();

        // Kill game on command
        if (play_bool == 1) {
            clean_field();
        } else {
            clean_goodbye();
            std::process::exit(1);
            return future::ok(())
        }
 
        // Else Set up the game
        let player = Player::new(0, 5, 10);
        let monster = Monster::new(1, 4, 8);

        // Initialise Gamestate
        let mut gamestate = GameState{
            physics_components: vec![],
            health_components: vec![],
            humanoid_state: vec![],
            players: vec![],
        };

        gamestate.physics_components.push(Some(player.physics));
        gamestate.physics_components.push(Some(monster.physics));

        gamestate.health_components.push(Some(player.health));
        gamestate.health_components.push(Some(monster.health));
       
        gamestate.humanoid_state.push(player.entity_type);
        gamestate.humanoid_state.push(monster.entity_type);

        gamestate.players.push(player.id);
        gamestate.players.push(monster.id);

        // Initialise Game 
        let initial_game = gamestate.current_state();
        render_game(initial_game);
         
        // GameLoop
        loop {

            // copy game state into loop
            let input_state = gamestate.current_state();
 
            for phys in &input_state.physics_components {
            
                //let res = take_input();
                //println!("{}", res.unwrap());
            }

            gamestate = input_state.current_state();
            
            let wait = time::Duration::from_millis(100);
            thread::sleep(wait);

        }

        std::process::exit(1);
        future::ok(())
    });
}
