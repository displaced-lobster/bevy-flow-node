use bevy::{prelude::*, render::camera::RenderTarget};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_system(update_cursor_position);
    }
}

#[derive(Default)]
pub struct CursorPosition {
    pub x: f32,
    pub y: f32,
}

impl CursorPosition {
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

fn update_cursor_position(
    mut cursor: ResMut<CursorPosition>,
    wnds: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.x = world_pos.x;
        cursor.y = world_pos.y;
    }
}
