use bevy::{prelude::*, window::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
// use web_sys::console;

use nox::animator;
use nox::bullet;
use nox::camera;
use nox::enemy;
use nox::physics;
use nox::player;
use nox::screens::mainmenu;

fn setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            width: 400.,
            height: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(camera::CameraPlugin)
        // .add_plugin(mainmenu::MainMenuPlugin)
        .add_startup_system(setup)
        .add_system(animator::animate_sprite)
        .run();
}
