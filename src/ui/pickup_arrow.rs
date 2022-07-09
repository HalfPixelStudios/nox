use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::{assetloader::get_tileset, dropped_item::{ClosestItemResource, DroppedItem}, animator::AnchorYAxisLens};

#[derive(Component)]
pub struct ArrowUI;

pub struct PickupArrowPlugin;

impl Plugin for PickupArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_arrow_ui).add_system(move_arrow_ui);
    }
}

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
    closest_item: Res<ClosestItemResource>,
    item_query: Query<&Transform, With<DroppedItem>>,
    mut arrow_query: Query<(&mut Transform, &mut TextureAtlasSprite), (With<ArrowUI>, Without<DroppedItem>)>,
) {
    let (mut trans, mut sprite) = arrow_query.single_mut();

    // set arrow to be invisible if no closest item
    match closest_item.entity {
        Some(e) => {
            // TODO maybe warning if could not get entity
            if let Ok(item_trans) = item_query.get(e) {
                trans.translation = item_trans.translation;
                sprite.color.set_a(1.);
            }
        },
        None => {
            sprite.color.set_a(0.);
        }
    };
}
