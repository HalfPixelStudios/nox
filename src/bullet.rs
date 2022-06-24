use bevy::prelude::*;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Movement(f32, Vec2);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_movement_system);
    }
}

pub fn spawn_player_bullet(mut cmd: Commands, pos: Vec3, dir: Vec2) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0., 1.),
            ..default()
        },
        transform: Transform {
            translation: pos,
            scale: Vec3::new(10., 2., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Bullet)
    .insert(Movement(500., dir));
}

fn bullet_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement), With<Bullet>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation += movement.0 * movement.1.extend(0.) * time.delta_seconds();
    }
}
