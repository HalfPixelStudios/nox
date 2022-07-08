use bevy::prelude::*;
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping};

pub mod gameover;
pub mod ingame;
pub mod mainmenu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(setup)
            .add_plugin(mainmenu::MainMenuPlugin)
            .add_plugin(ingame::InGamePlugin)
            .add_plugin(gameover::GameOverPlugin);
    }
}

fn setup(mut cmd: Commands, mut font_mapping: ResMut<FontMapping>, asset_server: Res<AssetServer>) {
    cmd.spawn_bundle(kayak_ui::bevy::UICameraBundle::new());

    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));
}
