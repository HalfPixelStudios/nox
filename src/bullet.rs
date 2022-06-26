use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use super::{collision_group::*, component::Damage};

pub type ShootFunction =
    fn(cmd: &mut Commands, attacker: Attacker, spawn_pos: Vec3, dir: Vec2) -> ();

#[derive(Component)]
pub struct Bullet {
    pub penetration: i32,
}

#[derive(Component)]
pub struct DistanceLifetime {
    distance_left: f32,
    previous_position: Vec3,
}

#[derive(Component)]
pub struct DurationLifetime {
    timer: Timer,
}

impl DistanceLifetime {
    pub fn new(max_distance: f32, start_position: Vec3) -> Self {
        DistanceLifetime {
            distance_left: max_distance,
            previous_position: start_position,
        }
    }
}
impl DurationLifetime {
    fn new(max_duration: f32) -> Self {
        DurationLifetime {
            timer: Timer::new(Duration::from_millis((max_duration * 1000.) as u64), false),
        }
    }
}

#[derive(Component)]
pub struct Movement(pub f32, pub Vec2);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_movement_system)
            .add_system(bullet_distance_lifetime_system)
            .add_system(bullet_duration_lifetime_system)
            .add_system(handle_collision)
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
    #[bundle]
    pub sprite: SpriteBundle,
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
            sprite: SpriteBundle { ..default() },
            damage: Damage(10),
            movement: Movement(500., Vec2::ZERO),
            rb: RigidBody::Dynamic,
            sensor: Sensor(true),
            col: Collider::cuboid(0.05, 0.01),
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

fn bullet_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement), With<Bullet>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation += movement.0 * movement.1.extend(0.) * time.delta_seconds();
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
        lifetime.distance_left -= transform.translation.distance(lifetime.previous_position);
        lifetime.previous_position = transform.translation;

        if lifetime.distance_left < 0. {
            bullet_die(&mut cmd, entity);
        }
    }
}

fn bullet_die_system(mut cmd: Commands, mut query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in query.iter() {
        if bullet.penetration <= 0 {
            bullet_die(&mut cmd, entity);
        }
    }
}

fn bullet_die(cmd: &mut Commands, entity: Entity) {
    cmd.entity(entity).despawn();
}

fn handle_collision(mut query: Query<&mut Bullet>, mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = event {
            // TODO this code sucks
            if let Ok(mut bullet) = query.get_component_mut::<Bullet>(*e1) {
                bullet.penetration -= 1;
            } else if let Ok(mut bullet) = query.get_component_mut::<Bullet>(*e2) {
                bullet.penetration -= 1;
            }
        }
    }
}
