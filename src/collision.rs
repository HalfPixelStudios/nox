use bevy::{ecs::world::EntityRef, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct OnCollide {
    pub handler: fn(me: &mut EntityRef, other: &mut EntityRef) -> (),
}

pub fn handle_collisions(mut events: EventReader<CollisionEvent>, world: &World) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = event {
            println!("collision started: {:?} {:?}", e1, e2);
            let mut e1_ref = world.get_entity(*e1).unwrap();
            let mut e2_ref = world.get_entity(*e2).unwrap();

            if e1_ref.contains::<OnCollide>() {
                let handler = e1_ref.get::<OnCollide>().unwrap().handler;
                handler(&mut e1_ref, &mut e2_ref);
            }
            if e2_ref.contains::<OnCollide>() {
                let handler = e2_ref.get::<OnCollide>().unwrap().handler;
                handler(&mut e2_ref, &mut e1_ref);
            }
        }
    }
}
