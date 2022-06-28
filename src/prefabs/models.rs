use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ENEMY =-=-=-=-=-=-=

#[derive(Debug, Serialize, Deserialize)]
struct EnemyPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub speed: f32,
    pub sprite: String,
}

// PROJECTILE =-=-=-=-=-=-=

#[derive(Debug, Serialize, Deserialize)]
pub enum Lifetime {
    Distance(f32),
    Duration(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectilePrefab {
    pub display_name: Option<String>,
    pub damage: u32,
    pub penetration: u32,
    pub speed: f32,
    pub lifetime: Lifetime,
    pub sprite: String,
}

// WEAPON =-=-=-=-=-=-=

#[derive(Debug, Serialize, Deserialize)]
pub enum ShootPattern {
    Straight,
    Shotgun { shots: u32, angle: f32 },
    Around { angle: f32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectileRef {
    Key(String),
    Inline(ProjectilePrefab),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeaponPrefab {
    pub display_name: Option<String>,
    pub projectile: ProjectileRef, // name of projectile that is fired (or inline definition)
    pub shoot_pattern: ShootPattern,
    pub attack_speed: f32, // time between consecutive attacks
}

pub type WeaponMap = HashMap<String, WeaponPrefab>;
