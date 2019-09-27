use itertools::izip;
use super::components::{EntityType, Position, Velocity, Weapon};
use super::input::{Action, translate_user_input};
use super::world::World;
use super::visuals::{GAME_GOODBYE, GAME_WELCOME, GameField};

// System to adjust position
struct PositionSystem;

impl PositionSystem {
    fn run(mut world: World) {
        for (position, velocity) in izip!(world.positions, world.velocities) { 
            position.x += velocity.y;
            position.y += velocity.x;
        }
    }
}

// System to check for weapons
pub struct WeaponSystem;

impl WeaponSystem {
    pub fn try_pick_up(mut world: World, user_position: &Position, weapon_position: &Position) -> World {

        if user_position == weapon_position {
            for (etype, weapon) in izip!(world.entitytype, world.weapons) {
                match etype {
                    EntityType::Human => {weapon = Some(Weapon);},
                    EntityType::Weapon => {weapon = Some(Weapon);},
                    _=> {} 
                }
            }
        }

        world

    }
}

// System to compute next moves
pub struct VelocitySystem;

impl VelocitySystem {
    pub fn run(mut world: World, user_position: &Position, user_input: Action) -> World {
    
        let human_vel = Velocity{x:0, y:0};

        for (etype, velocity, position, weapon) 
            in izip!(world.entitytype, world.velocities, world.positions, world.weapons) {
            match etype {
                EntityType::Human => {
                    velocity = translate_user_input(user_input);
                    human_vel = velocity.clone();
                },
                EntityType::Monster => {
                    velocity = compute_monster_move(position, user_position);
                },
                EntityType::Weapon => {
                    if let Some(weap) = weapon {
                        velocity = human_vel;
                    }
                },
                _=> {}
            }
        }
        world
    }
}

fn compute_monster_move(position: Position, user_position: &Position) -> Velocity {
    
    let mut new_velocity = Velocity{x:0, y:0};
    let mut shortest_distance: i16 = 1000; 

    for vel in [Velocity{x:0,y:-1}, Velocity{x:0,y:1}, Velocity{x:-1,y:0}, Velocity{x:1,y:0}, Velocity{x:0,y:0}].iter() {
        let potential_position = position.move_with_vel(vel);
        let d = potential_position.compute_distance(user_position);
        if d < shortest_distance {
            new_velocity = *vel;
            shortest_distance = d;
        }
    }
    new_velocity
}

// System to render world to screen
pub struct RenderingSystem;

impl RenderingSystem {

    pub fn render_welcome() {
        println!("{clear}{goto}{vis}{hide}",
            clear=termion::clear::All,
            goto=termion::cursor::Goto(1,1),
            vis=GAME_WELCOME,
            hide=termion::cursor::Hide)
    }

    pub fn goodbye_message() {
        println!("{clear}{goto}{vis}{hide}",
            clear=termion::clear::All,
            goto=termion::cursor::Goto(1,1),
            vis=GAME_GOODBYE,
            hide=termion::cursor::Hide);
    }

    pub fn render_world<'a>(world: &'a World, gamefield: &GameField) -> (&'a Position, &'a Position) {

        let mut user_position = &Position{x:0, y: 0};
        let mut weapon_position = &Position{x:0, y: 0};

        println!("{clear}{goto}{vis}{hide}",
            clear=termion::clear::All,
            goto=termion::cursor::Goto(1,1),
            vis=gamefield.new_gamefield,
            hide=termion::cursor::Hide);

        for (pos, entitytype, weapon) in izip!(&world.positions, &world.entitytype,  &world.weapons) {
            match entitytype {
                EntityType::Human => {
                    user_position = pos;
                    if let Some(weap) =  weapon {
                          println!("{goto}🤠{hide}",
                            goto=termion::cursor::Goto(pos.x as u16, pos.y as u16),
                            hide=termion::cursor::Hide);
                    } else {
                        println!("{goto}😃{hide}",
                            goto=termion::cursor::Goto(pos.x as u16, pos.y as u16),
                            hide=termion::cursor::Hide);
                    }
                },
                EntityType::Monster => {
                        println!("{goto}👿{hide}",
                            goto=termion::cursor::Goto(pos.x as u16, pos.y as u16),
                            hide=termion::cursor::Hide);
                },
                EntityType::Weapon => {
                    weapon_position = pos;
                    if let Some(weap) =  weapon {
                        println!("{goto}🔫{hide}",
                            goto=termion::cursor::Goto(pos.x as u16, pos.y as u16),                                hide=termion::cursor::Hide);
                    }
                },
                EntityType::Bullet => {
                        println!("{goto}💣{hide}",
                            goto=termion::cursor::Goto(pos.x as u16, pos.y as u16),
                            hide=termion::cursor::Hide);
                },
            }   
        }

        (user_position, weapon_position)
    }
}

