use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

use super::bullet::Attacker;

// single straight shot
#[macro_export]
macro_rules! straight {
    ( $shoot_fn:expr ) => {
        |cmd: &mut Commands, attacker: bullet::Attacker, spawn_pos: Vec3, dir: Vec2| {
            $shoot_fn(cmd, attacker, spawn_pos, dir);
        }
    };
}

// shotgun
#[macro_export]
macro_rules! shotgun {
    ( $shoot_fn:expr, $shot_count:expr, $angle:expr ) => {
        |cmd: &mut Commands, attacker: bullet::Attacker, spawn_pos: Vec3, dir: Vec2| {
            let offset_start = ($shot_count as f32) * $angle / 2.;
            for i in 0..$shot_count {
                $shoot_fn(
                    cmd,
                    attacker.clone(),
                    spawn_pos,
                    Mat2::from_angle(-offset_start + (i as f32) * $angle) * dir,
                );
            }
        }
    };
}

// shoot all around
#[macro_export]
macro_rules! around {
    ( $shoot_fn:expr, $spawn_pos:expr, $shot_count:expr ) => {
        move |cmd: &mut Commands, attacker: bullet::Attacker, spawn_pos: Vec3, dir: Vec2| {
            for i in 0..$shot_count {
                $shoot_fn(
                    cmd,
                    attacker.clone(),
                    $spawn_pos,
                    Mat2::from_angle((i as f32) * 2. * PI / ($shot_count as f32)) * dir,
                );
            }
        }
    };
}
