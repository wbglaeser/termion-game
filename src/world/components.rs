use rand::{thread_rng, Rng};

#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn new(term_size: &(i16, i16)) -> Self {
        Self {
            x: thread_rng().gen_range(2, term_size.0),
            y: thread_rng().gen_range(2, term_size.1),
        }
    }

    pub fn compute_distance(&self, other_position: &Position) -> i16 {
        let x_delta = self.x as i16 - other_position.x as i16;
        let y_delta = self.y as i16 - other_position.y as i16;
        x_delta + y_delta
    }

    pub fn move_with_vel(&self, vel: &Velocity) -> Self {
        Self {
            x: self.x + vel.x,
            y: self.y + vel.y,
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Velocity {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone)]
pub enum EntityType {
    Human,
    Monster,
    Weapon,
    Bullet,
}

#[derive(Copy, Clone)]
pub struct Weapon;
