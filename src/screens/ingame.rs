use super::super::component::*;
use super::super::player::*;
use super::super::{config::AppState, spawn_waves::WaveResource};
use super::UIRoot;
use bevy::{prelude::*, ui::FocusPolicy};
#[derive(Component)]
struct HealthCounter;
#[derive(Component)]
struct WaveCounter;

pub struct IngameMenuPlugin;

impl Plugin for IngameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_ui))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_ui)
                    .with_system(update_health),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    let font_path = "fonts/arcadeclassic.ttf";

    cmd.spawn_bundle(NodeBundle {
        style: Style {
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            margin: Rect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Undefined,
                bottom: Val::Auto,
            },
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
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        right: Val::Px(335.0),
                        ..default()
                    },
                    ..default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "hello\nbevy!",
                    TextStyle {
                        font: assets.load(font_path),
                        font_size: 40.0,
                        color: Color::CRIMSON,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(HealthCounter);

        parent
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        TextSection {
                            value: "Wave ".to_string(),
                            style: TextStyle {
                                font: assets.load(font_path),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: assets.load(font_path),
                                font_size: 40.0,
                                color: Color::GOLD,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            })
            .insert(WaveCounter);
    });
}
fn update_health(
    mut player_query: Query<&Health, With<Player>>,
    mut ui_query: Query<&mut Text, With<HealthCounter>>,
) {
    let health = player_query.single();
    for mut text in ui_query.iter_mut() {
        text.sections.get_mut(0).unwrap().value = format!("HP  {}", health.0);
    }
}

fn update_ui(mut query: Query<&mut Text, With<WaveCounter>>, wave_resource: Res<WaveResource>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", wave_resource.wave_number);
    }
}

fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {
    let entity = query.single();
    cmd.entity(entity).despawn_recursive();
}
