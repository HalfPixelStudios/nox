use bevy::{core::Stopwatch, math::Mat2, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::PI;
use std::time::Duration;

use super::{
    assetloader::*,
    audio::{PlaySoundEvent, SoundEmitter},
    bullet::{Attacker, Bullet, SpawnBulletEvent},
    collision_group::*,
    component::*,
    config::AppState,
    physics::{CollisionStartEvent, PhysicsBundle},
    player::Player,
    prefabs::{builder::enemy_builder, PrefabResource},
    souls::*,
    weapon::*,
};

pub struct SpawnEnemyEvent {
    pub enemy_id: String,
    pub spawn_pos: Vec2,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackPolicy {
    attack_range: f32, // min distance before attempting to attack
    weapon: String,
    attack_timer: Stopwatch,
}

impl AttackPolicy {
    pub fn new(attack_range: f32, weapon: String) -> Self {
        AttackPolicy {
            attack_range,
            weapon,
            attack_timer: Stopwatch::new(),
        }
    }
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

#[derive(Component)]
pub struct Drops {
    pub name: i32,
    pub frame: usize,
    pub chance: f32,
}
impl Default for Drops {
    fn default() -> Self {
        Drops {
            name: 0,
            frame: 0,
            chance: 0.,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(spawn_enemy_system)
                    .with_system(simple_movement_system)
                    .with_system(loiter_movement_system)
                    .with_system(attack_system),
            )
            .add_system(enemy_die_system)
            .add_system(handle_collision);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub drops: Drops,
    pub health: Health,
    #[bundle]
    pub physics: PhysicsBundle,
    pub collision_groups: CollisionGroups,
    pub sound_emitter: SoundEmitter,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            enemy: Enemy,
            health: Health(100),
            drops: Drops::default(),
            physics: PhysicsBundle::default(),
            collision_groups: CollisionGroups::new(ENEMY, PLAYER | PLAYER_BULLET | ENEMY),
            sound_emitter: SoundEmitter::default(),
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
    prefabs: Res<PrefabResource>,
    mut enemy_query: Query<(&Transform, &mut AttackPolicy), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut bullet_writer: EventWriter<SpawnBulletEvent>,
) {
    let player_transform = player_query.single();

    for (transform, mut ap) in enemy_query.iter_mut() {
        ap.attack_timer.tick(time.delta());

        let delta = player_transform.translation - transform.translation;

        // fetch enemy weapon (TOOD this is sorta disgusting)
        let weapon = prefabs.get_weapon(&ap.weapon);
        if weapon.is_none() {
            warn!("unable to fetch enemy weapon: {}", &ap.weapon);
        }
        let weapon = weapon.unwrap();

        if delta.length() < ap.attack_range && ap.attack_timer.elapsed_secs() > weapon.attack_speed
        {
            ap.attack_timer.reset();

            let bullet_dir = delta.truncate().normalize_or_zero();

            attack_pattern(
                &mut bullet_writer,
                weapon,
                Attacker::Enemy,
                transform.translation,
                bullet_dir,
            );
        }
    }
}

fn spawn_enemy_system(
    mut cmd: Commands,
    prefabs: Res<PrefabResource>,
    mut events: EventReader<SpawnEnemyEvent>,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for SpawnEnemyEvent {
        enemy_id,
        spawn_pos,
    } in events.iter()
    {
        let prefab = prefabs.get_enemy(enemy_id);
        if prefab.is_none() {
            warn!("unable to fetch enemy prefab: {}", enemy_id);
            continue;
        }
        let prefab = prefab.unwrap();

        let e = enemy_builder(&mut cmd, prefab);

        cmd.entity(e).insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 123,
                color: Color::rgb(0., 1., 0.),
                ..default()
            },
            texture_atlas: get_tileset(&assets, &mut texture_atlases),
            transform: Transform {
                translation: spawn_pos.extend(0.),
                ..default()
            },
            ..default()
        });
    }
}

fn enemy_die_system(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<
        (Entity, &Health, &Transform, &Drops, &SoundEmitter),
        (With<Enemy>, Without<Decay>),
    >,
    mut writer: EventWriter<PlaySoundEvent>,
) {
    for (entity, health, transform, drops, sound_emitter) in query.iter() {
        if health.0 <= 0 {
            spawn_drop(
                &mut cmd,
                &assets,
                &mut texture_atlases,
                &drops,
                transform.translation,
            );

            if let Some(sound_file) = sound_emitter.pick_die_sound() {
                writer.send(PlaySoundEvent(sound_file.clone()));
            }

            cmd.entity(entity).despawn();
        }
    }
}

fn handle_collision(
    mut health_query: Query<&mut Health, With<Enemy>>,
    sound_query: Query<&SoundEmitter, With<Enemy>>,
    bullet_query: Query<&Damage, With<Bullet>>,
    mut events: EventReader<CollisionStartEvent>,
    mut writer: EventWriter<PlaySoundEvent>,
) {
    for CollisionStartEvent { me, other } in events.iter() {
        if let (Ok(mut health), Ok(sound_emitter)) = (
            health_query.get_component_mut::<Health>(*me),
            sound_query.get_component::<SoundEmitter>(*me),
        ) {
            // hit by bullet
            if let Ok(damage) = bullet_query.get_component::<Damage>(*other) {
                health.take(damage.0);

                // play hit sound
                if let Some(sound_file) = sound_emitter.pick_hurt_sound() {
                    writer.send(PlaySoundEvent(sound_file.clone()));
                }
            }
        }
    }
}
