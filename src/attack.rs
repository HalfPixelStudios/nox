use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

use super::bullet::ShootFunction;

// single straight shot
pub fn straight(shoot_fn: ShootFunction) -> impl Fn(&mut Commands, Vec3, Vec2) {
    move |cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2| {
        shoot_fn(cmd, spawn_pos, dir);
    }
}

// shotgun
pub fn shotgun(
    shoot_fn: ShootFunction,
    shot_count: u8,
    angle: f32,
) -> impl Fn(&mut Commands, Vec3, Vec2) {
    move |cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2| {
        let offset_start = (shot_count as f32) * angle / 2.;
        for i in 0..shot_count {
            shoot_fn(
                cmd,
                spawn_pos,
                Mat2::from_angle(-offset_start + (i as f32) * angle) * dir,
            );
        }
    }
}

// shoot all around
pub fn around(
    shoot_fn: ShootFunction,
    spawn_pos: Vec3,
    shot_count: u8,
) -> impl Fn(&mut Commands, Vec3, Vec2) {
    move |cmd: &mut Commands, spawn_pos: Vec3, dir: Vec2| {
        for i in 0..shot_count {
            shoot_fn(
                cmd,
                spawn_pos,
                Mat2::from_angle((i as f32) * 2. * PI / (shot_count as f32)) * dir,
            );
        }
    }
}
