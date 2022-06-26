use bevy::{core::Stopwatch, prelude::*};

use super::super::{component::Health, enemy::*, weapon::Weapon};
use super::weapon::*;

pub fn bow_orc(cmd: &mut Commands, spawn_pos: Vec2) {
    _orc(cmd, spawn_pos, wooden_bow())
}

fn _orc(cmd: &mut Commands, spawn_pos: Vec2, weapon: Weapon) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..default()
            },
            transform: Transform {
                translation: spawn_pos.extend(0.),
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        },
        drops: Drops {
            name: "bow".to_string(),
            frame: 282,
            souls: 2,
            chance: 0.2,
        },

        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 200.,
        weapon,
        attack_timer: Stopwatch::new(),
    })
    .insert(SimpleMovement {
        speed: 40.,
        target_range: 100.,
    });
    // .insert(LoiterMovement {
    //     speed: 40.,
    //     chaos: 20,
    //     current_dir: Vec2::ZERO,
    // });
}
