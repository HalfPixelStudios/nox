use bevy::prelude::*;

#[derive(Component)]
pub enum Light {
    PointLight {
        radius: f32,
        intensity: f32,
    },
    AmbientLight {
        color: Color,
        intensity: f32,
    },
    SpotLight {
        direction: Vec2,
        cone_angle: f32,
        intensity: f32,
    }
}

#[derive(Bundle)]
pub struct LightBundle {
    light: Light,
    transform: Transform
}
