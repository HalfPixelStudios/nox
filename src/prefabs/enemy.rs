use bevy::{core::Stopwatch, prelude::*};

use super::super::{
    assetloader::get_tileset, audio::SoundEmitter, component::Health, enemy::*, weapon::Weapon,
};
use super::bullet::*;
use super::weapon::*;

pub fn bow_orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        drops: Drops { ..default() },

        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["orc/hurt1.wav".to_string(), "orc/hurt2.wav".to_string()],
            die_sounds: vec!["orc/die1.wav".to_string(), "orc/die2.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 123,
            color: Color::rgb(0., 1., 0.),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 200.,
        weapon: wooden_bow(),
        attack_timer: Stopwatch::new(),
    })
    .insert(SimpleMovement {
        speed: 40.,
        target_range: 100.,
    });
}

pub fn greatsword_orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(50),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["orc/hurt1.wav".to_string(), "orc/hurt2.wav".to_string()],
            die_sounds: vec!["orc/die1.wav".to_string(), "orc/die2.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 126,
            color: Color::rgb(0., 1., 0.),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 60.,
        weapon: steel_greatsword(),
        attack_timer: Stopwatch::new(),
    })
    .insert(SimpleMovement {
        speed: 40.,
        target_range: 40.,
    });
}

pub fn royal_hammer_orc(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(100),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["orc/hurt1.wav".to_string(), "orc/hurt2.wav".to_string()],
            die_sounds: vec!["orc/die1.wav".to_string(), "orc/die2.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 125,
            color: Color::rgb(0., 1., 0.),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 100.,
        weapon: royal_hammer(),
        attack_timer: Stopwatch::new(),
    })
    .insert(SimpleMovement {
        speed: 30.,
        target_range: 10.,
    });
}

pub fn bat(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(20),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["bat/hurt1.wav".to_string()],
            die_sounds: vec!["bat/die1.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 418,
            color: Color::rgb(0.2, 0.2, 0.2),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 50.,
        weapon: poison_dagger(),
        attack_timer: Stopwatch::new(),
    })
    .insert(LoiterMovement {
        speed: 40.,
        chaos: 20,
        current_dir: Vec2::ZERO,
    });
}

pub fn hornet(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(50),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["bee/hurt1.wav".to_string()],
            die_sounds: vec!["bee/die1.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 276,
            color: Color::rgb(0.9, 0.5, 0.02),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 50.,
        weapon: poison_dagger(),
        attack_timer: Stopwatch::new(),
    })
    .insert(LoiterMovement {
        speed: 70.,
        chaos: 5,
        current_dir: Vec2::ZERO,
    });
}

pub fn flame_mage(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    cmd.spawn_bundle(EnemyBundle {
        health: Health(70),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec!["player/hurt1.wav".to_string()],
            die_sounds: vec!["player/hurt1.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 73,
            color: Color::rgb(0.8, 0.2, 0.02),
            ..default()
        },
        texture_atlas: get_tileset(&assets, &mut texture_atlases),
        transform: Transform {
            translation: spawn_pos.extend(0.),
            ..default()
        },
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 100.,
        weapon: flamethrower_staff(),
        attack_timer: Stopwatch::new(),
    })
    .insert(LoiterMovement {
        speed: 20.,
        chaos: 50,
        current_dir: Vec2::ZERO,
    });
}

// bosses
pub fn boss_grand_sphinx(
    cmd: &mut Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_pos: Vec2,
) {
    let handle = assets.load("grand_sphinx.png");

    cmd.spawn_bundle(EnemyBundle {
        health: Health(300),
        drops: Drops::default(),
        sound_emitter: SoundEmitter {
            hurt_sounds: vec![
                "grand_sphinx/hurt1.wav".to_string(),
                "grand_sphinx/hurt2.wav".to_string(),
            ],
            die_sounds: vec!["grand_sphinx/die1.wav".to_string()],
        },
        ..default()
    })
    .insert_bundle(SpriteBundle {
        transform: Transform {
            translation: spawn_pos.extend(0.),
            scale: Vec3::splat(2.),
            ..default()
        },
        texture: handle,
        ..default()
    })
    .insert(AttackPolicy {
        attack_range: 500.,
        weapon: grand_sphinx_attack(),
        attack_timer: Stopwatch::new(),
    })
    .insert(LoiterMovement {
        speed: 50.,
        chaos: 60,
        current_dir: Vec2::ZERO,
    });
}
