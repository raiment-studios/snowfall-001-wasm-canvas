use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}
