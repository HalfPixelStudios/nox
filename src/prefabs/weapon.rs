use super::super::{bullet::Attacker, weapon::Weapon};
use super::{bullet, *};
use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

// single straight shot
macro_rules! straight {
    ( $shoot_fn:expr ) => {
        |cmd: &mut Commands,
         assets: &Res<AssetServer>,
         texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
         attacker: Attacker,
         spawn_pos: Vec3,
         dir: Vec2| {
            $shoot_fn(cmd, assets, texture_atlases, attacker, spawn_pos, dir);
        }
    };
}

// shotgun
macro_rules! shotgun {
    ( $shoot_fn:expr, $shot_count:expr, $angle:expr ) => {
        |cmd: &mut Commands,
         assets: &Res<AssetServer>,
         texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
         attacker: Attacker,
         spawn_pos: Vec3,
         dir: Vec2| {
            let offset_start = ($shot_count as f32) * $angle / 2.;
            for i in 0..$shot_count {
                $shoot_fn(
                    cmd,
                    assets,
                    texture_atlases,
                    attacker.clone(),
                    spawn_pos,
                    Mat2::from_angle(-offset_start + (i as f32) * $angle) * dir,
                );
            }
        }
    };
}

// shoot all around
macro_rules! around {
    ( $shoot_fn:expr, $spawn_pos:expr, $shot_count:expr ) => {
        move |cmd: &mut Commands, assets: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>, bullet::Attacker, spawn_pos: Vec3, dir: Vec2| {
            for i in 0..$shot_count {
                $shoot_fn(
                    cmd,
                    assets,
                    texture_atlases
                    attacker.clone(),
                    $spawn_pos,
                    Mat2::from_angle((i as f32) * 2. * PI / ($shot_count as f32)) * dir,
                );
            }
        }
    };
}

pub fn steel_sword() -> Weapon {
    Weapon {
        name: "steel sword".to_string(),
        attack_fn: shotgun!(bullet::steel_sword_bullet, 3, PI / 4.),
        attack_speed: 1.,
    }
}

pub fn wooden_bow() -> Weapon {
    Weapon {
        name: "wooden bow".to_string(),
        attack_fn: straight!(bullet::wooden_bow_bullet),
        attack_speed: 3.,
    }
}
