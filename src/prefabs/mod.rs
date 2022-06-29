pub mod builder;
pub mod models;

use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use models::*;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        let resource = PrefabResource::new(
            "./assets/data/bullets.ron",
            "./assets/data/weapons.ron",
            "./assets/data/enemies.ron",
        );
        app.insert_resource(resource);
    }
}

pub type BulletMap = HashMap<String, BulletPrefab>;
pub type WeaponMap = HashMap<String, WeaponPrefab>;
pub type EnemyMap = HashMap<String, EnemyPrefab>;

pub struct PrefabResource {
    bullet_map: BulletMap,
    weapon_map: WeaponMap,
    enemy_map: EnemyMap,
}

impl PrefabResource {
    // TODO duplicated code :(
    pub fn new(bullet_filepath: &str, weapon_filepath: &str, enemy_filepath: &str) -> Self {
        let bullet_content = fs::read_to_string(Path::new(&bullet_filepath)).unwrap();
        let bullet_map: BulletMap = ron::from_str(&bullet_content).unwrap();

        let weapon_content = fs::read_to_string(Path::new(&weapon_filepath)).unwrap();
        let weapon_map: WeaponMap = ron::from_str(&weapon_content).unwrap();

        let enemy_content = fs::read_to_string(Path::new(&enemy_filepath)).unwrap();
        let enemy_map: EnemyMap = ron::from_str(&enemy_content).unwrap();

        PrefabResource {
            bullet_map,
            weapon_map,
            enemy_map,
        }
    }

    pub fn get_bullet(&self, bullet_id: &str) -> Option<&BulletPrefab> {
        self.bullet_map.get(bullet_id)
    }
    pub fn get_weapon(&self, weapon_id: &str) -> Option<&WeaponPrefab> {
        self.weapon_map.get(weapon_id)
    }
    pub fn get_enemy(&self, enemy_id: &str) -> Option<&EnemyPrefab> {
        self.enemy_map.get(enemy_id)
    }
}
