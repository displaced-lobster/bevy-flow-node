use bevy::{prelude::*, winit::WinitSettings};

use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    NodeMenu, NodeMenuPlugin, NodePlugins, NodeSet,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<LogicNodes>::default())
        .add_plugin(NodeMenuPlugin::<LogicMenu, LogicNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy, Default)]
enum LogicNodes {
    True,
    False,
    And,
    Or,
    Not,
    Xor,
    Nand,
    Nor,
    Xnor,
    #[default]
    Result,
}

impl Into<NodeTemplate<LogicNodes>> for LogicNodes {
    fn into(self) -> NodeTemplate<LogicNodes> {
        match self {
            Self::True => NodeTemplate {
                title: "True".to_string(),
                output_label: Some("true".to_string()),
                node: self,
                ..default()
            },
            Self::False => NodeTemplate {
                title: "False".to_string(),
                output_label: Some("false".to_string()),
                node: self,
                ..default()
            },
            Self::And => NodeTemplate {
                title: "And".to_string(),
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
                output_label: Some("a & b".to_string()),
                node: self,
                ..default()
            },
            Self::Or => NodeTemplate {
                title: "Or".to_string(),
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
                output_label: Some("a | b".to_string()),
                node: self,
                ..default()
            },
            Self::Not => NodeTemplate {
                title: "Not".to_string(),
                inputs: Some(vec![NodeIOTemplate {
                    label: "a".to_string(),
                    ..default()
                }]),
                output_label: Some("!a".to_string()),
                node: self,
                ..default()
            },
            Self::Xor => NodeTemplate {
                title: "Xor".to_string(),
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
                output_label: Some("a ^ b".to_string()),
                node: self,
                ..default()
            },
            Self::Nand => NodeTemplate {
                title: "Nand".to_string(),
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
                output_label: Some("!(a & b)".to_string()),
                node: self,
                ..default()
            },
            Self::Nor => NodeTemplate {
                title: "Nor".to_string(),
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
                output_label: Some("!(a | b)".to_string()),
                node: self,
                ..default()
            },
            Self::Xnor => NodeTemplate {
                title: "Xnor".to_string(),
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
                output_label: Some("!(a ^ b)".to_string()),
                node: self,
                ..default()
            },
            Self::Result => NodeTemplate {
                title: "Result".to_string(),
                inputs: Some(vec![NodeIOTemplate {
                    label: "a".to_string(),
                    ..default()
                }]),
                node: self,
                ..default()
            },
        }
    }
}

impl NodeSet for LogicNodes {
    type NodeIO = bool;

    fn resolve(&self, inputs: &std::collections::HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match self {
            LogicNodes::True => true,
            LogicNodes::False => false,
            LogicNodes::And => inputs["a"] && inputs["b"],
            LogicNodes::Or => inputs["a"] || inputs["b"],
            LogicNodes::Not => !inputs["a"],
            LogicNodes::Xor => inputs["a"] ^ inputs["b"],
            LogicNodes::Nand => !(inputs["a"] && inputs["b"]),
            LogicNodes::Nor => !(inputs["a"] || inputs["b"]),
            LogicNodes::Xnor => !(inputs["a"] ^ inputs["b"]),
            LogicNodes::Result => {
                let r = inputs["a"];

                println!("{}", r);

                r
            }
        }
    }
}

#[derive(Default, Resource)]
struct LogicMenu;

impl NodeMenu<LogicNodes> for LogicMenu {
    fn build(&self, commands: &mut Commands, node: &LogicNodes) {
        let template: NodeTemplate<LogicNodes> = (*node).into();

        commands.spawn(template);
    }

    fn options(&self) -> Vec<(String, LogicNodes)> {
        vec![
            ("True".to_string(), LogicNodes::True),
            ("False".to_string(), LogicNodes::False),
            ("And".to_string(), LogicNodes::And),
            ("Or".to_string(), LogicNodes::Or),
            ("Not".to_string(), LogicNodes::Not),
            ("Xor".to_string(), LogicNodes::Xor),
            ("Nand".to_string(), LogicNodes::Nand),
            ("Nor".to_string(), LogicNodes::Nor),
            ("Xnor".to_string(), LogicNodes::Xnor),
            ("Result".to_string(), LogicNodes::Result),
        ]
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let template: NodeTemplate<LogicNodes> = LogicNodes::Result.into();

    commands.spawn(template);
}
