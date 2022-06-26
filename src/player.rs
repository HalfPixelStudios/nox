use bevy::{
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    prelude::*,
};

use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use super::{
    animator::*,
    assetloader::get_tileset,
    bullet::{Attacker, Bullet},
    camera::{CameraFollow, Cursor},
    collision_group::*,
    component::{Damage, Health},
    config::AppState,
    inventory::InventoryResource,
    physics::{CollisionStartEvent, PhysicsBundle},
    utils::find_collider,
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
            .add_system(player_controller)
            .add_system(player_attack)
            .add_system(player_switch_weapon)
            .add_system(handle_collision);
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
        ..default()
    })
    .insert(EntityState {
        action: Action::IDLE,
        direction: Dir::RIGHT,
    })
    .insert(Animatable)
    .insert(CameraFollow)
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(Animator::new(rot_tween));
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
) {
    let player_trans = player_query.single_mut();

    if input.just_pressed(KeyCode::Space) {
        // TODO: should error if bullet direction is ever zero
        let bullet_direction = (cursor.0 - player_trans.translation.truncate()).normalize_or_zero();

        let current_weapon = inventory.current_weapon();
        let shoot_fn = current_weapon.attack_fn;
        shoot_fn(
            &mut cmd,
            &assets,
            &mut texture_atlases,
            Attacker::Player,
            player_trans.translation,
            bullet_direction,
        );
    }
}

fn player_switch_weapon(mut inventory: ResMut<InventoryResource>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Key1) {
        inventory.equip_primary();
    }
    if input.just_pressed(KeyCode::Key2) {
        inventory.equip_secondary();
    }
}

fn handle_collision(
    mut player_query: Query<(Entity, &mut Health), With<Player>>,
    bullet_query: Query<&Damage, With<Bullet>>,
    mut events: EventReader<CollisionStartEvent>,
) {
    for CollisionStartEvent { me, other } in events.iter() {
        if let Ok(mut health) = player_query.get_component_mut::<Health>(*me) {
            // hit by bullet
            if let Ok(damage) = bullet_query.get_component::<Damage>(*other) {
                health.take(damage.0);
            }
        }
    }
}
