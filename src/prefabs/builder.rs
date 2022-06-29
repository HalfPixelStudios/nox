use bevy::prelude::*;

use super::super::bullet::*;
use super::super::component::*;
use super::super::enemy::*;
use super::models::*;

pub fn weapon_builder(prefab: WeaponPrefab) {}

pub fn bullet_builder(cmds: &mut Commands, prefab: &BulletPrefab) -> Entity {
    let e = cmds.spawn().id();

    cmds.entity(e).insert_bundle(BulletBundle {
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
            cmds.entity(e).insert(DistanceLifetime::new(d));
        }
        Lifetime::Duration(d) => {
            cmds.entity(e).insert(DurationLifetime::new(d));
        }
    };

    return e;
}

pub fn enemy_builder(cmds: &mut Commands, prefab: &EnemyPrefab) -> Entity {
    let e = cmds.spawn().id();

    cmds.entity(e)
        .insert_bundle(EnemyBundle {
            health: Health(prefab.health as i32),
            ..default()
        })
        .insert(AttackPolicy::new(200., "steel_sword".to_string()));

    match prefab.ai {
        AI::Simple { target_range } => {
            cmds.entity(e).insert(SimpleMovement {
                speed: prefab.speed,
                target_range,
            });
        }
        AI::Loiter { chaos } => {
            cmds.entity(e).insert(LoiterMovement {
                speed: prefab.speed,
                chaos,
                current_dir: Vec2::ZERO, //TODO exposing current dir is a bit ugly
            });
        }
        _ => {}
    }

    return e;
}
