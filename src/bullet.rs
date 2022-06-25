use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use super::component::Damage;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct DistanceLifetime {
    distance_left: f32,
    previous_position: Vec3,
}

#[derive(Component)]
struct DurationLifetime {
    timer: Timer,
}

impl DistanceLifetime {
    fn new(max_distance: f32, start_position: Vec3) -> Self {
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
struct Movement(f32, Vec2);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_movement_system)
            .add_system(bullet_distance_lifetime_system)
            .add_system(bullet_duration_lifetime_system);
    }
}

pub fn spawn_player_bullet(cmd: &mut Commands, pos: Vec3, dir: Vec2) {
    cmd.spawn_bundle(SpriteBundle {
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
    })
    .insert(Bullet)
    .insert(Damage(10))
    .insert(Movement(500., dir))
    .insert(RigidBody::Dynamic)
    .insert(Sensor(true))
    .insert(Collider::cuboid(0.05, 0.01));
}

pub fn spawn_enemy_bullet(cmd: &mut Commands, pos: Vec3, dir: Vec2) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 1., 0.),
            ..default()
        },
        transform: Transform {
            translation: pos,
            scale: Vec3::new(10., 2., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Bullet)
    .insert(Damage(10))
    .insert(Movement(500., dir))
    .insert(RigidBody::Dynamic)
    .insert(Sensor(true))
    .insert(Collider::cuboid(0.05, 0.01))
    .insert(DistanceLifetime::new(200., pos));
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
            cmd.entity(entity).despawn();
        }
    }
}
