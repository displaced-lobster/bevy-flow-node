use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef},
};

#[derive(Component)]
pub struct CursorCamera;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_system(update_cursor_position);
    }
}

#[derive(Default, Resource)]
pub struct CursorPosition {
    pub x: f32,
    pub y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
}

impl CursorPosition {
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

fn update_cursor_position(
    mut cursor: ResMut<CursorPosition>,
    q_primary: Query<&Window, With<PrimaryWindow>>,
    q_windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<CursorCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let wnd = if let RenderTarget::Window(WindowRef::Entity(id)) = camera.target {
        q_windows.get(id).unwrap()
    } else {
        q_primary.get_single().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width(), wnd.height());
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.x = world_pos.x;
        cursor.y = world_pos.y;
        cursor.screen_x = screen_pos.x;
        cursor.screen_y = window_size.y - screen_pos.y;
    }
}
