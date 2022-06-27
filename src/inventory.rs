use bevy::prelude::*;

use super::prefabs::weapon::*;
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
    pub fn switch_weapon(&mut self, name: i32, rarity: &Rarity) {
        self.eaten = false;
        self.eat_rarity = self.rarity.clone();
        if name == 1 {
            self.primary_weapon = wooden_bow();
        } else if name == 2 {
            self.primary_weapon = tome_of_zeus();
        } else if name == 3 {
            self.primary_weapon = royal_hammer();
        } else if name == 4 {
            self.primary_weapon = flamethrower_staff();
        } else if name == 5 {
            self.primary_weapon = steel_sword();
        } else if name == 6 {
            self.primary_weapon = steel_greatsword();
        } else if name == 7 {
            self.primary_weapon = orbs_of_despair();
        } else if name == 8 {
            self.primary_weapon = poison_dagger();
        } else {
            self.primary_weapon = tome_of_doom();
        }

        self.rarity = rarity.clone();
    }
}
