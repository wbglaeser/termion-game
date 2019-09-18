use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub mod components;
use components::physics::*;
use components::humanoid::*;

pub fn spawn_stdin_channel() -> Receiver<termion::event::Key> {

    let mut stdin = termion::async_stdin().keys();
    let (tx, rx) = mpsc::channel::<termion::event::Key>();

    thread::spawn(move || {
        loop {
            let input = stdin.next();

            if let Some(Ok(key)) = input {
                    tx.send(key).unwrap()
            }
        }
    });
    rx
}

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub fn pick_last_value(latest_keys: std::sync::mpsc::TryIter<termion::event::Key>) -> termion::event::Key {
    let mut final_val = termion::event::Key::Null;
    for x in latest_keys {
        final_val = x;
    }
    final_val
}

#[derive(Debug)]
pub enum Msg {
    Continue,
    End,
    Left,
    Right,
    Up,
    Down,
}

pub enum WelcomeRsp {
    Play,
    Exit,
    Null,
}

pub fn parse_input_welcome(val: termion::event::Key) -> WelcomeRsp {
    match val {
        termion::event::Key::Char('q') => {WelcomeRsp::Exit},
        termion::event::Key::Char('p') => {WelcomeRsp::Play},
        _=> {WelcomeRsp::Null},
    }
}

pub fn parse_input(val: termion::event::Key) -> Msg {
    
    match val {
        termion::event::Key::Char('q') => Msg::End,
        termion::event::Key::Char('D') => Msg::Left,
        termion::event::Key::Char('C') => Msg::Right,
        termion::event::Key::Char('A') => Msg::Up,
        termion::event::Key::Char('B') => Msg::Down,
        _=> Msg::Continue,
    }
}

// Entity Types
pub struct Player {
    physics: Physics,
    humanoid: HumanoidState,
    actions: NextMove,
}


impl Player {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            humanoid: HumanoidState::Human,
            actions: NextMove::NoMove,
        }
    }
}

pub struct Monster {
    physics: Physics,
    humanoid: HumanoidState,
    actions: NextMove,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            humanoid: HumanoidState::Monster,
            actions: NextMove::NoMove,
        }
    }
}

pub type EntityIndex = u16;

// GameState
#[derive(Clone)]
pub struct GameState {
    pub physics: Vec<Option<Physics>>,
    pub humanoid: Vec<Option<HumanoidState>>, 
    pub actions: Vec<Option<NextMove>>,
    pub entities: Vec<EntityIndex>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            physics: Vec::new(),
            humanoid: Vec::new(),
            actions: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn create_player(&mut self) {
        let new_player = Player::new();
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_player.physics));
        self.humanoid.push(Some(new_player.humanoid));
        self.actions.push(Some(NextMove::NoMove));
        self.entities.push(entity_count as u16 + 1);
    }
    pub fn create_monster(&mut self) {
        let new_monster = Monster::new();
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_monster.physics));
        self.humanoid.push(Some(new_monster.humanoid));
        self.actions.push(Some(NextMove::NoMove));
        self.entities.push(entity_count as u16 + 1);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NextMove {
    Up,
    Down,
    Left,
    Right,
    NoMove,
}

pub fn translate_user_input(user_input: Msg) -> NextMove {
    match user_input {
        Msg::Left => NextMove::Left,
        Msg::Right => NextMove::Right,
        Msg::Up => NextMove::Up,
        Msg::Down => NextMove::Down,
        _ => {NextMove::NoMove},
    }
}


pub fn intelligence_system(user_move: NextMove, mut gamestate: GameState) -> GameState {

    // retrieve position of user
    let mut user_position = (0, 0);
    for (h, p) in gamestate.humanoid.iter().zip(gamestate.physics.iter()) {
        if let Some(i) = h {
            match i {
                HumanoidState::Human => {
                    if let Some(k) = p {
                        user_position = k.accessPositionOuter();
                    }
                },
                HumanoidState::Monster => {},
            }
        }
    }

    let mut new_action_set = Vec::new();

    // iterate through all entities
    for (h, p) in gamestate.humanoid.iter().zip(gamestate.physics.iter()) {

        // check if monster or user
        if let Some(c) = p {
            
            if let Some(i) = h {
                match i {
                    HumanoidState::Human => {
                        new_action_set.push(Some(user_move));
                    },
                    HumanoidState::Monster => {
                        new_action_set.push(Some(compute_shortest_distance(c.accessPositionOuter(), user_position)));
                    }
                    
                }
            }
        }

    }

    // update new actions sets
gamestate.actions = new_action_set;
gamestate
}

fn compute_shortest_distance(own_pos: (u16, u16), target_pos: (u16, u16)) -> NextMove {
    NextMove::NoMove
}


// Action System
pub fn update_system(mut gamestate: GameState) -> GameState {
    
    let mut new_physics_vec = Vec::new();

    for (cp,nm) in gamestate.physics.iter().zip(gamestate.actions.iter()) {

        if let Some(c) = cp {

            let mut new_physics = c.clone();

            if let Some(m) = nm {
                match m {
                    NextMove::Up => {
                        new_physics.position = new_physics.position.move_up();
                    },
                    NextMove::Down => {
                        new_physics.position = new_physics.position.move_down();
                    },
                    NextMove::Left => {
                        new_physics.position = new_physics.position.move_left();
                    },
                    NextMove::Right => {
                        new_physics.position = new_physics.position.move_right();
                    }, 
                    _ => {}
                }
            new_physics_vec.push(Some(new_physics));
            }

        } else {
            new_physics_vec.push(None);
        }

    }

    // overwrite gamestate
    gamestate.physics = new_physics_vec;
    gamestate
} 
