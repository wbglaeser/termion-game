use rand::{thread_rng, Rng};
use crate::Boundaries;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(term_size: (u16, u16)) -> Self {
        Self {
            x: thread_rng().gen_range(2, term_size.0), 
            y: thread_rng().gen_range(2, term_size.1), 
        }
    }

    pub fn new_bullet(user_position: (u16, u16)) -> Self {
        Self {
            x: user_position.0,
            y: user_position.1,
        }
    }

    pub fn empty() -> Self {
        Self {
            x:0,
            y:0,
        }
    }

    pub fn accessPositionInner(self) -> (u16, u16) {
        (self.x, self.y)
    }

    // define moves
    pub fn move_up(mut self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn move_down(mut self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn move_left(mut self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn move_right(mut self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    
    pub fn is_inside(self, boundaries: &Boundaries) -> bool {
        if (self.x > boundaries.x_min && self.x < boundaries.x_max
            && self.y > boundaries.y_min && self.y < boundaries.y_max) {
            return true
        } else { false }
    }
} 

#[derive(Copy, Clone)]
struct Velocity {
    speed: f32,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Physics {
    pub position: Position,
    velocity: Velocity,
}

impl Physics {
    pub fn new(term_size: (u16, u16)) -> Self {
        Self {
            position: Position::new(term_size),
            velocity: Velocity::new()
        }
    }

    pub fn new_bullet(user_position: (u16, u16)) -> Self {
        Self {
            position: Position::new_bullet(user_position),
            velocity: Velocity::new(),
        }
    }

    pub fn accessPositionOuter(self) -> (u16, u16) {
        self.position.accessPositionInner()
    }
}

