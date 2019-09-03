use actix::{Actor,Addr, Handler, Message, SyncContext};
use std::fmt;

use lib::{Physics, Position};

type Value = u64;
pub type Id = i16;

// initialise coordinates
pub struct Monster {
    id: Id,
    physics: Physics,
}

impl Actor for Monster {
    type Context = SyncContext<Self>;
}

impl Monster {
    pub fn new(id: Id, x: i16, y: i16) -> Self {
        Self {
            id: id,
            physics: Physics::new(x,y),
        }
    }

    pub fn expose(self) -> (Id, Physics) {
        (self.id, self.physics)
    }
}

