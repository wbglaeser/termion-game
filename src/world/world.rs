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
    fn create_entity(mut self, 
                     position: Position, 
                     velocity: Velocity,
                     entitytype: EntityType,
                     weapon: Option(Weapon)) {
        self.positions.push(position);
        self.velocity.push(position);
        self.entitytype.push(position);
        self.weapons.push(weapon);
    }
}
