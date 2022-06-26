use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

use super::{bullet::ShootFunction, *};

#[derive(Component)]
pub struct Weapon {
    pub name: String,
    pub attack_fn: ShootFunction,
    pub attack_speed: f32, // time between consecutive attacks
}
