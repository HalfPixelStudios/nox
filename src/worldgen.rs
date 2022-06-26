use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use super::assetloader::get_tileset;

pub struct WorldgenPlugin;

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_world);
    }
}

fn generate_world(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const GRID_SIZE: f32 = 20.;
    const SPAWN_CHANCE: u32 = 100;

    let forest_trees: Vec<usize> = vec![50, 51, 52, 53, 54];
    let tileset_handle = get_tileset(&assets, &mut texture_atlases);

    for y in 0..100 {
        for x in 0..100 {
            if rand::thread_rng().gen_range(0..SPAWN_CHANCE) != 0 {
                continue;
            }
            let index = forest_trees.choose(&mut rand::thread_rng()).unwrap();
            let spawn_pos = Vec2::new(
                ((x as f32) - 50.) * GRID_SIZE,
                ((y as f32) - 50.) * GRID_SIZE,
            );
            spawn_forgery(&mut cmd, &tileset_handle, *index, spawn_pos);
        }
    }
}

fn spawn_forgery(
    cmd: &mut Commands,
    tileset_handle: &Handle<TextureAtlas>,
    index: usize,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: index,
            color: Color::rgb(0., 0.7, 0.),
            ..default()
        },
        texture_atlas: tileset_handle.clone(),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            scale: Vec3::splat(1.5),
            ..default()
        },
        ..default()
    });
}
