use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    widget::ReceiveWidgetValue,
    widgets::{DisplayWidget, DisplayWidgetPlugin, TextInputWidget, TextInputWidgetPlugin},
    NodeMenu, NodeMenuPlugin, NodePlugins, NodeSlot, Nodes,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<MathNodes>::default())
        .add_plugin(NodeMenuPlugin::<MathMenu, MathNodes>::default())
        .add_plugin(DisplayWidgetPlugin::<MathNodes>::default())
        .add_plugin(TextInputWidgetPlugin::<MathNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Resource)]
struct MathMenu;

impl NodeMenu<MathNodes> for MathMenu {
    fn build(&self, commands: &mut Commands, _asset_server: &Res<AssetServer>, node: &MathNodes) {
        let template: NodeTemplate<MathNodes> = (*node).into();

        let entity = commands.spawn(template).id();

        match node {
            MathNodes::Value(_) => {
                commands
                    .entity(entity)
                    .insert(TextInputWidget::<MathNodes> {
                        size: Vec2::new(100.0, 20.0),
                        value: MathIO(0.0),
                        ..default()
                    });
            }
            MathNodes::Print => {
                commands.entity(entity).insert(DisplayWidget {
                    size: Vec2::new(100.0, 20.0),
                    ..default()
                });
            }
            _ => {}
        }
    }

    fn options(&self) -> Vec<(String, MathNodes)> {
        vec![
            ("Value".to_string(), MathNodes::Value(MathIO::default())),
            ("Add".to_string(), MathNodes::Add),
            ("Multiply".to_string(), MathNodes::Mult),
            ("Print".to_string(), MathNodes::Print),
        ]
    }
}

#[derive(Clone, Copy, Default, Deref)]
struct MathIO(f32);

impl std::ops::AddAssign<char> for MathIO {
    fn add_assign(&mut self, rhs: char) {
        if rhs.is_digit(10) {
            self.0 *= 10.0;
            self.0 += rhs.to_digit(10).unwrap() as f32;
        }
    }
}

impl std::fmt::Display for MathIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f32> for MathIO {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl Into<String> for MathIO {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Copy)]
enum MathNodes {
    Add,
    Mult,
    Print,
    Value(MathIO),
}

impl Default for MathNodes {
    fn default() -> Self {
        Self::Value(MathIO::default())
    }
}

impl Nodes for MathNodes {
    type NodeIO = MathIO;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match *self {
            MathNodes::Add => {
                let a: f32 = *inputs["a"];
                let b: f32 = *inputs["b"];

                MathIO(a + b)
            }
            MathNodes::Mult => {
                let a: f32 = *inputs["a"];
                let b: f32 = *inputs["b"];

                MathIO(a * b)
            }
            MathNodes::Print => {
                let value = inputs["value"];

                println!("{}", value);
                value
            }
            MathNodes::Value(value) => value,
        }
    }
}

impl ReceiveWidgetValue<MathNodes> for MathNodes {
    fn receive_value(&mut self, value: MathIO) {
        match self {
            MathNodes::Value(io) => *io = value,
            _ => {}
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
                    ..default()
                }]),
                node: self,
                slot: Some(NodeSlot {
                    height: 20.0,
                    ..default()
                }),
                ..default()
            },
            Self::Value(_) => NodeTemplate {
                title: "Value".to_string(),
                output_label: Some("value".to_string()),
                node: self,
                slot: Some(NodeSlot {
                    height: 20.0,
                    ..default()
                }),
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let template: NodeTemplate<MathNodes> = MathNodes::Print.into();

    commands.spawn((
        template,
        DisplayWidget {
            size: Vec2::new(100.0, 20.0),
            ..default()
        },
    ));
}
