use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
use rand::prelude::*;
use std::time::Duration;

use crate::{
    animator::*, assetloader::*, collision_group::*, component::*, enemy::*,
    inventory::*, physics::PhysicsBundle, player::*, prefabs::{models::WeaponPrefab, PrefabResource}
};

pub struct SpawnDroppedItemEvent {
    pub weapon_id: String,
    pub spawn_pos: Vec2,
}

pub struct DroppedItemPlugin;
impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDroppedItemEvent>().add_system(pickup_system).add_system(spawn_dropped_item);
    }
}

#[derive(Component)]
pub struct DroppedItem {
    pub weapon_prefab: WeaponPrefab
}

pub fn spawn_dropped_item(mut cmd: Commands, mut events: EventReader<SpawnDroppedItemEvent>, prefab_res: Res<PrefabResource>, char_sheet:Res<CharSheet>) {

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
            texture_atlas: char_sheet.0.clone(),
            transform: Transform {
                translation: spawn_pos.extend(0.),
                ..default()
            },
            ..default()
        })
        .insert_bundle(PhysicsBundle::default())
        .insert(CollisionGroups::new(EQUIPABLE, EQUIPABLE));
    }
}

pub fn pickup_system() {

}
