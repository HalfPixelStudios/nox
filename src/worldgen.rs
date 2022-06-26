use bevy::prelude::*;

pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {}
