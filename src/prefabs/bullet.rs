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

fn rotation_from_dir(dir: Vec2, offset: f32) -> Quat {
    Quat::from_rotation_z(Vec2::X.angle_between(dir) + offset)
}

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

pub fn steel_greatsword_bullet(
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
                scale: Vec3::splat(2.),
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 1 },
        damage: Damage(30),
        movement: Movement(150., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(20., pos))
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
    const ROTATION_OFFSET: f32 = -PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 285,
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
    .insert(DistanceLifetime::new(125., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn flamethrower_staff_bolt(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 5. * PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 568,
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
        movement: Movement(50., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(100., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn posion_dagger_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 5. * PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 563,
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
        damage: Damage(15),
        movement: Movement(50., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(30., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn royal_hammer_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 0.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 616,
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
        damage: Damage(50),
        movement: Movement(200., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(15., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn tome_of_zeus_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 5. * PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 563,
                ..default()
            },
            texture_atlas: get_tileset(assets, texture_atlases),
            transform: Transform {
                translation: pos,
                rotation: rotation_from_dir(dir, ROTATION_OFFSET),
                scale: Vec3::splat(3.),
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 9999 },
        damage: Damage(200),
        movement: Movement(1000., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(1000., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn grand_sphinx_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 5. * PI / 4.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 563,
                ..default()
            },
            texture_atlas: get_tileset(assets, texture_atlases),
            transform: Transform {
                translation: pos,
                rotation: rotation_from_dir(dir, ROTATION_OFFSET),
                scale: Vec3::splat(3.),
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 9999 },
        damage: Damage(70),
        movement: Movement(50., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(1000., pos))
    .insert(attacker_collision_group(attacker));
}

pub fn orbs_of_despair_bullet(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    attacker: Attacker,
    pos: Vec3,
    dir: Vec2,
) {
    const ROTATION_OFFSET: f32 = 0.;
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 631,
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
        damage: Damage(50),
        movement: Movement(5., dir),
        ..default()
    })
    .insert(DurationLifetime::new(5.))
    .insert(attacker_collision_group(attacker));
}
