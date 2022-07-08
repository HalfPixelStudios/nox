use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

#[derive(Component)]
pub struct ArrowUI;

/*
pub fn create_arrow_ui(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tween = Tween::new(
        EaseFunction::SineInOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(1000),
        AnchorYAxisLens {
            start: -0.8,
            end: -1.1,
        },
    );

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 1059,
            color: Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.,
            },
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            scale: Vec3::new(1.5, 1.5, 0.),
            ..default()
        },
        ..default()
    })
    .insert(Animator::new(tween))
    .insert(ArrowUI);
}

fn move_arrow_ui(
    mut item_query: Query<(&Equipable, &Transform), Without<ArrowUI>>,
    mut arrow_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<ArrowUI>>,
) {
    let (mut arrow_transform, mut sprite) = arrow_query.single_mut();
    sprite.color = Color::Rgba {
        red: 1.,
        green: 1.,
        blue: 1.,
        alpha: 0.,
    };
    for (equipable, item_transform) in item_query.iter() {
        if equipable.closest {
            arrow_transform.translation = item_transform.translation;
            sprite.color = Color::WHITE;
        }
    }
}
*/
