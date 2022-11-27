use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    widget::ReceiveWidgetValue,
    widgets::{DisplayWidget, DisplayWidgetPlugin, TextInputWidget, TextInputWidgetPlugin},
    CursorCamera, NodeInput, NodePlugins, NodeSet, NodeSlot, NodeTemplate,
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

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match self {
            IONodes::Input(s) => NodeString(s.clone()),
            IONodes::Output => inputs["input"].clone(),
        }
    }
}

impl ReceiveWidgetValue<IONodes> for IONodes {
    fn receive_value(&mut self, value: NodeString) {
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

    commands.spawn((
        SpatialBundle::default(),
        NodeTemplate::<IONodes> {
            position: Vec2::new(-220.0, 0.0),
            title: "Input".to_string(),
            output_label: Some("output".to_string()),
            node: IONodes::Input(start_value.clone()),
            slot: Some(NodeSlot {
                height: 20.0,
                ..default()
            }),
            ..default()
        },
        TextInputWidget::<IONodes> {
            size: Vec2::new(200.0, 20.0),
            value: NodeString(start_value),
            ..default()
        },
    ));

    commands.spawn((
        SpatialBundle::default(),
        NodeTemplate::<IONodes> {
            position: Vec2::new(220.0, 0.0),
            title: "Output".to_string(),
            inputs: Some(vec![NodeInput::from_label("input")]),
            node: IONodes::Output,
            slot: Some(NodeSlot {
                height: 20.0,
                ..default()
            }),
            ..default()
        },
        DisplayWidget::default(),
    ));
}
