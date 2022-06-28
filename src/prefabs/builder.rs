use bevy::prelude::*;

use super::super::bullet::*;
use super::super::weapon::*;
use super::models::*;

pub fn weapon_builder(prefab: WeaponPrefab) {}

pub fn projectile_builder(cmds: &mut Commands, prefab: ProjectilePrefab) -> Entity {
    let e = cmds.spawn().id();

    cmds.entity(e).insert_bundle(BulletBundle {
        bullet: Bullet {
            penetration: prefab.penetration,
        },
        damage: Damage(prefab.damage),
        movement: Movement(prefab.speed, dir),
        ..default()
    });

    match prefab.lifetime {
        // TODO fix distance lifetime to not need spawn pos
        Lifetime::Distance(d) => {
            cmds.entity(e).insert(DistanceLifetime::new(d, Vec3::ZERO));
        }
        Lifetime::Duration(d) => {
            cmds.entity(e).insert(DurationLifetime::new(d));
        }
    };

    return e;
}
