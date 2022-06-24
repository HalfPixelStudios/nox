use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn handle_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        println!("collision event: {:?}", event);
    }
}
