use super::super::{bullet::Attacker, weapon::Weapon};
use super::{bullet, *};
use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

pub fn steel_sword() -> Weapon {
    Weapon {
        name: "steel sword".to_string(),
        bullet_id: "steel_sword_bullet".to_string(),
        attack_speed: 1.,
    }
}
