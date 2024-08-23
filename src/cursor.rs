use bevy::prelude::*;

#[derive(Component)]
pub struct CursorCamera;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_systems(Update, update_cursor_position);
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
    q_windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<CursorCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let window = q_windows.single();

    if let Some(screen_position) = window.cursor_position() {
        if let Some(world_position) = camera
            .viewport_to_world(camera_transform, screen_position)
            .map(|ray| ray.origin.truncate())
        {
            cursor.x = world_position.x;
            cursor.y = world_position.y;
            cursor.screen_x = screen_position.x;
            cursor.screen_y = screen_position.y;
        }
    }
}
