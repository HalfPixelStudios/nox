use super::super::{
    assetloader::get_tileset,
    bullet::{
        attacker_collision_group, Attacker, Bullet, BulletBundle, DistanceLifetime, Movement,
    },
    component::Damage,
};
use bevy::prelude::*;

pub fn steel_sword_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 564,
                color: Color::rgb(1., 0., 1.),
                ..default()
            },
            texture_atlas: get_tileset(assets, texture_atlases),
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 1 },
        damage: Damage(10),
        movement: Movement(500., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(50., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn wooden_bow_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 285,
                color: Color::rgb(1., 0., 1.),
                ..default()
            },
            texture_atlas: get_tileset(assets, texture_atlases),
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 1 },
        damage: Damage(10),
        movement: Movement(300., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(200., pos))
    .insert(attacker_collision_group(attacker));
}
