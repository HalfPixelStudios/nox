use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
pub enum Action {
    IDLE,
    WALK,

}
pub enum Direction {
    LEFT,
    RIGHT
    
}

#[derive(Component)]
pub struct AniState {
    action:Action,
    direction:Direction
    
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut AniState
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, state) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            sprite.flip_x = match state.direction{
                Direction::LEFT => true,
                Direction::RIGHT => false
            }

        }
    }
}



