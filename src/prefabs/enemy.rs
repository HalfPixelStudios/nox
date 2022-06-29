use bevy::{core::Stopwatch, prelude::*};

use super::super::{assetloader::get_tileset, audio::SoundEmitter, component::Health, enemy::*};
use super::bullet::*;

pub fn bow_orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        drops: Drops { ..default() },

        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["orc/hurt1.wav".to_string(), "orc/hurt2.wav".to_string()],
            die_sounds: vec!["orc/die1.wav".to_string(), "orc/die2.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
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
    })
    .insert(AttackPolicy::new(200., "steel_sword".to_string()))
    .insert(SimpleMovement {
        speed: 40.,
        target_range: 100.,
    });
}
