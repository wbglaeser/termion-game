use actix::{Actor,Addr, Handler, Message, SyncContext};
use std::fmt;

type Value = u64;
pub type Id = i16;

// Coordinates struct
#[derive(Clone)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Coordinates {
    pub fn new(_x: i16, _y: i16) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    } 

}

// initialise coordinates
pub struct Opponact {
    id: Id,
    coordinates: Coordinates,
}

impl Actor for Opponact {
    type Context = SyncContext<Self>;
}

impl Opponact {
    pub fn new(id: Id, x: i16, y: i16) -> Self {
        Self {
            id: id,
            coordinates: Coordinates::new(x,y),
        }
    }

    pub fn expose(self) -> (Id, Coordinates) {
        (self.id, self.coordinates)
    }
}

