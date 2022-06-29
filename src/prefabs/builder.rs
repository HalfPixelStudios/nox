use bevy::prelude::*;

use super::super::bullet::*;
use super::super::component::*;
use super::super::weapon::*;
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
