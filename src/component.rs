use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct Damage(pub u8);

#[derive(Component)]
pub struct Displacement {
    previous_position: Vec3,
}
