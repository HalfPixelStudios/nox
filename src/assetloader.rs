use bevy::prelude::*;

pub struct AssetloaderPlugin;

impl Plugin for AssetloaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {}
