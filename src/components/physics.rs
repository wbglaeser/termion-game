use rand::{thread_rng, Rng};

struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: thread_rng().gen_range(1, 20), 
            y: thread_rng().gen_range(1, 20), 
        }
    }
} 

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

pub struct Physics {
    position: Position,
    velocity: Velocity,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            velocity: Velocity::new()
        }
    }
}

