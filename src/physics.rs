use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::config::PPM;

pub struct PhysicsPlugin;

struct CustomPhysicsHook;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM))
            .add_startup_system(setup);
    }
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub vel: Velocity,
    pub rb: RigidBody,
    pub col: Collider,
    pub active_events: ActiveEvents,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        PhysicsBundle {
            vel: Velocity::default(),
            rb: RigidBody::Dynamic,
            col: Collider::cuboid(5., 5.),
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: 100.,
                ..default()
            },
        }
    }
}

fn setup(mut cmd: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;

    cmd.insert_resource(PhysicsHooksWithQueryResource(Box::new(
        CustomPhysicsHook {},
    )));
}

impl PhysicsHooksWithQuery<NoUserData> for CustomPhysicsHook {
    // fn filter_intersection_pair(
    //     &self,
    //     _context: PairFilterContextView,
    //     _user_data: &Query<NoUserData>
    // ) {

    // }
}
