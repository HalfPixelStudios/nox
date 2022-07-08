pub mod daylight_material;
pub mod simple_material;

use bevy::{prelude::*, sprite::*};

use daylight_material::*;
use simple_material::*;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<SimpleMaterial>::default())
            .add_plugin(Material2dPlugin::<DaylightMaterial>::default());
        // .add_startup_system(setup);
    }
}

fn setup(
    mut cmd: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<DaylightMaterial>>,
) {
    cmd.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets
            .add(Mesh::from(shape::Quad::new(Vec2::splat(5000.))))
            .into(),
        material: material_assets.add(DaylightMaterial {
            color: Color::rgba(0.01, 0.01, 0.01, 1.0),
            lights: vec![
                Light::new(Vec2::new(0., 0.), 10.),
                Light::new(Vec2::new(100., 100.), 10.),
            ],
        }),
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..default()
        },
        ..default()
    });
}
