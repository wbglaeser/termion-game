use std::io::{Error, ErrorKind};
use failure::Fail;
use actix::{Actor, Addr};
use std::fmt;

// Set up components for actor
#[derive(Clone)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(_x: i16, _y: i16) -> Self {
        Self {
            x: _x,
            y: _y,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }

}

#[derive(Clone)]
pub struct Physics {
    position: Position
}

impl Physics {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            position: Position::new(x, y),
        }
    }

    pub fn accessPosition(&self) -> &Position {
        &self.position
    }

}

// Translate errors into useful kind
pub fn other<E>(err: E) -> Error
where
    E: Into<Box<std::error::Error + Send + Sync>>,
{
    Error::new(ErrorKind::Other, err)
}
