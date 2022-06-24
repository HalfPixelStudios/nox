use bevy::prelude::*;

#[derive(Debug, Component)]
struct MainCamera;

pub struct Cursor(pub Vec2);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Cursor(Vec2::ZERO))
            .add_system(cursor_system);
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
