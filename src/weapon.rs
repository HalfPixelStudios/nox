use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

use super::{bullet::ShootFunction, *};

#[derive(Component)]
pub struct Weapon {
    pub name: String,
    pub attack_fn: Fn(&mut Commands, Vec3, Vec2),
    pub attack_speed: f32, // time between consecutive attacks
}

pub fn steel_sword_prefab() -> Weapon {
    Weapon {
        name: "steel sword".to_string(),
        attack_fn: attack::shotgun(bullet::player_bullet, 3, PI / 4.),
        attack_speed: 1.,
    }
}

pub fn wooden_bow_prefab() -> Weapon {
    Weapon {
        name: "wooden bow".to_string(),
        attack_fn: attack::straight(bullet::player_bullet),
        attack_speed: 1.,
    }
}
