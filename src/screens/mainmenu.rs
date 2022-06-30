use bevy::{app::*, asset::prelude::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{render, rsx, styles::*, Index},
    widgets::*,
};

use super::super::config::AppState;
use super::UIRoot;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(render_ui))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_listener))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    let ctx = BevyContext::new(|context| {
        if context.get_global::<World>().is_err() {
            return;
        }

        let button_style = Style {
            width: StyleProp::Value(Units::Pixels(200.)),
            height: StyleProp::Value(Units::Pixels(50.)),
            ..Style::default()
        };

        render! {
            <kayak_ui::widgets::App>
                <Button styles={Some(button_style)}>
                    <Text size={30.0} content={"Hello world".to_string()} />
                </Button>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(ctx);
}

fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {}

fn button_listener(mut app_state: ResMut<State<AppState>>) {}
