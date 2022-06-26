use bevy::prelude::*;

use bevy::prelude::*;
use bevy::sprite::*;

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

pub struct AnchorXAxisLens {
    pub start: f32,
    pub end: f32,
}
impl Lens<TextureAtlasSprite> for AnchorXAxisLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;

        target.anchor = Anchor::Custom(Vec2::new(value, target.anchor.as_vec().y));
    }
}
pub struct AnchorYAxisLens {
    pub start: f32,
    pub end: f32,
}
impl Lens<TextureAtlasSprite> for AnchorYAxisLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;

        target.anchor = Anchor::Custom(Vec2::new(target.anchor.as_vec().x, value));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
#[derive(Debug)]
pub enum Action {
    IDLE,
    WALK,
}
#[derive(Debug)]
pub enum Dir {
    LEFT,
    RIGHT,
}
#[derive(Component)]
pub struct Animatable;

#[derive(Component, Debug)]
pub struct EntityState {
    pub action: Action,
    pub direction: Dir,
}
pub fn animate_sprite(
    mut query: Query<
        (&mut Transform, &mut Animator<Transform>, &mut EntityState),
        With<Animatable>,
    >,
) {
    for (mut transform, mut animator, state) in query.iter_mut() {
        match state.action {
            Action::IDLE => {
                animator.stop();
                transform.rotation = Quat::IDENTITY;
            }
            Action::WALK => animator.state = AnimatorState::Playing,
            _ => (),
        };
    }
}

//pub fn animate_sprite(
//    time: Res<Time>,
//    texture_atlases: Res<Assets<TextureAtlas>>,
//    mut query: Query<(
//        &mut AnimationTimer,
//        &mut TextureAtlasSprite,
//        &Handle<TextureAtlas>,
//        &mut EntityState),
//        With<Animatable>>) {
//    for (mut timer, mut sprite, texture_atlas_handle, state) in query.iter_mut() {
//        timer.tick(time.delta());
//        if timer.just_finished() {
//            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
//            sprite.flip_x = match state.direction {
//                Dir::LEFT => true,
//                Dir::RIGHT => false,
//            }
//        }
//    }
//}
