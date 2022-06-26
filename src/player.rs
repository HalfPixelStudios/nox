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

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 25,
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform { ..default() },
        ..default()
    })
    .insert(Name::new("Player"))
    .insert(Player)
    .insert(Health(100))
    .insert(Movement { speed: 100. })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(5., 5.))
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

    let move_vec = input_vec.normalize_or_zero().extend(0.);
    transform.translation += move_vec * movement.speed * time.delta_seconds();
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
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = event {
            if let (Ok(mut health), Ok(damage)) = (
                player_query.get_component_mut::<Health>(*e1),
                bullet_query.get_component::<Damage>(*e2),
            ) {
                health.0 -= damage.0;
            } else if let (Ok(mut health), Ok(damage)) = (
                player_query.get_component_mut::<Health>(*e2),
                bullet_query.get_component::<Damage>(*e1),
            ) {
                health.0 -= damage.0;
            }
        }
    }
}
