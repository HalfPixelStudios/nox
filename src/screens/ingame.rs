use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{styles::*, bind, render, rsx, widget, Binding, Bound, MutableBound},
    widgets::*,
};

use super::super::{
    player::Player,
    component::Health,
    config::AppState
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_ui))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_player_hp))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(destroy_ui));
    }
}

fn render_ui(mut cmd: Commands) {

    let hp_binding = bind(Health::default());
    cmd.insert_resource(hp_binding);

    let context = BevyContext::new(|context| {

        render! {
            <kayak_ui::widgets::App>
                <HealthWidget />
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

#[widget]
fn HealthWidget() {

    let hp_binding = context.query_world::<Res<Binding<Health>>, _, _>(|hp| hp.clone());
    context.bind(&hp_binding);

    rsx! {
        <Window>
            <Text size={30.0} content={format!("health {}", hp_binding.get().0)} />
        </Window>
    }
}

fn update_player_hp(query: Query<&Health, With<Player>>, binding: Res<Binding<Health>>) {
    let health = query.single();
    binding.set(health.clone());
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}

