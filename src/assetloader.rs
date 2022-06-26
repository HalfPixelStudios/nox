use bevy::prelude::*;

const ATLAS_WIDTH: usize = 49;
const ATLAS_HEIGHT: usize = 22;

pub fn get_tileset(
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let tileset_handle: Handle<Image> = assets.load("tilesheet.png");
    println!("The handle Id in get_tileset is {:?}", tileset_handle.id);

    let atlas = TextureAtlas::from_grid(
        tileset_handle,
        Vec2::new(16.0, 16.0),
        ATLAS_WIDTH,
        ATLAS_HEIGHT,
    );
    texture_atlases.add(atlas)
}
