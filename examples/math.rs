use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    widgets::{DisplayWidget, DisplayWidgetPlugin, InputWidget, InputWidgetPlugin, InputWidgetValue},
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
        .add_plugin(InputWidgetPlugin::<MathNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Resource)]
struct MathMenu;

impl NodeMenu<MathNodes> for MathMenu {
    fn options(&self) -> Vec<(String, MathNodes)> {
        vec![
            ("Value".to_string(), MathNodes::Value(MathIO::default())),
            ("Add".to_string(), MathNodes::Add),
            ("Multiply".to_string(), MathNodes::Mult),
            ("Output".to_string(), MathNodes::Output),
        ]
    }
}

#[derive(Clone, Default)]
struct MathIO {
    s_value: String,
    value: f32,
}

impl MathIO {
    fn new(value: f32) -> Self {
        let s_value = if value != 0.0 {
            value.to_string()
        } else {
            "".to_string()
        };

        Self { s_value, value }
    }
}

impl InputWidgetValue for MathIO {
    fn backspace(&mut self) {
        self.s_value.pop();

        if let Ok(value) = self.s_value.parse() {
            self.value = value;
        } else {
            self.value = 0.0;
        }
    }

    fn on_input(&mut self, c: char) {
        if c.is_digit(10) {
            self.s_value.push(c);

            if let Ok(value) = self.s_value.parse::<f32>() {
                self.value = value;
            }
        } else if c == '.' && !self.s_value.chars().any(|c| c == '.') {
            self.s_value.push(c);
        }
    }

    fn peek(&self) -> String {
        self.s_value.clone()
    }
}

impl std::fmt::Display for MathIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s_value)
    }
}

impl From<f32> for MathIO {
    fn from(f: f32) -> Self {
        Self {
            value: f,
            ..default()
        }
    }
}

impl Into<String> for MathIO {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone)]
enum MathNodes {
    Add,
    Mult,
    Output,
    Value(MathIO),
}

impl Default for MathNodes {
    fn default() -> Self {
        Self::Value(MathIO::default())
    }
}

impl NodeSet for MathNodes {
    type NodeIO = MathIO;

    fn resolve(
        &self,
        inputs: &HashMap<String, Self::NodeIO>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
        match self {
            MathNodes::Add => {
                let a: f32 = inputs["a"].value;
                let b: f32 = inputs["b"].value;

                MathIO::new(a + b)
            }
            MathNodes::Mult => {
                let a: f32 = inputs["a"].value;
                let b: f32 = inputs["b"].value;

                MathIO::new(a * b)
            }
            MathNodes::Output => {
                let value = inputs["value"].clone();

                value
            }
            MathNodes::Value(value) => value.clone(),
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

impl SlotWidget<Self, InputWidget<Self>> for MathNodes {
    fn get_widget(&self) -> Option<InputWidget<Self>> {
        match self {
            MathNodes::Value(_) => Some(InputWidget::default()),
            _ => None,
        }
    }

    fn set_value(&mut self, value: MathIO) {
        match self {
            MathNodes::Value(v) => *v = value,
            _ => {}
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(MathNodes::Output.template());
}
