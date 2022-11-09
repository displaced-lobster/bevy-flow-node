use bevy::prelude::*;

use bevy_node_editor::{
    FlowNodePlugins,
    node::FlowNodeTemplate,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(FlowNodePlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert(FlowNodeTemplate {
            position: Vec2::new(-200.0, 0.0),
            title: "Node 1".to_string(),
            n_outputs: 2,
            ..default()
        });

    commands
        .spawn()
        .insert(FlowNodeTemplate {
            position: Vec2::new(200.0, 0.0),
            title: "Node 2".to_string(),
            n_inputs: 1,
            ..default()
        });
}
