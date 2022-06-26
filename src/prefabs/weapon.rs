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
    ( $shoot_fn:expr, $shot_count:expr ) => {
        move |cmd: &mut Commands,
              assets: &Res<AssetServer>,
              texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
              attacker: Attacker,
              spawn_pos: Vec3,
              dir: Vec2| {
            for i in 0..$shot_count {
                $shoot_fn(
                    cmd,
                    assets,
                    texture_atlases,
                    attacker.clone(),
                    spawn_pos,
                    Mat2::from_angle((i as f32) * 2. * PI / ($shot_count as f32)) * dir,
                );
            }
        }
    };
}

pub fn steel_sword() -> Weapon {
    Weapon {
        name: "steel sword".to_string(),
        attack_fn: straight!(bullet::steel_sword_bullet),
        attack_speed: 1.,
        attack_sounds: vec![
            "steel_sword/attack1.wav".to_string(),
            "steel_sword/attack2.wav".to_string(),
        ],
    }
}

pub fn steel_greatsword() -> Weapon {
    Weapon {
        name: "steel greatsword".to_string(),
        attack_fn: straight!(bullet::steel_greatsword_bullet),
        attack_speed: 2.,
        attack_sounds: vec![
            "steel_sword/attack1.wav".to_string(),
            "steel_sword/attack2.wav".to_string(),
        ],
    }
}

pub fn wooden_bow() -> Weapon {
    Weapon {
        name: "wooden bow".to_string(),
        attack_fn: straight!(bullet::wooden_bow_bullet),
        attack_speed: 3.,
        attack_sounds: vec!["wooden_bow/attack1.wav".to_string()],
    }
}

pub fn flamethrower_staff() -> Weapon {
    Weapon {
        name: "flamethrower staff".to_string(),
        attack_fn: straight!(bullet::flamethrower_staff_bolt),
        attack_speed: 0.5,
        attack_sounds: vec![],
    }
}

pub fn poison_dagger() -> Weapon {
    Weapon {
        name: "poison_dagger".to_string(),
        attack_fn: straight!(bullet::posion_dagger_bullet),
        attack_speed: 0.75,
        attack_sounds: vec![],
    }
}

pub fn tome_of_doom() -> Weapon {
    Weapon {
        name: "tome of doom".to_string(),
        attack_fn: around!(bullet::flamethrower_staff_bolt, 8),
        attack_speed: 1.,
        attack_sounds: vec![],
    }
}

pub fn royal_hammer() -> Weapon {
    Weapon {
        name: "royal hammer".to_string(),
        attack_fn: shotgun!(bullet::royal_hammer_bullet, 3, PI / 4.),
        attack_speed: 3.,
        attack_sounds: vec!["royal_hammer/attack1.wav".to_string()],
    }
}

pub fn tome_of_zeus() -> Weapon {
    Weapon {
        name: "tome of zeus".to_string(),
        attack_fn: straight!(bullet::tome_of_zeus_bullet),
        attack_speed: 10.,
        attack_sounds: vec!["tome_of_zeus/attack1.wav".to_string()],
    }
}

pub fn orbs_of_despair() -> Weapon {
    Weapon {
        name: "orbs of despair".to_string(),
        attack_fn: straight!(bullet::orbs_of_despair_bullet),
        attack_speed: 10.,
        attack_sounds: vec![],
    }
}

// enemy weapons
pub fn grand_sphinx_attack() -> Weapon {
    Weapon {
        name: "".to_string(),
        attack_fn: shotgun!(bullet::grand_sphinx_bullet, 5, PI / 8.),
        attack_speed: 3.,
        attack_sounds: vec![],
    }
}
