use super::components::*;

#[derive(Clone)]
pub struct World {
    pub positions: Vec<Position>,
    pub velocities: Vec<Velocity>,
    pub entitytype: Vec<EntityType>,
    pub weapons: Vec<Option<Weapon>>,
}

impl World {

    // Construct new world
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            entitytype: Vec::new(),
            weapons: Vec::new(),
        }
    }

    // Register a new resource
    pub fn create_entity(&mut self, entitytype: EntityType, term_size: &(u16, u16)) {
        self.positions.push(Position::new(&(term_size.0 as i16, term_size.1 as i16)));
        self.velocities.push(Velocity { x: 0, y: 0 });
        self.entitytype.push(entitytype);
        self.weapons.push(None);
    }
}
