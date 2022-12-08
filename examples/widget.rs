use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    widgets::{DisplayWidget, DisplayWidgetPlugin, TextInputWidget, TextInputWidgetPlugin},
    CursorCamera, NodeInput, NodeOutput, NodePlugins, NodeSet, NodeSlot, NodeTemplate, SlotWidget,
};
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{AddAssign, SubAssign},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<IONodes>::default())
        .add_plugin(DisplayWidgetPlugin::<IONodes>::default())
        .add_plugin(TextInputWidgetPlugin::<IONodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Default)]
struct NodeString(String);

#[derive(Clone)]
enum IONodes {
    Input(String),
    Output,
}

impl Default for IONodes {
    fn default() -> Self {
        Self::Output
    }
}

impl NodeSet for IONodes {
    type NodeIO = NodeString;

    fn resolve(
        &self,
        inputs: &HashMap<String, Self::NodeIO>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
        match self {
            IONodes::Input(s) => NodeString(s.clone()),
            IONodes::Output => inputs["input"].clone(),
        }
    }

    fn template(self) -> NodeTemplate<Self> {
        match self {
            IONodes::Input(_) => NodeTemplate {
                title: "Input".to_string(),
                outputs: Some(vec![NodeOutput::from_label("output")]),
                node: self,
                slot: Some(NodeSlot {
                    height: 20.0,
                    ..default()
                }),
                ..default()
            },
            IONodes::Output => NodeTemplate {
                title: "Output".to_string(),
                inputs: Some(vec![NodeInput::from_label("input")]),
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

impl SlotWidget<Self, DisplayWidget> for IONodes {
    fn get_widget(&self) -> Option<DisplayWidget> {
        match self {
            IONodes::Output => Some(DisplayWidget::default()),
            _ => None,
        }
    }
}

impl SlotWidget<Self, TextInputWidget<Self>> for IONodes {
    fn get_widget(&self) -> Option<TextInputWidget<Self>> {
        match self {
            IONodes::Input(_) => Some(TextInputWidget::default()),
            _ => None,
        }
    }

    fn set_value(&mut self, value: NodeString) {
        match self {
            IONodes::Input(s) => *s = value.to_string(),
            _ => {}
        }
    }
}

impl AddAssign<char> for NodeString {
    fn add_assign(&mut self, other: char) {
        self.0.push(other);
    }
}

impl SubAssign<char> for NodeString {
    fn sub_assign(&mut self, _other: char) {
        self.0.pop();
    }
}

impl Display for NodeString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for NodeString {
    fn into(self) -> String {
        self.0.clone()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CursorCamera));

    let start_value = "Hello World!".to_string();
    let mut input_template = IONodes::Input(start_value).template();
    let mut output_template = IONodes::Output.template();

    input_template.position.x = -220.0;
    output_template.position.x = 220.0;

    commands.spawn(input_template);
    commands.spawn(output_template);
}
