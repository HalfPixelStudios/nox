use bevy::prelude::*;

pub mod ingame;
pub mod mainmenu;

#[derive(Component)]
pub struct UIRoot;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(mainmenu::MainMenuPlugin)
            .add_plugin(ingame::IngameMenuPlugin);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(UiCameraBundle::default());
}
