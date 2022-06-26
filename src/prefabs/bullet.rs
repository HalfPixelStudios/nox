use super::super::{
    bullet::{
        attacker_collision_group, Attacker, Bullet, BulletBundle, DistanceLifetime, Movement,
    },
    component::Damage,
};
use bevy::prelude::*;

pub fn steel_sword_bullet(cmd: &mut Commands, attacker: Attacker, pos: Vec3, dir: Vec2) {
    cmd.spawn_bundle(BulletBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 1.),
                ..default()
            },
            transform: Transform {
                translation: pos,
                scale: Vec3::new(10., 2., 1.),
                ..default()
            },
            ..default()
        },
        bullet: Bullet { penetration: 1 },
        damage: Damage(10),
        movement: Movement(500., dir),
        ..default()
    })
    .insert(DistanceLifetime::new(200., pos))
    .insert(attacker_collision_group(attacker));
}
