use itertools::izip;

/// System to adjust position
struct PositionSystem;

impl PositionSystem {
    fn run(mut world: World) {
        for (position, velocity) in world.positions.iter().zip(world.velocities.iter()) { 
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
    fn render(&world: World) {
        for (position, entitytype, weapon) in izip!(&world.positions, &world.entitytype,  &world.weapons) {
            if let Some(pos) = position {
                EntityType::Human => {},
                EntityType::Monster => {},
                EntityType::Weapon => {},
                EntityType::Bullet => {},
                None => {},
            } 
        }
    }
}
