use itertools::izip;

/// System to adjust position
struct PositionSystem;

impl PositionSystem {
    fn run(mut world: World) {
        for (position, velocity) in izip!(&world.positions, &world.velocities { 
            if let Some(pos) = position {
                self.pos.0 += self.velocity.0;
                self.pos.1 += self.velocity.1;
            }
        }
    }
}


/// System to check for weapons
struct WeaponSystem;

impl WeaponSystem {
    pub fn try_pick_up(mut world: World, user_position: Position, weapon_position: Position) -> World {

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

/// System to render world to screen
struct RenderingSystem;

impl RenderingSystem {

    fn render_welcome() {
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

    fn render_world(&world: World) -> (Position, Position) {

        let mut user_position = Position{x:0, y: 0};
        let mut weapon_position = Position{x:0, y: 0};

        println!("{clear}{goto}{vis}{hide}",
            clear=termion::clear::All,
            goto=termion::cursor::Goto(1,1),
            vis=gamefield.new_gamefield,
            hide=termion::cursor::Hide);

        for (position, entitytype, weapon) in izip!(&world.positions, &world.entitytype,  &world.weapons) {
            if let Some(pos) = position {
                if let Some(etype) = entitytype {
                    EntityType::Human => {
                        user_position = pos;
                        if let Some(weap) =  weapon {
                            println!("{goto}🤠{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        } else {
                            println!("{goto}😃{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        }
                    },
                    EntityType::Monster => {
                            println!("{goto}👿{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                    },
                    EntityType::Weapon => {
                        weapon_position = pos;
                        if let Some(weap) =  weapon {
                            println!("{goto}🔫{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                        }
                    },
                    EntityType::Bullet => {
                            println!("{goto}💣{hide}",
                                goto=termion::cursor::Goto(pos.0, pos.1),
                                hide=termion::cursor::Hide);
                    },
                    None => {},
                }   
            } 
        }
    }
}
