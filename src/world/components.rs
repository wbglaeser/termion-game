#[derive(Debug)]
pub(crate) struct Position {
    x,
    y,
}

#[derive(Debug)]
pub(crate) struct Velocity {
    x,
    y,
}

pub(crate) enum EntityType {
    Human,
    Monster,
    Weapon,
    Bullet,
}

pub(crate) struct Weapon;
