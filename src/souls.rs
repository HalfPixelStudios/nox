use super::component::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;
use super::enemy::*;

#[derive(Component)]
struct Soul;

#[derive(Bundle)]
struct SoulBundle {
    soul: Soul,
    #[bundle]
    sprite: SpriteBundle,
    decay: Decay,
}
pub fn spawn_drop(cmd: &mut Commands,assets: &Res<AssetServer>,atlases: &mut ResMut<Assets<TextureAtlas>>, drops: &Drops, spawn_pos: Vec3){
    let mut rng = thread_rng();
    let c:f32 = rng.gen();
    if c>drops.chance{
        return ;
    }
    let handle:Handle<TextureAtlas> = assets.load("tilesheet.png");
    let atlas = atlases.get_handle(handle);
    
	cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite{
            index:drops.frame.into(),
            ..default()
        },
        texture_atlas: atlas,
        transform: Transform{
            scale: Vec3::new(1.5, 1.5, 0.),
            ..default()
        },
        ..default()
    })
    .insert(Name::new(drops.name.clone()));

}

    

pub fn spawn_soul(cmd: &mut Commands, assets: &Res<AssetServer>, spawn_pos: Vec3) {
    cmd.spawn_bundle(SoulBundle {
        soul: Soul,
        sprite: SpriteBundle {
            texture: assets.load("soul.png"),

            transform: Transform {
                translation: spawn_pos,
                scale: Vec3::new(0.1, 0.1, 0.),
                ..default()
            },
            ..default()
        },

        decay: Decay {
            timer: Timer::new(Duration::from_secs(10), true),
        },
    });
}
