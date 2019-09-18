use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod components;
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
        termion::event::Key::Left => Msg::Left,
        termion::event::Key::Right => Msg::Right,
        termion::event::Key::Up => Msg::Up,
        termion::event::Key::Down => Msg::Down,
        _=> Msg::Continue,
    }
}

// Entity Types
pub struct Player {
    physics: Physics,
    humanoid: HumanoidState,
}


impl Player {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            humanoid: HumanoidState::Human,
        }
    }
}

pub struct Monster {
    physics: Physics,
    humanoid: HumanoidState,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            humanoid: HumanoidState::Monster,
        }
    }
}

pub type EntityIndex = u16;

// GameState
#[derive(Clone)]
pub struct GameState {
    pub physics: Vec<Option<Physics>>,
    pub humanoid: Vec<Option<HumanoidState>>,
    pub entities: Vec<EntityIndex>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            physics: Vec::new(),
            humanoid: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn create_player(&mut self) {
        let new_player = Player::new();
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_player.physics));
        self.humanoid.push(Some(new_player.humanoid));
        self.entities.push(entity_count as u16 + 1);
    }
    pub fn create_monster(&mut self) {
        let new_monster = Monster::new();
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_monster.physics));
        self.humanoid.push(Some(new_monster.humanoid));
        self.entities.push(entity_count as u16 + 1);
    }
}


