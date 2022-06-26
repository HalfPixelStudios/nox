use super::enemy::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use super::{assetloader::get_tileset, component::*};

pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    MYTHIC,
}
impl Rarity {
    fn color(&self) -> Color {
        match *self {
            Rarity::COMMON => Color::OLIVE,
            Rarity::UNCOMMON => Color::SILVER,
            Rarity::RARE => Color::ORANGE,
            Rarity::MYTHIC => Color::RED,
        }
    }
    fn bonus(&self) -> i32 {
        match *self {
            Rarity::COMMON => 0,
            Rarity::UNCOMMON => 10,
            Rarity::RARE => 25,
            Rarity::MYTHIC => 60,
        }
    }
    fn new(val: i32) -> Rarity {
        if val == 0 {
            Rarity::COMMON
        } else if val == 1 {
            Rarity::UNCOMMON
        } else if val == 2 {
            Rarity::RARE
        } else {
            Rarity::MYTHIC
        }
    }
}
#[derive(Component)]
struct Soul;

#[derive(Bundle)]
struct SoulBundle {
    soul: Soul,
    #[bundle]
    sprite: SpriteSheetBundle,
    decay: Decay,
}
pub fn spawn_drop(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    drops: &Drops,
    spawn_pos: Vec3,
) {
    let mut rng = thread_rng();
    let c: f32 = rng.gen();
    let r = rng.gen_range(0..=3);

    let rarity = Rarity::new(r);
    // if c>drops.chance{
    //     return ;
    // }

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: drops.frame,
            color: rarity.color(),
            ..default()
        },
        texture_atlas: get_tileset(assets, atlases),

        transform: Transform {
            scale: Vec3::new(1.5, 1.5, 0.),
            translation: spawn_pos,
            ..default()
        },
        ..default()
    })
    .insert(Name::new(drops.name.clone()));
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
