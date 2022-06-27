use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
use rand::prelude::*;
use std::time::Duration;

use super::{
    animator::*, assetloader::get_tileset, collision_group::*, component::*, enemy::*,
    inventory::*, physics::PhysicsBundle, player::*, prefabs::weapon::*,
};

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_arrow_ui)
            .add_system(equip_system)
            .add_system(move_arrow_ui);
    }
}

#[derive(Clone)]
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
pub struct Equipable {
    pub rarity: Rarity,
    pub name: i32,
    closest: bool,
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
    if c > drops.chance {
        return;
    }

    let rarity = Rarity::new(r);
    let tween = Tween::new(
        EaseFunction::BounceOut,
        TweeningType::PingPong,
        std::time::Duration::from_millis(1000),
        AnchorYAxisLens {
            start: -0.11,
            end: 0.11,
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
    .insert_bundle(PhysicsBundle::default())
    .insert(CollisionGroups::new(EQUIPABLE, EQUIPABLE))
    .insert(Animator::new(tween));
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

fn equip_system(
    mut cmd: Commands,
    mut item_query: Query<(Entity, &mut Equipable, &Transform), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut inventory: ResMut<InventoryResource>,
    input: Res<Input<KeyCode>>,
) {
    let ptransform = player_query.single();
    let mut least_distance = 1000.;
    let mut close_equip = None;
    let mut close_entity = None;
    for (entity, mut equipable, transform) in item_query.iter_mut() {
        println!("hello?");

        equipable.closest = false;
        let dist = transform
            .translation
            .truncate()
            .distance(ptransform.translation.truncate());

        if dist < 20. && dist < least_distance {
            println!("Close Enough");

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
                println!("pickup");
                inventory.switch_weapon(e.name, &e.rarity);
                created = true;
            }
            if created {
                let entity = close_entity.unwrap();
                cmd.entity(entity).despawn();
            }
        }
        None => (),
    }
}
