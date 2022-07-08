use bevy::prelude::*;

use super::models::*;
use crate::audio::SoundEmitter;
use crate::bullet::*;
use crate::component::*;
use crate::enemy::*;

pub fn weapon_builder(prefab: WeaponPrefab) {}

pub fn bullet_builder(cmd: &mut Commands, prefab: &BulletPrefab) -> Entity {
    let e = cmd.spawn().id();

    cmd.entity(e).insert_bundle(BulletBundle {
        bullet: Bullet {
            penetration: prefab.penetration as i32,
        },
        damage: Damage(prefab.damage as i32),
        movement: Movement(prefab.speed),
        ..default()
    });

    match prefab.lifetime {
        // TODO fix distance lifetime to not need spawn pos
        Lifetime::Distance(d) => {
            cmd.entity(e).insert(DistanceLifetime::new(d));
        }
        Lifetime::Duration(d) => {
            cmd.entity(e).insert(DurationLifetime::new(d));
        }
    };

    return e;
}

pub fn enemy_builder(cmd: &mut Commands, prefab: &EnemyPrefab) -> Entity {
    let e = cmd.spawn().id();

    cmd.entity(e)
        .insert_bundle(EnemyBundle {
            health: Health(prefab.health as i32),
            sound_emitter: SoundEmitter {
                hurt_sounds: prefab.hurt_sounds.clone(),
                die_sounds: prefab.die_sounds.clone(),
            },
            ..default()
        })
        .insert(AttackPolicy::new(
            prefab.attack_range,
            prefab.weapon.clone(),
        ));

    match prefab.ai {
        AI::Simple { target_range } => {
            cmd.entity(e).insert(SimpleMovement {
                speed: prefab.speed,
                target_range,
            });
        }
        AI::Loiter { chaos } => {
            cmd.entity(e).insert(LoiterMovement {
                speed: prefab.speed,
                chaos,
                current_dir: Vec2::ZERO, //TODO exposing current dir is a bit ugly
            });
        }
    }

    return e;
}
