use bevy::prelude::*;
use std::f32::consts::PI;

use super::super::{
    assetloader::get_tileset,
    bullet::{
        attacker_collision_group, Attacker, Bullet, BulletBundle, DistanceLifetime,
        DurationLifetime, Movement,
    },
    component::Damage,
};

/*
pub fn steel_sword_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = -PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 564,
                ..default()
            },
            texture_atlas: get_tileset(assets, texture_atlases),
            transform: Transform {
                translation: pos,
                rotation: rotation_from_dir(dir, ROTATION_OFFSET),
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 1 },
        damage: Damage(10),
        movement: Movement(200., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(30., pos))
    .insert(attacker_collision_group(attacker));
}
*/
