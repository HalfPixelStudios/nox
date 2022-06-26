use super::enemy::*;
use super::prefabs::weapon::{steel_sword, wooden_bow};

use super::collision_group::*;
use bevy_rapier2d::prelude::*;

use bevy_tweening::{lens::*, *};

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use super::{animator::*, assetloader::get_tileset, component::*, inventory::*, player::*};

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_arrow_ui)
            .add_system(equip_system);
    }
}

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
struct ArrowUI;
#[derive(Component)]
struct Soul;
#[derive(Component)]
pub struct Equipable {
    pub rarity: Rarity,
    pub name: String,
    closest: bool,
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
    .insert(Equipable {
        rarity,
        name: drops.name.clone(),
        closest: false,
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(CollisionGroups::new(EQUIPABLE, EQUIPABLE))
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
pub fn create_arrow_ui(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tween = Tween::new(
        EaseFunction::BounceOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(1000),
        AnchorYAxisLens {
            start: -0.1,
            end: 0.1,
        },
    );

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 25,
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

fn equip_system(
    mut cmd: Commands,
    mut item_query: Query<(Entity, &mut Equipable, &mut Name, &Transform), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut inventory: ResMut<InventoryResource>,
    input: Res<Input<KeyCode>>,
) {
    let ptransform = player_query.single();
    let mut least_distance = 1000.;
    let mut close_equip = None;
    let mut close_entity = None;
    for (entity, mut equipable, name, transform) in item_query.iter_mut() {
        let dist = transform
            .translation
            .truncate()
            .distance(ptransform.translation.truncate());

        if dist < 20. && dist < least_distance {
            close_equip = Some(equipable);
            close_entity = Some(entity);

            least_distance = dist;
        }
    }
    match close_equip {
        Some(mut e) => {
            e.closest = true;
            let mut created = false;

            if input.just_pressed(KeyCode::E) {
                if e.name == "bow" {
                    inventory.primary_weapon = wooden_bow();
                    created = true;
                } else if (e.name == "sword") {
                    inventory.primary_weapon = steel_sword();
                    created = true;
                }
            }
            if created {
                let entity = close_entity.unwrap();
                cmd.entity(entity).despawn();
            }
        }
        None => (),
    }
}
