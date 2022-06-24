use bevy::{
    ecs::world::EntityRef,
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    prelude::*,
};
// use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use super::bullet::spawn_player_bullet;
use super::camera::Cursor;
use super::collision::OnCollide;
use super::component::{Damage, Health};
use super::error::{BoxResult, BreakError};

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
            .add_system(player_shoot);
    }
}

fn spawn_player(mut cmd: Commands) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0., 0.),
            ..default()
        },
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
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(OnCollide {
        handler: on_collide,
    });
}

fn player_controller(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Movement), With<Player>>,
) {
    let (mut transform, movement) = query.single_mut();

    let mut input_vec = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        input_vec += Vec2::Y;
    } else if input.pressed(KeyCode::S) {
        input_vec -= Vec2::Y;
    }
    if input.pressed(KeyCode::A) {
        input_vec -= Vec2::X;
    } else if input.pressed(KeyCode::D) {
        input_vec += Vec2::X;
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
        spawn_player_bullet(&mut cmd, player_trans.translation, bullet_direction);
    }
}

fn on_collide(me: &mut EntityRef, other: &mut EntityRef) -> BoxResult<()> {
    let health = me.get::<Health>().ok_or(BreakError)?;
    let bullet_damage = other.get::<Damage>().ok_or(BreakError)?;

    println!("have {} health, took {} damage", health.0, bullet_damage.0);

    Ok(())
}
