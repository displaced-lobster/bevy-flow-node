use bevy::prelude::*;

use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    Node, NodeInput, NodeOutput, NodePlugins, Nodes, OutputNode,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<NoOpNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy, Default)]
struct NoOpNodes;

impl Nodes for NoOpNodes {
    type NodeIO = ();

    fn resolve(
        &self,
        _entity: Entity,
        _node: &Node<Self>,
        _q_nodes: &Query<(Entity, &Node<Self>), Without<OutputNode>>,
        _q_inputs: &Query<(&Parent, &NodeInput<Self>)>,
        _q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> Self::NodeIO {
        ()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn().insert(NodeTemplate::<NoOpNodes> {
        position: Vec2::new(-200.0, 0.0),
        title: "Node 1".to_string(),
        output_label: Some("Output".to_string()),
        ..default()
    });

    commands.spawn().insert(NodeTemplate::<NoOpNodes> {
        position: Vec2::new(200.0, 0.0),
        title: "Node 2".to_string(),
        inputs: Some(vec![NodeIOTemplate {
            label: "Input".to_string(),
            ..default()
        }]),
        ..default()
    });
}
