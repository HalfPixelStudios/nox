use bevy::prelude::*;

const ATLAS_WIDTH: usize = 49;
const ATLAS_HEIGHT: usize = 22;
pub struct CharSheet(pub Handle<TextureAtlas>);

pub fn load_tileset(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let tileset_handle: Handle<Image> = assets.load("tilesheet.png");

    let atlas = TextureAtlas::from_grid(
        tileset_handle,
        Vec2::new(16.0, 16.0),
        ATLAS_WIDTH,
        ATLAS_HEIGHT,
    );
    let atlas_handle = texture_atlases.add(atlas);
    cmd.insert_resource(CharSheet(atlas_handle));
}
pub struct AssetLoadPlugin;
impl Plugin for AssetLoadPlugin{
    fn build(&self,app:&mut App){
        app.add_startup_system_to_stage(StartupStage::PreStartup,load_tileset);
    }
}
