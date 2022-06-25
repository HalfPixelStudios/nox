use bevy::prelude::*;

#[derive(Debug, Component)]
struct MainCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct Cursor(pub Vec2);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Cursor(Vec2::ZERO))
            .add_system(cursor_system)
            .add_system(camera_controller);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

// from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn cursor_system(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    let (camera, transform) = query.single();

    let win = windows.get_primary().unwrap();

    if let Some(pos) = win.cursor_position() {
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);
        let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.0 = world_pos.truncate();
    }
}
fn camera_controller(entity_query:Query<&mut GlobalTransform, (With<CameraFollow>,Without<MainCamera>)>,
                     mut camera_query:Query<(&mut Camera, &mut GlobalTransform), (With<MainCamera>,Without<CameraFollow>)>){
    let (mut camera,mut cam_transform )= camera_query.single_mut(); 
    for(transform) in entity_query.iter(){
        cam_transform.translation.x = transform.translation.x;
        cam_transform.translation.y = transform.translation.y;

        
        
    }
}
