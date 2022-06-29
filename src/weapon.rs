use super::{
    bullet::{Attacker, SpawnBulletEvent},
    prefabs::models::*,
};
use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

pub fn attack_pattern(
    bullet_writer: &mut EventWriter<SpawnBulletEvent>,
    prefab: &WeaponPrefab,
    attacker: Attacker,
    spawn_pos: Vec3,
    dir: Vec2,
) {
    match prefab.shoot_pattern {
        ShootPattern::Straight => {
            bullet_writer.send(SpawnBulletEvent {
                bullet_id: prefab.projectile.clone(),
                attacker: attacker.clone(),
                spawn_pos,
                dir,
            });
        }
        ShootPattern::Shotgun { shots, angle } => {
            let offset_start = (shots as f32) * angle / 2.;
            for i in 0..shots {
                bullet_writer.send(SpawnBulletEvent {
                    bullet_id: prefab.projectile.clone(),
                    attacker: attacker.clone(),
                    spawn_pos,
                    dir: Mat2::from_angle(-offset_start + (i as f32) * angle) * dir,
                });
            }
        }
        ShootPattern::Around { shots } => {
            for i in 0..shots {
                bullet_writer.send(SpawnBulletEvent {
                    bullet_id: prefab.projectile.clone(),
                    attacker: attacker.clone(),
                    spawn_pos,
                    dir: Mat2::from_angle((i as f32) * 2. * PI / (shots as f32)) * dir,
                });
            }
        }
    };
}
