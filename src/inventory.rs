use bevy::prelude::*;

struct InventoryResource {
    // primary_weapon
    // secondary_weapon
    // armour
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InventoryResource {});
    }
}
