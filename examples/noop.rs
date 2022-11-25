use bevy::{prelude::*, winit::WinitSettings};

use bevy_node_editor::{NodeInput, NodePlugins, NodeSet, NodeTemplate};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<NoOpNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy, Default)]
struct NoOpNodes;

impl NodeSet for NoOpNodes {
    type NodeIO = ();

    fn resolve(&self, _inputs: &std::collections::HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        ()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeTemplate::<NoOpNodes> {
        position: Vec2::new(-200.0, 0.0),
        title: "Node 1".to_string(),
        output_label: Some("Output".to_string()),
        ..default()
    });

    commands.spawn(NodeTemplate::<NoOpNodes> {
        position: Vec2::new(200.0, 0.0),
        title: "Node 2".to_string(),
        inputs: Some(vec![NodeInput::from_label("Input")]),
        ..default()
    });
}
