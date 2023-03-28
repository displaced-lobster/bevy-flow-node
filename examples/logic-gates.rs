use bevy::{prelude::*, winit::WinitSettings};

use bevy_flow_node::{
    FlowNodeInput,
    FlowNodeMenu,
    FlowNodeMenuPlugin,
    FlowNodeOutput,
    FlowNodePlugins,
    FlowNodeSet,
    FlowNodeTemplate,
    PanCameraPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(FlowNodePlugins::<LogicNodes>::default())
        .add_plugin(PanCameraPlugin)
        .add_plugin(FlowNodeMenuPlugin::<LogicMenu, LogicNodes>::default())
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

impl FlowNodeSet for LogicNodes {
    type NodeIO = bool;

    fn resolve(
        &self,
        inputs: std::collections::HashMap<String, Option<Self::NodeIO>>,
        output: Option<&str>,
    ) -> Self::NodeIO {
        let a = inputs.get("a").unwrap_or(&None).unwrap_or(false);
        let b = inputs.get("b").unwrap_or(&None).unwrap_or(false);

        match self {
            LogicNodes::Input => output == Some("true"),
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

    fn template(self) -> FlowNodeTemplate<Self> {
        match self {
            Self::Input => FlowNodeTemplate {
                title: "Input".to_string(),
                outputs: Some(vec![
                    FlowNodeOutput::from_label("true"),
                    FlowNodeOutput::from_label("false"),
                ]),
                node: self,
                ..default()
            },
            Self::And => FlowNodeTemplate {
                title: "And".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("a & b")]),
                node: self,
                ..default()
            },
            Self::Or => FlowNodeTemplate {
                title: "Or".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("a | b")]),
                node: self,
                ..default()
            },
            Self::Not => FlowNodeTemplate {
                title: "Not".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("a")]),
                outputs: Some(vec![FlowNodeOutput::from_label("!a")]),
                node: self,
                ..default()
            },
            Self::Xor => FlowNodeTemplate {
                title: "Xor".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("a ^ b")]),
                node: self,
                ..default()
            },
            Self::Nand => FlowNodeTemplate {
                title: "Nand".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("!(a & b)")]),
                node: self,
                ..default()
            },
            Self::Nor => FlowNodeTemplate {
                title: "Nor".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("!(a | b)")]),
                node: self,
                ..default()
            },
            Self::Xnor => FlowNodeTemplate {
                title: "Xnor".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("!(a ^ b)")]),
                node: self,
                ..default()
            },
            Self::Result => FlowNodeTemplate {
                title: "Result".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("a")]),
                node: self,
                ..default()
            },
        }
    }
}

#[derive(Default, Resource)]
struct LogicMenu;

impl FlowNodeMenu<LogicNodes> for LogicMenu {
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
