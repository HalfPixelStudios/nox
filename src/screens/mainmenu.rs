use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(UiCameraBundle::default());
    cmd.spawn_bundle(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.), Val::Percent(10.)),
            margin: Rect::all(Val::Auto),
            ..default()
        },
        ..default()
    });
}
