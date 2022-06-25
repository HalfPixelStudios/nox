use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

use super::bullet::spawn_player_bullet;

type AttackFunction = fn(cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2) -> ();

#[derive(Component)]
pub struct Weapon {
    pub name: String,
    pub attack_fn: AttackFunction,
    pub attack_speed: f32, // time between consecutive attacks
}

pub fn steel_sword_prefab() -> Weapon {
    Weapon {
        name: "steel sword".to_string(),
        attack_fn: steel_sword_attack,
        attack_speed: 1.,
    }
}
fn steel_sword_attack(cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2) {
    spawn_player_bullet(cmd, spawn_pos, Mat2::from_angle(-PI / 4.) * dir);
    spawn_player_bullet(cmd, spawn_pos, dir);
    spawn_player_bullet(cmd, spawn_pos, Mat2::from_angle(PI / 4.) * dir);
}

pub fn wooden_bow_prefab() -> Weapon {
    Weapon {
        name: "wooden bow".to_string(),
        attack_fn: spawn_player_bullet,
        attack_speed: 1.,
    }
}
