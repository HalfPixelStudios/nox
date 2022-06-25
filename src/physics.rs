use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::config::PPM;

pub struct PhysicsPlugin;

struct CustomPhysicsHook;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PPM))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup);
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
