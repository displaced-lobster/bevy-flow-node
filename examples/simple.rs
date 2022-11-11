use bevy::prelude::*;

use bevy_node_editor::{
    Node,
    NodeIO,
    NodeInput,
    NodeOutput,
    NodePlugins,
    NodeResolver,
    node::{NodeIOTemplate, NodeTemplate},
    OutputNode,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<NoOpNodeResolver>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default)]
struct NoOpNodeResolver;

impl NodeResolver for NoOpNodeResolver {
    fn resolve(
        &self,
        _entity:Entity,
        _node: &Node,
        _q_nodes: &Query<(Entity, &Node), Without<OutputNode>>,
        _q_inputs: &Query<(&Parent, &NodeInput)>,
        _q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> NodeIO {
        NodeIO::default()
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert(NodeTemplate {
            position: Vec2::new(-200.0, 0.0),
            title: "Node 1".to_string(),
            output: Some(NodeIOTemplate {
                label: "Output".to_string(),
                ..default()
            }),
            ..default()
        });

    commands
        .spawn()
        .insert(NodeTemplate {
            position: Vec2::new(200.0, 0.0),
            title: "Node 2".to_string(),
            inputs: Some(vec![
                NodeIOTemplate {
                    label: "Input".to_string(),
                    ..default()
                },
            ]),
            ..default()
        });
}
