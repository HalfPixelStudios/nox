pub mod simple_material;

use bevy::{prelude::*, sprite::*};

use simple_material::*;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<SimpleMaterial>::default());
    }
}

fn setup(
    mut cmd: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<SimpleMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("player.png");

    cmd.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets
            .add(Mesh::from(shape::Quad::new(Vec2::splat(100.))))
            .into(),
        material: material_assets.add(SimpleMaterial {
            color: Color::RED,
            texture: texture_handle,
        }),
        ..default()
    });
}
