use bevy::{core::Stopwatch, prelude::*};
use bevy_rapier2d::prelude::*;

use super::{
    bullet::{spawn_enemy_bullet, Bullet},
    collision_group::*,
    component::{Damage, Health},
    player::Player,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct SimpleAI {
    speed: f32,
    target_range: f32, // the distance at which enemy will stop chasing player
    attack_range: f32, // min distance before attempting to attack
    shoot_speed: f32,  // amount of time between attacks (in seconds)
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(simple_enemy_movement_system)
            .add_system(simple_enemy_attack_system)
            .add_system(enemy_die_system)
            .add_system(handle_collision);
    }
}

#[derive(Bundle)]
struct SimpleEnemyBundle {
    enemy: Enemy,
    #[bundle]
    sprite: SpriteBundle,
    health: Health,
    ai: SimpleAI,
    rb: RigidBody,
    col: Collider,
    attack_timer: AttackTimer,
}

#[derive(Component)]
struct AttackTimer(Stopwatch);

fn setup(mut cmd: Commands) {
    spawn_simple_enemy(cmd, Vec2::new(50., 50.));
}

fn spawn_simple_enemy(mut cmd: Commands, spawn_pos: Vec2) {
    cmd.spawn_bundle(SimpleEnemyBundle {
        enemy: Enemy,
        sprite: SpriteBundle {
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
        },
        health: Health(20),
        ai: SimpleAI {
            speed: 40.,
            target_range: 100.,
            attack_range: 200.,
            shoot_speed: 1.,
        },
        rb: RigidBody::Dynamic,
        col: Collider::cuboid(0.5, 0.5),
        attack_timer: AttackTimer(Stopwatch::new()),
    })
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(CollisionGroups::new(ENEMY, PLAYER | PLAYER_BULLET));
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

fn simple_enemy_attack_system(
    mut cmd: Commands,
    time: Res<Time>,
    mut enemy_query: Query<
        (&Transform, &SimpleAI, &mut AttackTimer),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (transform, ai, mut attack_timer) in enemy_query.iter_mut() {
        attack_timer.0.tick(time.delta());

        let delta = player_transform.translation - transform.translation;
        if delta.length() < ai.attack_range && attack_timer.0.elapsed_secs() > ai.shoot_speed {
            attack_timer.0.reset();

            let bullet_dir = delta.truncate().normalize_or_zero();
            spawn_enemy_bullet(&mut cmd, transform.translation, bullet_dir);
        }
    }
}

fn enemy_die_system(mut cmd: Commands, query: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            enemy_die(&mut cmd, entity);
        }
    }
}

fn enemy_die(cmd: &mut Commands, entity: Entity) {
    cmd.entity(entity).despawn();
}

fn handle_collision(
    mut enemy_query: Query<(Entity, &mut Health), With<Enemy>>,
    bullet_query: Query<&Damage, With<Bullet>>,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = event {
            if let (Ok(mut health), Ok(damage)) = (
                enemy_query.get_component_mut::<Health>(*e1),
                bullet_query.get_component::<Damage>(*e2),
            ) {
                health.0 -= damage.0;
            } else if let (Ok(mut health), Ok(damage)) = (
                enemy_query.get_component_mut::<Health>(*e2),
                bullet_query.get_component::<Damage>(*e1),
            ) {
                health.0 -= damage.0;
            }
        }
    }
}
