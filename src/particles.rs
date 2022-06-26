use bevy::prelude::*;
use bevy_hanabi::*;

pub struct ParticlePlugin;

pub struct EffectAtlas {
    test: Handle<EffectAsset>,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut cmd: Commands, mut effect_asset: ResMut<Assets<EffectAsset>>) {
    let test_effect = effect_asset.add(
        EffectAsset {
            capacity: 32768,
            spawner: Spawner::rate(5.0.into()),
            ..default()
        }
        .init(PositionSphereModifier {
            radius: 1.,
            speed: 10.0.into(),
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(10.)),
        })
        .render(ColorOverLifetimeModifier {
            gradient: Gradient::constant(Vec4::splat(1.)),
        }),
    );

    cmd.insert_resource(EffectAtlas { test: test_effect });
}

pub fn spawn_test_particles(mut cmd: Commands, effects: Res<EffectAtlas>) {
    cmd.spawn().insert_bundle(ParticleEffectBundle {
        effect: ParticleEffect::new(effects.test.clone()),
        ..default()
    });
}
