use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{bind, render, rsx, styles::*, widget, Binding, Bound, MutableBound},
    widgets::*,
};

use super::super::{
    component::Health, config::AppState, player::Player, spawn_waves::WaveResource,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(render_ui))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_player_hp)
                    .with_system(update_wave_number),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(destroy_ui));
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
struct WaveNumber(u32);

fn render_ui(mut cmd: Commands) {
    cmd.insert_resource(bind(Health::default()));
    cmd.insert_resource(bind(WaveNumber::default()));

    let context = BevyContext::new(|context| {
        render! {
            <kayak_ui::widgets::App>
                <HealthWidget />
                <WaveWidget />
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

#[widget]
fn WaveWidget() {
    let wave_binding =
        context.query_world::<Res<Binding<WaveNumber>>, _, _>(|wave_number| wave_number.clone());
    context.bind(&wave_binding);

    let wave_style = Style {
        padding_left: StyleProp::Value(Units::Pixels(100.)),
        ..Style::default()
    };

    rsx! {
        <Window styles={Some(wave_style)}>
            <Text size={30.0} content={format!("Wave {}", wave_binding.get().0)} />
        </Window>
    }
}

fn update_player_hp(query: Query<&Health, With<Player>>, binding: Res<Binding<Health>>) {
    let health = query.single();
    binding.set(health.clone());
}

fn update_wave_number(wave_resource: Res<WaveResource>, binding: Res<Binding<WaveNumber>>) {
    binding.set(WaveNumber(wave_resource.wave_number));
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
