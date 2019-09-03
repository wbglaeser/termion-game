use actix::{Actor, Handler, Message, SyncContext, System};
use std::collections::HashMap;

use crate::actors::monster::Id;
use lib::{Physics, Position};

type Value = u64;

#[derive(Clone)]
pub struct GameState {
    pub state: HashMap<Id, Physics>,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}

impl Message for GameBoard {
    type Result = Value;
}

impl Handler<GameBoard> for Rendact {
    type Result = Value;

    fn handle(&mut self, msg: GameBoard, _ctx: &mut SyncContext<Self>) -> Self::Result {

        for (id, phyis) in &msg.state {
            let pos = phyis.accessPosition();
            self.place_player(pos);
        }
        0
    }
}

