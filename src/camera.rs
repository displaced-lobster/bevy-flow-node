use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};

use crate::cursor::CursorCamera;

pub struct PanCameraPlugin;

impl Plugin for PanCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (move_camera, zoom_camera));
    }
}

#[derive(Component)]
pub struct PanCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CursorCamera, PanCamera));
}

fn move_camera(
    mouse: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<PanCamera>>,
    mut previous_pos: Local<Option<Vec2>>,
) {
    let window = q_window.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let current_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };
    let delta_pos = current_pos - previous_pos.unwrap_or(current_pos);

    if mouse.pressed(MouseButton::Middle) {
        for (mut transform, projection) in query.iter_mut() {
            if let ScalingMode::Fixed { width, height } = projection.scaling_mode {
                let proj_size = Vec2::new(width, height) * projection.scale;
                let world_pos_ratio = proj_size / window_size;
                let delta_world = delta_pos * world_pos_ratio;

                transform.translation -= delta_world.extend(0.0);
            }
        }
    }

    *previous_pos = Some(current_pos);
}

fn zoom_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<PanCamera>>,
) {
    let scroll = ev_scroll.read().map(|ev| ev.y).sum::<f32>();

    if scroll == 0.0 {
        return;
    }

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        log_scale -= scroll * 0.25;
        projection.scale = log_scale.exp();
    }
}
