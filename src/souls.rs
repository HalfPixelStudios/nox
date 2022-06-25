use super::component::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct Soul;

#[derive(Bundle)]
struct SoulBundle {
    soul: Soul,
    #[bundle]
    sprite: SpriteBundle,
    decay: Decay,
}

pub fn spawn_soul(cmd: &mut Commands, assets: &Res<AssetServer>, spawn_pos: Vec3) {
    cmd.spawn_bundle(SoulBundle {
        soul: Soul,
        sprite: SpriteBundle {
            texture: assets.load("soul.png"),

            transform: Transform {
                translation: spawn_pos,
                scale: Vec3::new(0.1, 0.1, 0.),
                ..default()
            },
            ..default()
        },

        decay: Decay {
            timer: Timer::new(Duration::from_secs(10), true),
        },
    });
}
