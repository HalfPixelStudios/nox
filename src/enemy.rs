use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::component::Health;
use super::player::Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct SimpleAI {
    pub speed: f32,
    pub target_range: f32, // the distance at which enemy will stop chasing player
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(simple_enemy_movement_system);
    }
}

fn setup(mut cmd: Commands) {
    spawn_simple_enemy(cmd, Vec2::new(50., 50.));
}

fn spawn_simple_enemy(mut cmd: Commands, spawn_pos: Vec2) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 1., 0.),
            ..default()
        },
        transform: Transform {
            translation: spawn_pos.extend(0.),
            scale: Vec3::new(10., 10., 10.),
            ..default()
        },
        ..default()
    })
    .insert(Enemy)
    .insert(Health(100))
    .insert(SimpleAI {
        speed: 40.,
        target_range: 100.,
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5));
}

fn simple_enemy_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &SimpleAI), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (mut transform, ai) in enemy_query.iter_mut() {
        let delta = player_transform.translation - transform.translation;

        if delta.length() < ai.target_range {
            continue;
        }

        let direction = delta.truncate().normalize_or_zero().extend(0.);
        transform.translation += ai.speed * direction * time.delta_seconds();
    }
}
