use bevy::prelude::*;

type SpawnFunction = fn(cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2) -> ();

#[derive(Component)]
struct Weapon {
    name: String,
    projecticle: SpawnFunction,
    attack_speed: f32,
}
