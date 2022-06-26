use bevy::{core::Stopwatch, prelude::*};

use super::super::{
    assetloader::get_tileset, audio::SoundEmitter, component::Health, enemy::*, weapon::Weapon,
};
use super::weapon::*;

pub fn bow_orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    _orc(cmd, assets, texture_atlases, spawn_pos, wooden_bow())
}

fn _orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
    weapon: Weapon,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 123,
                color: Color::rgb(0., 1., 0.),
                ..default()
            },
            texture_atlas: get_tileset(&assets, &mut texture_atlases),
            transform: Transform {
                translation: spawn_pos.extend(0.),
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
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["orc/hurt1.wav".to_string(), "orc/hurt2.wav".to_string()],
            die_sounds: vec!["orc/die1.wav".to_string(), "orc/die2.wav".to_string()],
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
}

pub fn bat(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 418,
                color: Color::rgb(0.2, 0.2, 0.2),
                ..default()
            },
            texture_atlas: get_tileset(&assets, &mut texture_atlases),
            transform: Transform {
                translation: spawn_pos.extend(0.),
                ..default()
            },
            ..default()
        },
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec![],
            die_sounds: vec![],
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 50.,
        weapon: poison_dagger(),
        attack_timer: Stopwatch::new(),
    })
    .insert(LoiterMovement {
        speed: 40.,
        chaos: 20,
        current_dir: Vec2::ZERO,
    });
}
