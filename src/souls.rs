use super::enemy::*;
use super::prefabs::weapon::{steel_sword, wooden_bow};

use super::collision_group::*;
use bevy_rapier2d::prelude::*;

use bevy_tweening::{lens::*, *};

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use super::{animator::*, assetloader::get_tileset, component::*, inventory::*};

pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    MYTHIC,
}
impl Rarity {
    fn color(&self) -> Color {
        match *self {
            Rarity::COMMON => Color::OLIVE,
            Rarity::UNCOMMON => Color::SILVER,
            Rarity::RARE => Color::ORANGE,
            Rarity::MYTHIC => Color::RED,
        }
    }
    fn bonus(&self) -> i32 {
        match *self {
            Rarity::COMMON => 0,
            Rarity::UNCOMMON => 10,
            Rarity::RARE => 25,
            Rarity::MYTHIC => 60,
        }
    }
    fn new(val: i32) -> Rarity {
        if val == 0 {
            Rarity::COMMON
        } else if val == 1 {
            Rarity::UNCOMMON
        } else if val == 2 {
            Rarity::RARE
        } else {
            Rarity::MYTHIC
        }
    }
}
#[derive(Component)]
struct Soul;
#[derive(Component)]
struct Equipable {
    rarity: Rarity,
}

#[derive(Bundle)]
struct SoulBundle {
    soul: Soul,
    #[bundle]
    sprite: SpriteSheetBundle,
    decay: Decay,
}
pub fn spawn_drop(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    drops: &Drops,
    spawn_pos: Vec3,
) {
    let mut rng = thread_rng();
    let c: f32 = rng.gen();
    let r = rng.gen_range(0..=3);

    let rarity = Rarity::new(r);
    // if c>drops.chance{
    //     return ;
    // }
    let tween = Tween::new(
        EaseFunction::SineInOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(3000),
        AnchorYAxisLens {
            start: -0.15,
            end: 0.15,
        },
    );

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: drops.frame,
            color: rarity.color(),
            ..default()
        },
        texture_atlas: get_tileset(assets, atlases),

        transform: Transform {
            scale: Vec3::new(1.5, 1.5, 0.),
            translation: spawn_pos,
            ..default()
        },
        ..default()
    })
    .insert(Equipable { rarity })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(CollisionGroups::new(EQUIPABLE, PLAYER))
    .insert(Animator::new(tween))
    .insert(Name::new(drops.name.clone()));
}

pub fn spawn_soul(
    cmd: &mut Commands,
    assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec3,
) {
    cmd.spawn_bundle(SoulBundle {
        soul: Soul,
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 1056,
                ..default()
            },
            texture_atlas: get_tileset(&assets, texture_atlases),
            transform: Transform {
                translation: spawn_pos,
                ..default()
            },
            ..default()
        },

        decay: Decay {
            timer: Timer::new(Duration::from_secs(10), true),
        },
    });
}

fn handle_item_collision(
    mut item_query: Query<(&Equipable, &mut Name)>,
    mut events: EventReader<CollisionEvent>,
    mut inventory: &mut ResMut<InventoryResource>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(mut e1, mut e2, flags) = event {
            if let Ok(equipable) = item_query.get_component_mut::<Equipable>(e1) {
                let r = item_query.get_component_mut::<Name>(e1);
                match r {
                    Ok(name) => {
                        if name.as_str() == "bow" {
                            inventory.primary_weapon = wooden_bow();
                        } else if (name.as_str() == "sword") {
                            inventory.primary_weapon = steel_sword();
                        }
                    }
                    Err(er) => println!("Error"),
                }
            } else if let equipable = item_query.get_component_mut::<Equipable>(e2) {
                let r = item_query.get_component_mut::<Name>(e1);
                match r {
                    Ok(name) => {
                        if name.as_str() == "bow" {
                            inventory.primary_weapon = wooden_bow()
                        } else if (name.as_str() == "sword") {
                            inventory.primary_weapon = steel_sword()
                        }
                    }
                    Err(er) => println!("Error"),
                }
            }
        }
    }
}
