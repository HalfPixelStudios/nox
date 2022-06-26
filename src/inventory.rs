use bevy::prelude::*;

use super::prefabs::weapon::{steel_sword, wooden_bow};
use super::souls::*;
use super::weapon::Weapon;

pub struct InventoryResource {
    pub primary_weapon: Weapon,
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
            primary_weapon: steel_sword(),
            rarity: Rarity::COMMON,
            eaten: true,
            eat_rarity: Rarity::COMMON,
        }
    }
}

impl InventoryResource {
    pub fn switch_weapon(&mut self, name: &String, rarity: &Rarity) {
        self.eaten = false;
        self.eat_rarity = self.rarity.clone();
        if (name == "bow") {
            self.primary_weapon = wooden_bow();
        } else if (name == "sword") {
            self.primary_weapon = steel_sword();
        }

        self.rarity = rarity.clone();
    }
}
