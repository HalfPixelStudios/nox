use bevy::prelude::*;

#[derive(Component, Default, Clone, Eq, PartialEq)]
pub struct Health(pub i32);

impl Health {
    pub fn take(&mut self, amount: i32) {
        self.0 -= amount;
        if self.0 <= 0 {
            self.0 = 0;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.0 += amount as i32;
    }
}

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Decay {
    pub timer: Timer,
}

pub fn decay_system(mut cmd: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Decay)>) {
    for (entity, mut decay) in query.iter_mut() {
        decay.timer.tick(time.delta());
        if decay.timer.just_finished() {
            cmd.entity(entity).despawn();
        }
    }
}

pub struct Displacement {
    prev_pos: Option<Vec3>,
    total_distance: f32,
}

impl Displacement {
    pub fn new() -> Self {
        Displacement {
            prev_pos: None,
            total_distance: 0.,
        }
    }

    pub fn get_displacement(&self, new_pos: Vec3) -> f32 {
        match self.prev_pos {
            Some(prev_pos) => prev_pos.distance(new_pos),
            None => 0.,
        }
    }

    pub fn get_distance(&self) -> f32 {
        self.total_distance
    }

    pub fn update(&mut self, new_pos: Vec3) {
        if let Some(prev_pos) = self.prev_pos {
            self.total_distance += prev_pos.distance(new_pos);
        }
        self.prev_pos = Some(new_pos);
    }
}
