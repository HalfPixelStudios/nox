use bevy::{core::Stopwatch, math::Mat2, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

use super::{
    bullet::{Attacker, Bullet},
    collision_group::*,
    component::*,
    player::Player,
    souls::*,
    weapon::{wooden_bow_prefab, Weapon},
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackPolicy {
    attack_range: f32, // min distance before attempting to attack
    shoot_speed: f32,  // amount of time between attacks (in seconds)
    weapon: Weapon,
}

#[derive(Component)]
struct SimpleMovement {
    speed: f32,
    target_range: f32, // the distance at which enemy will stop chasing player
}

// ai that just wanders aimlessly around on the spot
#[derive(Component)]
struct LoiterMovement {
    speed: f32,
    chaos: u32, // how often changes direction
    current_dir: Vec2,
}

// circles around target
#[derive(Component)]
struct CircleMovement {}

// dashes straight towards target
#[derive(Component)]
struct ChargeMovement {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(simple_movement_system)
            .add_system(loiter_movement_system)
            .add_system(attack_system)
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
    ap: AttackPolicy,
    rb: RigidBody,
    col: Collider,
    attack_timer: AttackTimer,
}

#[derive(Component)]
struct AttackTimer(Stopwatch);

fn setup(mut cmd: Commands) {
    spawn_simple_enemy(&mut cmd, Vec2::new(50., 50.));
}

pub fn spawn_simple_enemy(cmd: &mut Commands, spawn_pos: Vec2) {
    _spawn_simple_enemy(cmd, spawn_pos, Color::rgb(0., 1., 0.));
}
pub fn spawn_simple_enemy_strong(cmd: &mut Commands, spawn_pos: Vec2) {
    _spawn_simple_enemy(cmd, spawn_pos, Color::rgb(0., 1., 1.));
}

fn _spawn_simple_enemy(cmd: &mut Commands, spawn_pos: Vec2, color: Color) {
    cmd.spawn_bundle(SimpleEnemyBundle {
        enemy: Enemy,
        sprite: SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation: spawn_pos.extend(0.),
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        },
        health: Health(20),
        ap: AttackPolicy {
            attack_range: 200.,
            shoot_speed: 1.,
            weapon: wooden_bow_prefab(),
        },
        rb: RigidBody::Dynamic,
        col: Collider::cuboid(0.5, 0.5),
        attack_timer: AttackTimer(Stopwatch::new()),
    })
    // .insert(
    //     SimpleMovement {
    //         speed: 40.,
    //         target_range: 100.,
    //     }
    // )
    .insert(LoiterMovement {
        speed: 40.,
        chaos: 20,
        current_dir: Vec2::ZERO,
    })
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(CollisionGroups::new(ENEMY, PLAYER | PLAYER_BULLET));
}

fn simple_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &SimpleMovement), (With<Enemy>, Without<Player>)>,
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

fn loiter_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut LoiterMovement), With<Enemy>>,
) {
    for (mut transform, mut ai) in enemy_query.iter_mut() {
        // randomly switch direction
        if rand::thread_rng().gen_range(0..ai.chaos) == 0 {
            let angle: i32 = rand::thread_rng().gen_range(0..360);
            let new_dir = Mat2::from_angle((angle as f32) * PI / 180.) * Vec2::new(1., 0.);
            ai.current_dir = new_dir;
        }

        transform.translation += ai.speed * ai.current_dir.extend(0.) * time.delta_seconds();
    }
}

fn attack_system(
    mut cmd: Commands,
    time: Res<Time>,
    mut enemy_query: Query<
        (&Transform, &AttackPolicy, &mut AttackTimer),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (transform, ap, mut attack_timer) in enemy_query.iter_mut() {
        attack_timer.0.tick(time.delta());

        let delta = player_transform.translation - transform.translation;
        if delta.length() < ap.attack_range && attack_timer.0.elapsed_secs() > ap.shoot_speed {
            attack_timer.0.reset();

            let bullet_dir = delta.truncate().normalize_or_zero();
            (ap.weapon.attack_fn)(&mut cmd, Attacker::Enemy, transform.translation, bullet_dir);
        }
    }
}

fn enemy_die_system(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &Sprite, &Health, &Transform), (With<Enemy>, Without<Decay>)>,
) {
    for (entity, sprite, health, transform) in query.iter() {
        if health.0 <= 0 {
            spawn_soul(
                &mut cmd,
                &assets,
                &mut texture_atlases,
                transform.translation,
            );
            cmd.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: sprite.color,
                    ..default()
                },
                transform: Transform {
                    translation: transform.translation,
                    scale: transform.scale,
                    rotation: transform.rotation,
                },
                ..default()
            })
            .insert(Decay {
                timer: Timer::new(Duration::from_secs(3), true),
            });
            cmd.entity(entity).despawn();
        }
    }
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
