use bevy::prelude::*;
use std::time::Duration;

use super::{assetloader::get_tileset, component::*};

#[derive(Component)]
struct Soul;

#[derive(Bundle)]
struct SoulBundle {
    soul: Soul,
    #[bundle]
    sprite: SpriteSheetBundle,
    decay: Decay,
}

pub fn spawn_soul(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec3,
) {
    cmd.spawn_bundle(SoulBundle {
        soul: Soul,
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 1056,
                ..default()
            },
            texture_atlas: get_tileset(&assets, texture_atlases),
            transform: Transform {
                translation: spawn_pos,
                ..default()
            },
            ..default()
        },

        decay: Decay {
            timer: Timer::new(Duration::from_secs(10), true),
        },
    });
}
