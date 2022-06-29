use bevy::prelude::*;

pub fn find_collider<'a>(
    target: Entity,
    e1: &'a Entity,
    e2: &'a Entity,
) -> Option<(&'a Entity, &'a Entity)> {
    if target.id() == e1.id() {
        return Some((e1, e2));
    }
    if target.id() == e2.id() {
        return Some((e2, e1));
    }
    None
}

pub fn rotation_from_dir(dir: Vec2, offset: f32) -> Quat {
    Quat::from_rotation_z(Vec2::X.angle_between(dir) + offset)
}

// TODO get_components() for query
