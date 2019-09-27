use rand::{thread_rng, Rng};

#[derive(Debug)]
pub(crate) struct Position {
    x,
    y,
}

impl Position {
    pub fn new(&term_size: (u16, u16)) -> Self {
        Self {
            x: thread_rng().gen_range(2, term_size.0),
            y: thread_rng().gen_range(2, term_size.1),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Velocity {
    x,
    y,
}

pub(crate) enum EntityType {
    Human,
    Monster,
    Weapon,
    Bullet,
}

pub(crate) struct Weapon;
