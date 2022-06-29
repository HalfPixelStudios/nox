use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use super::config::AppState;
use super::{
    assetloader::*,
    collision_group::*,
    component::{Damage, Displacement},
    physics::CollisionStartEvent,
    prefabs::{builder::*, *},
    utils::rotation_from_dir,
};

pub struct SpawnBulletEvent {
    pub bullet_id: String,
    pub attacker: Attacker,
    pub spawn_pos: Vec3,
    pub dir: Vec2,
}

#[derive(Component)]
pub struct Bullet {
    pub penetration: i32,
}

#[derive(Component)]
pub struct DistanceLifetime {
    max_distance: f32,
    pub displacement: Displacement,
}

#[derive(Component)]
pub struct DurationLifetime {
    timer: Timer,
}

impl DistanceLifetime {
    pub fn new(max_distance: f32) -> Self {
        DistanceLifetime {
            max_distance,
            displacement: Displacement::new(),
        }
    }
}
impl DurationLifetime {
    pub fn new(max_duration: f32) -> Self {
        DurationLifetime {
            timer: Timer::new(Duration::from_millis((max_duration * 1000.) as u64), false),
        }
    }
}

#[derive(Component)]
pub struct Movement(pub f32);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBulletEvent>()
            .add_system(bullet_distance_lifetime_system)
            .add_system(bullet_duration_lifetime_system)
            .add_system(handle_collision)
            .add_system(spawn_bullet_system)
            .add_system(bullet_die_system);
    }
}

#[derive(Clone)]
pub enum Attacker {
    Player,
    Enemy,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub damage: Damage,
    pub movement: Movement,
    pub rb: RigidBody,
    pub sensor: Sensor,
    pub col: Collider,
    pub active_events: ActiveEvents,
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            bullet: Bullet { penetration: 1 },
            damage: Damage(10),
            movement: Movement(500.),
            rb: RigidBody::Dynamic,
            sensor: Sensor(true),
            col: Collider::cuboid(1., 1.),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

pub fn attacker_collision_group(attacker: Attacker) -> CollisionGroups {
    match attacker {
        Attacker::Player => CollisionGroups::new(PLAYER_BULLET, ENEMY),
        Attacker::Enemy => CollisionGroups::new(ENEMY_BULLET, PLAYER),
    }
}

fn bullet_duration_lifetime_system(
    mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DurationLifetime), With<Bullet>>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            cmd.entity(entity).despawn();
        }
    }
}

fn bullet_distance_lifetime_system(
    mut cmd: Commands,
    mut query: Query<(Entity, &Transform, &mut DistanceLifetime), With<Bullet>>,
) {
    for (entity, transform, mut lifetime) in query.iter_mut() {
        if lifetime.displacement.get_distance() > lifetime.max_distance {
            bullet_die(&mut cmd, entity);
        }

        lifetime.displacement.update(transform.translation);
    }
}

fn spawn_bullet_system(
    mut cmds: Commands,
    mut events: EventReader<SpawnBulletEvent>,
    prefab_res: Res<PrefabResource>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for SpawnBulletEvent {
        bullet_id,
        attacker,
        spawn_pos,
        dir,
    } in events.iter()
    {
        let prefab = prefab_res.get_bullet(bullet_id);
        if prefab.is_none() {
            warn!("unable to fetch bullet prefab: {}", bullet_id);
            continue;
        }
        let prefab = prefab.unwrap();

        let e = bullet_builder(&mut cmds, prefab);

        cmds.entity(e)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index as usize,
                    color: Color::rgb(
                        prefab.sprite_color.0,
                        prefab.sprite_color.1,
                        prefab.sprite_color.2,
                    ),
                    ..default()
                },
                texture_atlas: get_tileset(&assets, &mut texture_atlases),
                transform: Transform {
                    translation: spawn_pos.clone(),
                    // TODO do proper rotation offset
                    rotation: rotation_from_dir(dir.clone(), 0.),
                    ..default()
                },
                ..default()
            })
            .insert(attacker_collision_group(attacker.clone()))
            .insert(Velocity {
                linvel: prefab.speed * dir.clone(),
                ..default()
            });
    }
}

fn bullet_die_system(mut cmd: Commands, query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in query.iter() {
        if bullet.penetration <= 0 {
            bullet_die(&mut cmd, entity);
        }
    }
}

fn bullet_die(cmd: &mut Commands, entity: Entity) {
    cmd.entity(entity).despawn();
}

fn handle_collision(mut query: Query<&mut Bullet>, mut events: EventReader<CollisionStartEvent>) {
    for CollisionStartEvent { me, other } in events.iter() {
        if let Ok(mut bullet) = query.get_component_mut::<Bullet>(*me) {
            bullet.penetration -= 1;
        }
    }
}
