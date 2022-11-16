use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    NodeMenu, NodeMenuPlugin, NodePlugins, Nodes,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<MathNodes>::default())
        .add_plugin(NodeMenuPlugin::<MathMenu, MathNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default)]
struct MathMenu;

impl NodeMenu for MathMenu {
    type Nodes = MathNodes;

    fn options(&self) -> Vec<(String, Self::Nodes)> {
        vec![
            ("Value".to_string(), MathNodes::Value(1.0)),
            ("Add".to_string(), MathNodes::Add),
            ("Multiply".to_string(), MathNodes::Mult),
            ("Print".to_string(), MathNodes::Print),
        ]
    }
}

#[derive(Clone, Copy)]
enum MathNodes {
    Add,
    Mult,
    Print,
    Value(f32),
}

impl Default for MathNodes {
    fn default() -> Self {
        Self::Value(0.0)
    }
}

impl Nodes for MathNodes {
    type NodeIO = f32;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match *self {
            MathNodes::Add => {
                let a: f32 = inputs["a"];
                let b: f32 = inputs["b"];

                a + b
            }
            MathNodes::Mult => {
                let a: f32 = inputs["a"];
                let b: f32 = inputs["b"];

                a * b
            }
            MathNodes::Print => {
                let value = inputs["value"];

                println!("{:?}", value);
                value
            }
            MathNodes::Value(value) => value,
        }
    }
}

impl Into<NodeTemplate<MathNodes>> for MathNodes {
    fn into(self) -> NodeTemplate<MathNodes> {
        match self {
            Self::Add => NodeTemplate {
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
                output_label: Some("result".to_string()),
                node: self,
                ..default()
            },
            Self::Mult => NodeTemplate {
                title: "Multiply".to_string(),
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
                node: self,
                output_label: Some("value".to_string()),
                ..default()
            },
            Self::Print => NodeTemplate {
                title: "Print".to_string(),
                inputs: Some(vec![NodeIOTemplate {
                    label: "value".to_string(),
                    ..Default::default()
                }]),
                node: self,
                ..default()
            },
            Self::Value(value) => NodeTemplate {
                title: "Value".to_string(),
                output_label: Some(format!("{}", value)),
                node: self,
                ..Default::default()
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let template: NodeTemplate<MathNodes> = MathNodes::Print.into();

    commands.spawn().insert(template);
}
