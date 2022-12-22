use bevy::{prelude::*, winit::WinitSettings};

use bevy_node_editor::{
    NodeInput,
    NodeMenu,
    NodeMenuPlugin,
    NodeOutput,
    NodePlugins,
    NodeSet,
    NodeTemplate,
    PanCameraPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<LogicNodes>::default())
        .add_plugin(PanCameraPlugin)
        .add_plugin(NodeMenuPlugin::<LogicMenu, LogicNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy, Default)]
enum LogicNodes {
    Input,
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

impl NodeSet for LogicNodes {
    type NodeIO = bool;

    fn resolve(
        &self,
        inputs: std::collections::HashMap<String, Option<Self::NodeIO>>,
        output: Option<&str>,
    ) -> Self::NodeIO {
        let a = inputs["a"].unwrap_or(false);
        let b = inputs.get("b").unwrap_or(&None).unwrap_or(false);

        match self {
            LogicNodes::Input => output.unwrap() == "true",
            LogicNodes::And => a && b,
            LogicNodes::Or => a || b,
            LogicNodes::Not => !a,
            LogicNodes::Xor => a ^ b,
            LogicNodes::Nand => !(a && b),
            LogicNodes::Nor => !(a || b),
            LogicNodes::Xnor => !(a ^ b),
            LogicNodes::Result => {
                let r = a;

                println!("{}", r);

                r
            }
        }
    }

    fn template(self) -> NodeTemplate<Self> {
        match self {
            Self::Input => NodeTemplate {
                title: "Input".to_string(),
                outputs: Some(vec![
                    NodeOutput::from_label("true"),
                    NodeOutput::from_label("false"),
                ]),
                node: self,
                ..default()
            },
            Self::And => NodeTemplate {
                title: "And".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("a & b")]),
                node: self,
                ..default()
            },
            Self::Or => NodeTemplate {
                title: "Or".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("a | b")]),
                node: self,
                ..default()
            },
            Self::Not => NodeTemplate {
                title: "Not".to_string(),
                inputs: Some(vec![NodeInput::from_label("a")]),
                outputs: Some(vec![NodeOutput::from_label("!a")]),
                node: self,
                ..default()
            },
            Self::Xor => NodeTemplate {
                title: "Xor".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("a ^ b")]),
                node: self,
                ..default()
            },
            Self::Nand => NodeTemplate {
                title: "Nand".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("!(a & b)")]),
                node: self,
                ..default()
            },
            Self::Nor => NodeTemplate {
                title: "Nor".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("!(a | b)")]),
                node: self,
                ..default()
            },
            Self::Xnor => NodeTemplate {
                title: "Xnor".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("!(a ^ b)")]),
                node: self,
                ..default()
            },
            Self::Result => NodeTemplate {
                title: "Result".to_string(),
                inputs: Some(vec![NodeInput::from_label("a")]),
                node: self,
                ..default()
            },
        }
    }
}

#[derive(Default, Resource)]
struct LogicMenu;

impl NodeMenu<LogicNodes> for LogicMenu {
    fn options(&self) -> Vec<(String, LogicNodes)> {
        vec![
            ("Input".to_string(), LogicNodes::Input),
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
    commands.spawn(LogicNodes::Result.template());
}
