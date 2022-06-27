use bevy::{
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    prelude::*,
};
use rand::{seq::SliceRandom, Rng};

use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use super::{
    animator::*,
    assetloader::get_tileset,
    audio::{PlaySoundEvent, SoundEmitter},
    bullet::{Attacker, Bullet},
    camera::{CameraFollow, Cursor},
    collision_group::*,
    component::{Damage, Health},
    config::AppState,
    inventory::InventoryResource,
    physics::{CollisionStartEvent, PhysicsBundle},
    souls::*,
};
use bevy_tweening::{lens::*, *};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_controller)
                    .with_system(player_attack)
                    .with_system(player_die),
            )
            .add_system(handle_collision)
            .add_system(eat_weapon);
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    player: Player,
    health: Health,
    movement: Movement,
    #[bundle]
    sprite: SpriteSheetBundle,
    #[bundle]
    physics: PhysicsBundle,
    collision_groups: CollisionGroups,
    sound_emitter: SoundEmitter,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            name: Name::new("Player"),
            player: Player,
            health: Health(100),
            movement: Movement { speed: 100. },
            sprite: SpriteSheetBundle::default(),
            physics: PhysicsBundle::default(),
            collision_groups: CollisionGroups::new(PLAYER, ENEMY | ENEMY_BULLET),
            sound_emitter: SoundEmitter::default(),
        }
    }
}

fn spawn_player(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tween = Tween::new(
        EaseFunction::SineInOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(500),
        TransformDimensionLens {
            start: 1.,
            end: 1.1,
            freeze_width: true,
            freeze_height: false,
        },
    );
    let rot_tween = Tween::new(
        EaseFunction::SineInOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(500),
        TransformRotationLens {
            start: Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 8.),
            end: Quat::from_axis_angle(Vec3::Z, -std::f32::consts::PI / 8.),
        },
    );

    cmd.spawn_bundle(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 25,
                ..default()
            },
            texture_atlas: get_tileset(&assets, &mut texture_atlases),
            transform: Transform { ..default() },
            ..default()
        },
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["player/hurt.wav".to_string()],
            die_sounds: vec!["player/die.wav".to_string()],
        },
        ..default()
    })
    .insert(EntityState {
        action: Action::IDLE,
        direction: Dir::RIGHT,
    })
    .insert(Animatable)
    .insert(CameraFollow)
    .insert(AnimationTimer(Timer::from_seconds(0.05, true)))
    .insert(Animator::new(rot_tween))
    .insert(CollisionGroups::new(PLAYER, ENEMY | ENEMY_BULLET))
    .insert(ActiveEvents::COLLISION_EVENTS);
}

fn player_controller(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Movement, &mut EntityState), With<Player>>,
) {
    let (mut transform, movement, mut state) = query.single_mut();

    let mut input_vec = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        input_vec += Vec2::Y;
    } else if input.pressed(KeyCode::S) {
        input_vec -= Vec2::Y;
    }
    if input.pressed(KeyCode::A) {
        input_vec -= Vec2::X;
        state.direction = Dir::LEFT;
    } else if input.pressed(KeyCode::D) {
        input_vec += Vec2::X;
        state.direction = Dir::RIGHT;
    }
    if input_vec.eq(&Vec2::ZERO) {
        state.action = Action::IDLE;
    } else {
        state.action = Action::WALK;
    }

    let move_vec = input_vec.normalize_or_zero();
    transform.translation += move_vec.extend(0.) * movement.speed * time.delta_seconds();
}

fn player_attack(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    inventory: Res<InventoryResource>,
    mut player_query: Query<&Transform, With<Player>>,
    mut writer: EventWriter<PlaySoundEvent>,
) {
    let player_trans = player_query.single_mut();

    if input.just_pressed(KeyCode::Space) {
        // TODO: should error if bullet direction is ever zero
        let bullet_direction = (cursor.0 - player_trans.translation.truncate()).normalize_or_zero();

        let shoot_fn = inventory.primary_weapon.attack_fn;
        shoot_fn(
            &mut cmd,
            &assets,
            &mut texture_atlases,
            Attacker::Player,
            player_trans.translation,
            bullet_direction,
        );

        // play attack sound
        if let Some(sound_file) = inventory
            .primary_weapon
            .attack_sounds
            .choose(&mut rand::thread_rng())
        {
            writer.send(PlaySoundEvent(sound_file.clone()));
        }
    }
}

fn eat_weapon(
    mut player_query: Query<&mut Health, With<Player>>,
    mut inventory: ResMut<InventoryResource>,
    mut writer: EventWriter<PlaySoundEvent>,
) {
    let mut health = player_query.single_mut();
    if !inventory.eaten {
        inventory.eaten = true;
        match inventory.eat_rarity {
            Rarity::COMMON => health.0 += 10,
            Rarity::UNCOMMON => health.0 += 50,
            Rarity::RARE => health.0 += 100,
            Rarity::MYTHIC => health.0 += 300,
        }

        // play sound
        let sounds = vec!["eat/eat1.wav", "eat/eat2.wav", "eat/eat3.wav"];
        let sfx = sounds.choose(&mut rand::thread_rng()).unwrap();
        writer.send(PlaySoundEvent(sfx.to_string()));
    }
}

fn player_die(mut app_state: ResMut<State<AppState>>, query: Query<&Health, With<Player>>) {
    let health = query.single();

    if health.0 <= 0 {
        app_state.set(AppState::GameOver).unwrap();
    }
}

fn handle_collision(
    mut health_query: Query<&mut Health, With<Player>>,
    sound_query: Query<&SoundEmitter, With<Player>>,
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

                // play sound
                if let Some(sound_file) = sound_emitter.pick_hurt_sound() {
                    writer.send(PlaySoundEvent(sound_file.clone()));
                }
            }
        }
    }
}
