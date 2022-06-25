use bevy::{
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    prelude::*,
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use super::animator::*;
use bevy_rapier2d::prelude::*;

use super::bullet::spawn_player_bullet;
use super::camera::Cursor;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(u8);

#[derive(Component, Inspectable, Default)]
struct Movement {
    speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_controller)
            .add_system(player_shoot)
            .register_inspectable::<Movement>();
    }
}

fn spawn_player(mut cmd: Commands, assets:Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = assets.load("player.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(96.0,84.0),14,20);
    let atlas_handle = texture_atlases.add(atlas);
    cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas_handle,
        transform: Transform {
            scale: Vec3::new(10., 10., 10.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Health(100))
    .insert(Movement { speed: 100. })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(AniState{action:Action::IDLE,direction:Dir::RIGHT})
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

}

fn player_controller(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Movement, &mut AniState), With<Player>>,
) {
    let (mut transform, movement,mut state) = query.single_mut();

    let mut input_vec = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        input_vec += Vec2::Y;
    } else if input.pressed(KeyCode::S) {
        input_vec -= Vec2::Y;
    }
    if input.pressed(KeyCode::A) {
        input_vec -= Vec2::X;
        state.direction=Dir::LEFT;
    } else if input.pressed(KeyCode::D) {
        input_vec += Vec2::X;
        state.direction=Dir::RIGHT;
    }

    let move_vec = input_vec.normalize_or_zero().extend(0.);
    transform.translation += move_vec * movement.speed * time.delta_seconds();
}

fn player_shoot(
    mut cmd: Commands,
    input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let player_trans = player_query.single_mut();

    if input.just_pressed(KeyCode::Space) {
        // TODO: should error if bullet direction is ever zero
        let bullet_direction = (cursor.0 - player_trans.translation.truncate()).normalize_or_zero();
        spawn_player_bullet(cmd, player_trans.translation, bullet_direction);
    }
}
