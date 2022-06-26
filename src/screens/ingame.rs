use bevy::{prelude::*, ui::FocusPolicy};

use super::super::{config::AppState, spawn_waves::WaveResource};
use super::UIRoot;

#[derive(Component)]
struct WaveCounter;

pub struct IngameMenuPlugin;

impl Plugin for IngameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_ui))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_ui))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    let font_handle = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn_bundle(NodeBundle {
        style: Style {
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            margin: Rect::all(Val::Auto),
            ..default()
        },
        color: UiColor(Color::NONE),
        ..default()
    })
    .insert(UIRoot)
    .with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    margin: Rect::all(Val::Auto),
                    ..default()
                },
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: font_handle,
                        font_size: 40.,
                        color: Color::rgb(0., 0., 0.),
                        ..default()
                    },
                    default(),
                ),
                ..default()
            })
            .insert(WaveCounter);
    });
}

fn update_ui(mut query: Query<&mut Text, With<WaveCounter>>, wave_resource: Res<WaveResource>) {
    let mut text_box = query.single_mut();
    let text_section = text_box.sections.get_mut(0).unwrap();
    text_section.value = format!("Wave {}", wave_resource.wave_number);
}

fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {
    let entity = query.single();
    cmd.entity(entity).despawn_recursive();
}
