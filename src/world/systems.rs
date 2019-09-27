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

    fn render_world(&world: World) {

        println!("{clear}{goto}{vis}{hide}",
            clear=termion::clear::All,
            goto=termion::cursor::Goto(1,1),
            vis=gamefield.new_gamefield,
            hide=termion::cursor::Hide);

        for (position, entitytype, weapon) in izip!(&world.positions, &world.entitytype,  &world.weapons) {
            if let Some(pos) = position {
                if let Some(etype) = entitytype {
                    EntityType::Human => {
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
