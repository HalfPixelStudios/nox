use super::models::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub type BulletMap = HashMap<String, BulletPrefab>;

pub struct PrefabResource {
    bullet_map: BulletMap,
}

impl PrefabResource {
    pub fn new(filepath: &str) -> Self {
        let formatted = format!("./assets/{}", filepath);
        let content = fs::read_to_string(Path::new(&formatted)).unwrap();
        let bullet_map: BulletMap = ron::from_str(&content).unwrap();

        println!("{:?}", bullet_map);

        PrefabResource { bullet_map }
    }

    pub fn get_bullet(&self, bullet_id: &str) -> Option<&BulletPrefab> {
        self.bullet_map.get(bullet_id)
    }
}
