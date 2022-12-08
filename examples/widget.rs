use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    widgets::{DisplayWidget, DisplayWidgetPlugin, InputWidget, InputWidgetPlugin},
    CursorCamera,
    NodeInput,
    NodeOutput,
    NodePlugins,
    NodeSet,
    NodeSlot,
    NodeTemplate,
    SlotWidget,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<IONodes>::default())
        .add_plugin(DisplayWidgetPlugin::<IONodes>::default())
        .add_plugin(InputWidgetPlugin::<IONodes, String>::default())
        .add_startup_system(setup)
        .run();
}

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
    type NodeIO = String;

    fn resolve(
        &self,
        inputs: &HashMap<String, Self::NodeIO>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
        match self {
            IONodes::Input(s) => s.clone(),
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

impl SlotWidget<Self, InputWidget<String>> for IONodes {
    fn get_widget(&self) -> Option<InputWidget<String>> {
        match self {
            IONodes::Input(_) => Some(InputWidget::default()),
            _ => None,
        }
    }

    fn set_value(&mut self, value: String) {
        match self {
            IONodes::Input(s) => *s = value,
            _ => {}
        }
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
