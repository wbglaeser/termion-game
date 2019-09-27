use std::Hashmap;

pub struct World {
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
    entitytype: Vec<EntityType>,
    weapons: Vec<Option(Weapon)>,
}

impl World {

    /// Construct new world
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            entitytype: Vec::new(),
            weapons: Vec::new(),
        }
    };

    /// Register a new resource
    fn create_entity(mut self, entitytype: EntityType, &term_size: (u16, u16)) {
        self.positions.push(Position::new(term_size));
        self.velocity.push(Velocity { x: 0, y: 0 };
        self.entitytype.push(entitytype);
        self.weapons.push(None);
    }
}
