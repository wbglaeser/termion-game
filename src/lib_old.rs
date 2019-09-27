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
    Shot,
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
        termion::event::Key::Char('s') => Msg::Shot,
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
    pub fn new(term_size: (u16, u16)) -> Self {
        Self {
            physics: Physics::new(term_size),
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
    pub fn new(term_size: (u16, u16)) -> Self {
        Self {
            physics: Physics::new(term_size),
            humanoid: HumanoidState::Monster,
            actions: NextMove::NoMove,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponCtx {
    Loaded,
    NotLoaded,
}

pub struct Weapon {
    physics: Physics,
    humanoid: HumanoidState,
}

impl Weapon {
    pub fn new(term_size: (u16, u16)) -> Self {
        Self {
            physics: Physics::new(term_size),
            humanoid: HumanoidState::Weapon,
        }
    }
}

pub struct Bullet {
    physics: Physics,
    humanoid: HumanoidState,
}

impl Bullet {
    pub fn fire_bullet(user_position: (u16, u16)) -> Self {
        Self {
            physics: Physics::new_bullet(user_position),
            humanoid: HumanoidState::Bullet,
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
    pub weapons: Vec<Option<WeaponCtx>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            physics: Vec::new(),
            humanoid: Vec::new(),
            actions: Vec::new(),
            entities: Vec::new(),
            weapons: Vec::new(),
        }
    }

    pub fn create_player(&mut self, term_size: (u16, u16)) {
        let new_player = Player::new(term_size);
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_player.physics));
        self.humanoid.push(Some(new_player.humanoid));
        self.actions.push(Some(NextMove::NoMove));
        self.entities.push(entity_count as u16 + 1);
        self.weapons.push(None);
    }
    pub fn create_monster(&mut self, term_size: (u16, u16)) {
        let new_monster = Monster::new(term_size);
        let entity_count = self.entities.len();
        
        self.physics.push(Some(new_monster.physics));
        self.humanoid.push(Some(new_monster.humanoid));
        self.actions.push(Some(NextMove::NoMove));
        self.entities.push(entity_count as u16 + 1);
        self.weapons.push(None);
    }
    pub fn create_weapon(&mut self, term_size: (u16, u16)) {
        let new_weapon = Weapon::new(term_size);
        let entity_count = self.entities.len();

        self.physics.push(Some(new_weapon.physics));
        self.humanoid.push(Some(new_weapon.humanoid));
        self.entities.push(entity_count as u16 + 1);
        self.weapons.push(None);
    }

    pub fn create_bullet(&mut self, user_position: (u16, u16)) {
        let new_bullet = Bullet::fire_bullet(user_position);
        let entity_count = self.entities.len();

        self.physics.push(Some(new_bullet.physics));
        self.humanoid.push(Some(new_bullet.humanoid));
        self.entities.push(entity_count as u16 + 1);
        self.weapons.push(None);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NextMove {
    Up,
    Down,
    Left,
    Right,
    NoMove,
    Shot,
}

pub fn translate_user_input(user_input: Msg) -> NextMove {
    match user_input {
        Msg::Left => NextMove::Left,
        Msg::Right => NextMove::Right,
        Msg::Up => NextMove::Up,
        Msg::Down => NextMove::Down,
        Msg::Shot => NextMove::Shot,
        _ => {NextMove::NoMove},
    }
}

pub fn retrieve_user_position(gamestate: &GameState) -> (u16, u16) {
    let mut new_position = (0,0);

    for (h, p) in gamestate.humanoid.iter().zip(gamestate.physics.iter()) {
        if let Some(i) = h {
            match i {
                HumanoidState::Human => {
                    if let Some(k) = p {
                        new_position = k.accessPositionOuter();
                    }
                },
                _ => {}
            }
        }
    }
    new_position
}

pub fn retrieve_weapon_position(gamestate: &GameState) -> (u16, u16) {
    let mut new_position = (0,0);

    for (h, p) in gamestate.humanoid.iter().zip(gamestate.physics.iter()) {
        if let Some(i) = h {
            match i {
                HumanoidState::Weapon => {
                    if let Some(k) = p {
                        new_position = k.accessPositionOuter();
                    }
                },
                _ => {}
            }
        }
    }
    new_position
}

pub fn weapon_system(mut gamestate: GameState, user_position: (u16, u16), weapon_position: (u16, u16)) -> GameState {
    
    let mut new_weapon_vec = Vec::new();
 
    if user_position == weapon_position {
        for h in gamestate.humanoid.iter() {
        
            if let Some(i) = h {
                match i {
                    HumanoidState::Human => {
                        new_weapon_vec.push(Some(WeaponCtx::Loaded));
                    },
                    HumanoidState::Weapon => {
                        new_weapon_vec.push(Some(WeaponCtx::Loaded));
                    }
                    _=> {new_weapon_vec.push(None);}
                }
            }
        }
    }
    gamestate.weapons = new_weapon_vec;
    gamestate
}

pub fn intelligence_system(mut gamestate: GameState, user_position: (u16, u16), user_move: NextMove) -> GameState {

    let mut new_action_set = Vec::new();

    // iterate through all entities
    let mut index = 0;
    for h in gamestate.humanoid.iter() {      
        if let Some(i) = h {
            match i {
                HumanoidState::Human => {
                    new_action_set.push(Some(user_move));
                },
                HumanoidState::Monster => {
                    if let Some(c) = gamestate.physics.get(index).unwrap() {
                        new_action_set.push(Some(compute_shortest_distance(c.accessPositionOuter(), user_position)));
                    }
                },
                HumanoidState::Weapon => {
                    if let Some(w) = gamestate.weapons.get(index).unwrap_or(&Some(WeaponCtx::NotLoaded)) {
                        match w {
                            WeaponCtx::Loaded => {
                                new_action_set.push(Some(user_move));
                            }
                            _ => {
                                new_action_set.push(Some(NextMove::NoMove));
                            }
                        }
                    }
                },                    
                _ => {new_action_set.push(Some(NextMove::NoMove));}
            }
        }
        index = index + 1;
    }

// update new actions sets
gamestate.actions = new_action_set;
gamestate
}

// Action System
pub fn update_system(mut gamestate: GameState, boundaries: &Boundaries, user_position: (u16, u16)) -> GameState {
    let mut new_physics_vec = Vec::new();
    let mut shot_fired = false;

    for (cp,nm) in gamestate.physics.iter().zip(gamestate.actions.iter()) {

        if let Some(c) = cp {

            let mut new_physics = c.clone();
            let mut new_position = new_physics.position.clone();

            if let Some(m) = nm {
                match m {
                    NextMove::Up => {
                        new_position = new_physics.position.move_up();
                        if (new_position.is_inside(boundaries)) {
                            new_physics.position = new_position;
                        }
                    },
                    NextMove::Down => {
                        new_position = new_physics.position.move_down();
                        if (new_position.is_inside(boundaries)) {
                            new_physics.position = new_position;
                        }
                    },
                    NextMove::Left => {
                        new_position = new_physics.position.move_left();
                        if (new_position.is_inside(boundaries)) {
                            new_physics.position = new_position;
                        }
                     
                    },
                    NextMove::Right => {
                        new_position = new_physics.position.move_right();
                        if (new_position.is_inside(boundaries)) {
                            new_physics.position = new_position;
                        }
                    }, 
                    NextMove::Shot => {
                        if let Some(w) = gamestate.weapons.get(0).unwrap_or(&Some(WeaponCtx::NotLoaded)) {
                            match w {
                                WeaponCtx::Loaded => {shot_fired = true;},
                                _=> {}
                            }
                        }
                    }
                    _ => {}
                }
            new_physics_vec.push(Some(new_physics));
            }

        } else {
            new_physics_vec.push(None);
        }

    }

    gamestate.physics = new_physics_vec;
    
    if shot_fired {
        gamestate.create_bullet(user_position);
    }

    // overwrite gamestate
    gamestate
} 

fn compute_shortest_distance(own_pos: (u16, u16), target_pos: (u16, u16)) -> NextMove {
    let mut next_move = NextMove::NoMove;
    let mut shortest_distance: i16 = 1000;
    for x in [NextMove::Up, NextMove::Down, NextMove::Left, NextMove::Right].iter() {
        let potential_position = position_change(own_pos, &x);
        let d = compute_distance(potential_position, target_pos);
        if (d < shortest_distance) {
            next_move = *x;
            shortest_distance = d;
        }
    }
    next_move
}

fn position_change(position: (u16, u16), next_move: &NextMove) -> (u16, u16) {
    let mut new_position = position;
    match next_move {
        NextMove::Up => new_position = (new_position.0, new_position.1 - 1), 
        NextMove::Down => new_position = (new_position.0, new_position.1 + 1), 
        NextMove::Left => new_position = (new_position.0 - 1, new_position.1), 
        NextMove::Right => new_position = (new_position.0 + 1, new_position.1), 
        _ => {}
    }
    new_position
}

fn compute_distance(potential_position: (u16, u16), target_position: (u16, u16)) -> i16 {
    let x_delta = potential_position.0 as i16 - target_position.0 as i16; 
    let y_delta = potential_position.1 as i16 - target_position.1 as i16; 
    x_delta.abs() + y_delta.abs()
}



pub struct Boundaries {
    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
}

impl Boundaries {
    pub fn set_boundaries(term_size: (u16, u16)) -> Self {
        Self {
            x_min: 1,
            x_max: term_size.0-1,
            y_min: 1,
            y_max: term_size.1-1,
        }
    }
}