use bevy::prelude::*;

use super::prefabs::weapon::*;
use super::souls::*;
use super::weapon::Weapon;

enum Selected {
    Primary,
    Secondary,
}

pub struct InventoryResource {
    selected_weapon: Selected,
    pub primary_weapon: Weapon,
    secondary_weapon: Weapon, // armour
    pub closest: Option<Equipable>,
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
            selected_weapon: Selected::Primary,
            primary_weapon: flamethrower_staff(),
            secondary_weapon: wooden_bow(),
            closest: None,
        }
    }
}

impl InventoryResource {
    pub fn current_weapon(&self) -> &Weapon {
        match &self.selected_weapon {
            Selected::Primary => &self.primary_weapon,
            Selected::Secondary => &self.secondary_weapon,
        }
    }
    pub fn equip_primary(&mut self) {
        self.selected_weapon = Selected::Primary;
    }
    pub fn equip_secondary(&mut self) {
        self.selected_weapon = Selected::Secondary;
    }
}
