use bevy::prelude::*;

use bevy_node_editor::{
    Node,
    NodeIO,
    NodeInput,
    NodeOutput,
    NodePlugins,
    NodeResolver,
    NodeType,
    node::{NodeIOTemplate, NodeTemplate},
    OutputNode,
};

const ADDITION_NODE: NodeType = NodeType(1);
const VALUE_NODE: NodeType = NodeType(2);
const PRINT_NODE: NodeType = NodeType(3);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<MathNodeResolver>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default)]
struct AdditionNode {
    position: Vec2,
}

impl AdditionNode {
    fn to_template(&self) -> NodeTemplate {
        NodeTemplate {
            node_type: ADDITION_NODE,
            position: self.position,
            title: "Add".to_string(),
            inputs: Some(vec![
                NodeIOTemplate {
                    label: "a".to_string(),
                    ..default()
                },
                NodeIOTemplate {
                    label: "b".to_string(),
                    ..default()
                },
            ]),
            output: Some(NodeIOTemplate {
                label: "result".to_string(),
                ..default()
            }),
            ..default()
        }
    }
}

#[derive(Default)]
struct MathNodeResolver;

impl NodeResolver for MathNodeResolver {
    fn resolve(
        &self,
        entity:Entity,
        node: &Node,
        q_nodes: &Query<(Entity, &Node), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> NodeIO {
        let inputs = node.get_inputs(
            self,
            entity,
            q_nodes,
            q_inputs,
            q_outputs,
        );

        match node.node_type {
            ADDITION_NODE => {
                let a: f32 = inputs["a"].into();
                let b: f32 = inputs["b"].into();

                NodeIO::F32(a + b)
            },
            VALUE_NODE => {
                node.value
            },
            PRINT_NODE => {
                println!("{:?}", inputs["value"]);
                NodeIO::None
            },
            _ => NodeIO::None,
        }
    }
}

#[derive(Default)]
struct FloatNode {
    value: f32,
    position: Vec2,
}

impl FloatNode {
    fn new(value: f32, position: Vec2) -> Self {
        Self { value, position }
    }

    fn to_template(&self) -> NodeTemplate {
        NodeTemplate {
            node_type: VALUE_NODE,
            title: "Value".to_string(),
            output: Some(NodeIOTemplate {
                label: format!("{}", self.value),
                ..default()
            }),
            position: self.position,
            value: NodeIO::F32(self.value),
            ..default()
        }
    }
}

#[derive(Default)]
struct PrintNode {
    position: Vec2,
}

impl PrintNode {
    fn to_template(&self) -> NodeTemplate {
        NodeTemplate {
            node_type: PRINT_NODE,
            title: "Print".to_string(),
            inputs: Some(vec![
                NodeIOTemplate {
                    label: "value".to_string(),
                    ..default()
                },
            ]),
            position: self.position,
            ..default()
        }
    }
}

#[derive(Component)]
struct Print;

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert(FloatNode::new(5.0, Vec2::new(-150.0, 100.0)).to_template());

    commands
        .spawn()
        .insert(FloatNode::new(7.0, Vec2::new(-150.0, -100.0)).to_template());

    commands
        .spawn()
        .insert(AdditionNode { position: Vec2::new(150.0, 0.0) }.to_template());

    commands
        .spawn()
        .insert(PrintNode { position: Vec2::new(450.0, 0.0) }.to_template())
        .insert(Print);
}
