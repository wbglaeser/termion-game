use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: thread_rng().gen_range(2, 39), 
            y: thread_rng().gen_range(2, 17), 
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
    pub fn new() -> Self {
        Self {
            position: Position::new(),
            velocity: Velocity::new()
        }
    }

    pub fn accessPositionOuter(self) -> (u16, u16) {
        self.position.accessPositionInner()
    }
}

