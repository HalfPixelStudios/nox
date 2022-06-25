use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i8);

#[derive(Component)]
pub struct Damage(pub i8);

#[derive(Component)]
pub struct Displacement {
    previous_position: Vec3,
}
