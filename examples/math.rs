use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    widgets::{DisplayWidget, DisplayWidgetPlugin, InputWidget, InputWidgetPlugin, NumberInput},
    NodeInput, NodeMenu, NodeMenuPlugin, NodeOutput, NodePlugins, NodeSet, NodeSlot, NodeTemplate,
    PanCameraPlugin, SlotWidget,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<MathNodes>::default())
        .add_plugin(PanCameraPlugin)
        .add_plugin(NodeMenuPlugin::<MathMenu, MathNodes>::default())
        .add_plugin(DisplayWidgetPlugin::<MathNodes>::default())
        .add_plugin(InputWidgetPlugin::<MathNodes, NumberInput>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Resource)]
struct MathMenu;

impl NodeMenu<MathNodes> for MathMenu {
    fn options(&self) -> Vec<(String, MathNodes)> {
        vec![
            (
                "Value".to_string(),
                MathNodes::Value(NumberInput::default()),
            ),
            ("Add".to_string(), MathNodes::Add),
            ("Multiply".to_string(), MathNodes::Mult),
            ("Output".to_string(), MathNodes::Output),
        ]
    }
}

#[derive(Clone)]
enum MathNodes {
    Add,
    Mult,
    Output,
    Value(NumberInput),
}

impl Default for MathNodes {
    fn default() -> Self {
        Self::Value(NumberInput::default())
    }
}

impl NodeSet for MathNodes {
    type NodeIO = f32;

    fn resolve(
        &self,
        inputs: &HashMap<String, Self::NodeIO>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
        match self {
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
            MathNodes::Output => inputs["value"],
            MathNodes::Value(value) => value.value,
        }
    }

    fn template(self) -> NodeTemplate<Self> {
        match self {
            Self::Add => NodeTemplate {
                title: "Add".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                outputs: Some(vec![NodeOutput::from_label("result")]),
                node: self,
                ..default()
            },
            Self::Mult => NodeTemplate {
                title: "Multiply".to_string(),
                inputs: Some(vec![NodeInput::from_label("a"), NodeInput::from_label("b")]),
                node: self,
                outputs: Some(vec![NodeOutput::from_label("result")]),
                ..default()
            },
            Self::Output => NodeTemplate {
                title: "Output".to_string(),
                inputs: Some(vec![NodeInput::from_label("value")]),
                node: self,
                slot: Some(NodeSlot {
                    height: 20.0,
                    ..default()
                }),
                ..default()
            },
            Self::Value(_) => NodeTemplate {
                title: "Value".to_string(),
                outputs: Some(vec![NodeOutput::from_label("value")]),
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

impl SlotWidget<Self, DisplayWidget> for MathNodes {
    fn get_widget(&self) -> Option<DisplayWidget> {
        match self {
            MathNodes::Output => Some(DisplayWidget::default()),
            _ => None,
        }
    }
}

impl SlotWidget<Self, InputWidget<NumberInput>> for MathNodes {
    fn get_widget(&self) -> Option<InputWidget<NumberInput>> {
        match self {
            MathNodes::Value(_) => Some(InputWidget::default()),
            _ => None,
        }
    }

    fn set_value(&mut self, value: NumberInput) {
        match self {
            MathNodes::Value(v) => *v = value,
            _ => {}
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(MathNodes::Output.template());
}
