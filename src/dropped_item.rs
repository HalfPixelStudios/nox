use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
use rand::prelude::*;
use std::time::Duration;

use crate::{
    animator::*, assetloader::get_tileset, collision_group::*, component::*, enemy::*,
    inventory::*, physics::PhysicsBundle, player::*, prefabs::{models::WeaponPrefab, PrefabResource}
};

pub struct SpawnDroppedItemEvent {
    pub weapon_id: String,
    pub spawn_pos: Vec2,
}

pub struct PickupItemEvent {
    pub weapon_id: String
}

pub struct DroppedItemPlugin;
impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDroppedItemEvent>().add_event::<PickupItemEvent>().add_system(pickup_system).add_system(spawn_dropped_item);
    }
}

#[derive(Component)]
pub struct DroppedItem {
    pub weapon_id: String
}

pub fn spawn_dropped_item(mut cmd: Commands, mut events: EventReader<SpawnDroppedItemEvent>, prefab_res: Res<PrefabResource>, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    for SpawnDroppedItemEvent { weapon_id, spawn_pos } in events.iter() {

        let prefab = prefab_res.get_weapon(weapon_id);
        if prefab.is_none() {
            warn!("unable to fetch weapon prefab: {}", weapon_id);
            continue;
        }
        let prefab = prefab.unwrap();

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: prefab.sprite_index as usize,
                color: Color::rgb(prefab.sprite_color.0, prefab.sprite_color.1, prefab.sprite_color.2), // TODO hella ugly
                ..default()
            },
            texture_atlas: get_tileset(&assets, &mut texture_atlases),
            transform: Transform {
                translation: spawn_pos.extend(0.),
                ..default()
            },
            ..default()
        })
        .insert_bundle(PhysicsBundle::default())
        .insert(DroppedItem { weapon_id: weapon_id.clone() })
        .insert(CollisionGroups::new(EQUIPABLE, EQUIPABLE));
    }
}

pub fn pickup_system(
    mut cmd: Commands,
    input: Res<Input<KeyCode>>,
    item_query: Query<(Entity, &DroppedItem, &Transform), Without<Player>>,
    player_query: Query<(&Transform, &Pickup), With<Player>>,
    mut writer: EventWriter<PickupItemEvent>
) {
    if !input.just_pressed(KeyCode::E) {
        return;
    }

    let (player_trans, pickup) = player_query.single();

    // find closest item to pickup
    let closest = item_query.iter().fold(None, |min, item@(_, _, item_trans)| {
        if player_trans.translation.truncate().distance(item_trans.translation.truncate()) <= pickup.range {
            Some(item)
        } else {
            min
        }
    });

    if let Some((e, dropped_item, _)) = closest {
        writer.send(PickupItemEvent { weapon_id: dropped_item.weapon_id.clone() });
        cmd.entity(e).despawn();
    }
}
