use bevy::prelude::*;

pub struct InventoryResource {
    pub primary_weapon: String,
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
        }
    }
}
