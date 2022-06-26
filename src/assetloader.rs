use bevy::prelude::*;

pub fn load_from_tileset(
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    x: u32,
    y: u32,
) -> Handle<TextureAtlas> {
    let tileset_handle: Handle<Image> = assets.load("tilesheet.png");
    let atlas = TextureAtlas::from_grid(
        tileset_handle,
        Vec2::new(16.0, 16.0),
        x as usize,
        y as usize,
    );
    texture_atlases.add(atlas)
}
