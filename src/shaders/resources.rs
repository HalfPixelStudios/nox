use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::std140::{AsStd140, Std140},
        renderer::RenderQueue,
        *,
    },
};
use bevy_rapier2d::prelude::*;

use super::super::player::Player;

#[derive(Deref)]
pub struct ExtractedVelocity(pub Vec2);

pub fn extract_velocity(mut cmd: Commands, query: Query<&Velocity, With<Player>>) {
    let velocity = query.single();
    cmd.insert_resource(ExtractedVelocity(velocity.linvel));
}
