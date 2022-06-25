use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);
impl Health {
    fn take(&mut self,dmg:i32) {
        self.0-=dmg;
        if self.0<=0{
            self.0=0;
        }

    }
}

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Decay{
    pub timer: Timer,
}

pub fn decay_system(mut cmd: Commands, time: Res<Time>,mut query: Query<(Entity, &mut Decay)>){
    for(entity,mut decay) in query.iter_mut() {
       decay.timer.tick(time.delta());
       if decay.timer.just_finished() {
           cmd.entity(entity).despawn();
       }
    }
}
