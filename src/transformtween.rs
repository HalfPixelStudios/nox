use bevy::prelude::*;

use bevy_tweening::{lens::*, *};

pub struct TransformDimensionLens {
    pub start: f32,
    pub end: f32,
    pub freeze_width: bool,
    pub freeze_height: bool,
}
impl Lens<Transform> for TransformDimensionLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        target.scale = Vec3::new(
            self.start + (self.end - self.start) * ratio,
            self.start + (self.end - self.start) * ratio,
            0.,
        );
    }
}
