use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{styles::*, *},
    widgets::*,
};

use super::super::config::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(render_ui))
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands) {
    let context = BevyContext::new(|context| {
        let button_style = Style {
            width: StyleProp::Value(Units::Pixels(200.)),
            height: StyleProp::Value(Units::Pixels(50.)),
            ..Style::default()
        };

        let click_event = OnEvent::new(move |context, evt| {
            context.query_world::<EventWriter<AppExit>, _, _>(|mut writer| match evt.event_type {
                EventType::Click(..) => {
                    writer.send(AppExit);
                }
                _ => {}
            });
        });

        render! {
            <kayak_ui::widgets::App>
                <Button styles={Some(button_style)} on_event={Some(click_event)}>
                    <Text size={30.0} content={"Quit Game".to_string()} />
                </Button>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
