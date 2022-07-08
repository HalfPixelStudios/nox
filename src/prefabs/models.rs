use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ENEMY =-=-=-=-=-=-=
#[derive(Debug, Serialize, Deserialize)]
pub enum AI {
    Simple { target_range: f32 },
    Loiter { chaos: u32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Drop {
    pub item_id: String,
    pub chance: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub speed: f32,
    pub ai: AI,
    pub weapon: String,
    pub attack_range: f32,
    pub sprite_index: u32,
    pub sprite_color: (f32, f32, f32),
    pub hurt_sounds: Vec<String>,
    pub die_sounds: Vec<String>,
    pub drops: Vec<Drop>,
}

// PROJECTILE =-=-=-=-=-=-=

#[derive(Debug, Serialize, Deserialize)]
pub enum Lifetime {
    Distance(f32),
    Duration(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulletPrefab {
    pub display_name: Option<String>,
    pub damage: u32,
    pub penetration: u32,
    pub speed: f32,
    pub lifetime: Lifetime,
    pub sprite_index: u32,
    pub sprite_color: (f32, f32, f32),
}

// WEAPON =-=-=-=-=-=-=

#[derive(Debug, Serialize, Deserialize)]
pub enum ShootPattern {
    Straight,
    Shotgun { shots: u32, angle: f32 },
    Around { shots: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BulletRef {
    Key(String),
    Inline(BulletPrefab),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponPrefab {
    pub display_name: Option<String>,
    pub projectile: String, // name of projectile that is fired (or inline definition)
    pub shoot_pattern: ShootPattern,
    pub attack_speed: f32, // time between consecutive attacks
    pub attack_sounds: Vec<String>,
    pub sprite_index: u32,
    pub sprite_color: (f32, f32, f32),
}
