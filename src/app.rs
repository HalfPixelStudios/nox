use bevy::{prelude::*, window::*};
use bevy_hanabi::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
// use web_sys::console;

use super::{
    animator, bullet, camera, component, config::AppState, enemy, inventory, particles, physics,
    player, screens, spawn_waves, worldgen,
};

fn setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub struct AppConfig {
    pub app_state: AppState,
    pub fullscreen: bool,
    pub egui_enabled: bool,
    pub debug_render: bool,
}

pub fn run_app(app_config: AppConfig) {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        ..default()
    };
    if !app_config.fullscreen {
        window_descriptor.width = 800.;
        window_descriptor.height = 600.;
    }

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor)
        .add_plugins(DefaultPlugins)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(HanabiPlugin)
        .add_state(app_config.app_state)
        .add_system_set(SystemSet::on_update(AppState::InGame))
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(particles::ParticlePlugin)
        .add_system(component_animator_system::<TextureAtlasSprite>)
        .add_system(component::decay_system)
        .add_plugin(worldgen::WorldgenPlugin)
        .add_plugin(spawn_waves::SpawnWavesPlugin)
        .add_plugin(inventory::InventoryPlugin)
        .add_system(animator::animate_sprite)
        .add_plugin(screens::UIPlugin);

    if app_config.egui_enabled {
        app.add_plugin(WorldInspectorPlugin::new());
    }
    if app_config.debug_render {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}
