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
    prefabs::enemy::bow_orc,
    souls::*,
    weapon::Weapon,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackPolicy {
    pub attack_range: f32, // min distance before attempting to attack
    pub weapon: Weapon,
    pub attack_timer: Stopwatch,
}

#[derive(Component)]
pub struct SimpleMovement {
    pub speed: f32,
    pub target_range: f32, // the distance at which enemy will stop chasing player
}

// ai that just wanders aimlessly around on the spot
#[derive(Component)]
pub struct LoiterMovement {
    pub speed: f32,
    pub chaos: u32, // how often changes direction
    pub current_dir: Vec2,
}

// circles around target
#[derive(Component)]
pub struct CircleMovement {}

// dashes straight towards target
#[derive(Component)]
pub struct ChargeMovement {}

#[derive(Component, Default)]
pub struct Drops {
    pub name: String,
    pub frame: usize,
    pub souls: i32,
    pub chance: f32,
}
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
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub drops: Drops,

    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub health: Health,
    pub rb: RigidBody,
    pub col: Collider,
    pub active_events: ActiveEvents,
    pub collision_groups: CollisionGroups,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            enemy: Enemy,
            sprite: SpriteSheetBundle { ..default() },
            health: Health(100),
            rb: RigidBody::Dynamic,
            col: Collider::cuboid(5., 5.),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_groups: CollisionGroups::new(ENEMY, PLAYER | PLAYER_BULLET),
            drops: Drops { ..default() },
        }
    }
}

fn setup(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // bow_orc(&mut cmd, assets, texture_atlases, Vec2::ZERO);
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
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut enemy_query: Query<(&Transform, &mut AttackPolicy), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (transform, mut ap) in enemy_query.iter_mut() {
        ap.attack_timer.tick(time.delta());

        let delta = player_transform.translation - transform.translation;
        if delta.length() < ap.attack_range
            && ap.attack_timer.elapsed_secs() > ap.weapon.attack_speed
        {
            ap.attack_timer.reset();

            let bullet_dir = delta.truncate().normalize_or_zero();
            (ap.weapon.attack_fn)(
                &mut cmd,
                &assets,
                &mut texture_atlases,
                Attacker::Enemy,
                transform.translation,
                bullet_dir,
            );
        }
    }
}

fn enemy_die_system(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<(Entity, &Health, &Transform, &Drops), (With<Enemy>, Without<Decay>)>,
) {
    for (entity, health, transform, drops) in query.iter() {
        if health.0 <= 0 {
            spawn_soul(
                &mut cmd,
                &assets,
                &mut texture_atlases,
                transform.translation,
            );
            spawn_drop(
                &mut cmd,
                &assets,
                &mut texture_atlases,
                &drops,
                transform.translation,
            );

            /*
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
            */
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
