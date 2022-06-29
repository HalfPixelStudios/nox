use bevy::prelude::*;

use super::souls::*;

pub struct InventoryResource {
    pub primary_weapon: String,
    pub rarity: Rarity,
    pub eaten: bool,
    pub eat_rarity: Rarity,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryResource>();
    }
}

impl Default for InventoryResource {
    fn default() -> Self {
        InventoryResource {
            primary_weapon: "steel_sword".to_string(),
            rarity: Rarity::COMMON,
            eaten: true,
            eat_rarity: Rarity::COMMON,
        }
    }
}

impl InventoryResource {
    pub fn switch_weapon(&mut self, name: i32, rarity: &Rarity) {}
}
