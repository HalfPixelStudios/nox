use bevy::{prelude::*, ui::FocusPolicy};

use super::super::config::AppState;
use super::UIRoot;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(render_ui))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_listener))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    let font_handle = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn_bundle(NodeBundle {
        style: Style {
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.), Val::Percent(20.)),
            margin: Rect::all(Val::Auto),
            ..default()
        },
        focus_policy: FocusPolicy::Pass,
        ..default()
    })
    .insert(UIRoot)
    .with_children(|parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect::all(Val::Auto),
                        ..default()
                    },
                    text: Text::with_section(
                        "Play",
                        TextStyle {
                            font: font_handle,
                            font_size: 40.,
                            color: Color::rgb(0., 0., 0.),
                            ..default()
                        },
                        default(),
                    ),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                });
            });
    });
}

fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {
    let entity = query.single();
    cmd.entity(entity).despawn_recursive();
}

fn button_listener(
    query: Query<&Interaction, Changed<Interaction>>,
    mut app_state: ResMut<State<AppState>>,
) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Clicked => {
                app_state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
