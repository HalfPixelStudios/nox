use bevy::prelude::*;

pub mod pickup_arrow;

pub struct UIPluginGroup;

impl PluginGroup for UIPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(pickup_arrow::PickupArrowPlugin);
    }
}
