use bevy::{prelude::*, winit::WinitSettings};
use bevy_flow_node::{
    widgets::{DisplayWidget, DisplayWidgetPlugin, InputWidget, InputWidgetPlugin, NumberInput},
    FlowNodeInput,
    FlowNodeMenu,
    FlowNodeMenuPlugin,
    FlowNodeOutput,
    FlowNodePlugins,
    FlowNodeSet,
    FlowNodeSlot,
    FlowNodeTemplate,
    PanCameraPlugin,
    SlotWidget,
};
use std::collections::HashMap;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(FlowNodePlugins::<MathNodes>::default())
        .add_plugin(PanCameraPlugin)
        .add_plugin(FlowNodeMenuPlugin::<MathMenu, MathNodes>::default())
        .add_plugin(DisplayWidgetPlugin::<MathNodes>::default())
        .add_plugin(InputWidgetPlugin::<MathNodes, NumberInput>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Resource)]
struct MathMenu;

impl FlowNodeMenu<MathNodes> for MathMenu {
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

impl FlowNodeSet for MathNodes {
    type NodeIO = f32;

    fn resolve(
        &self,
        inputs: HashMap<String, Option<Self::NodeIO>>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
        let a = inputs.get("a").unwrap_or(&None).unwrap_or(0.0);
        let b = inputs.get("b").unwrap_or(&None).unwrap_or(0.0);

        match self {
            MathNodes::Add => a + b,
            MathNodes::Mult => a * b,
            MathNodes::Output => inputs["value"].unwrap_or(0.0),
            MathNodes::Value(value) => value.value,
        }
    }

    fn template(self) -> FlowNodeTemplate<Self> {
        match self {
            Self::Add => FlowNodeTemplate {
                title: "Add".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                outputs: Some(vec![FlowNodeOutput::from_label("result")]),
                node: self,
                ..default()
            },
            Self::Mult => FlowNodeTemplate {
                title: "Multiply".to_string(),
                inputs: Some(vec![
                    FlowNodeInput::from_label("a"),
                    FlowNodeInput::from_label("b"),
                ]),
                node: self,
                outputs: Some(vec![FlowNodeOutput::from_label("result")]),
                ..default()
            },
            Self::Output => FlowNodeTemplate {
                title: "Output".to_string(),
                inputs: Some(vec![FlowNodeInput::from_label("value")]),
                node: self,
                slot: Some(FlowNodeSlot {
                    height: 20.0,
                    ..default()
                }),
                ..default()
            },
            Self::Value(_) => FlowNodeTemplate {
                title: "Value".to_string(),
                outputs: Some(vec![FlowNodeOutput::from_label("value")]),
                node: self,
                slot: Some(FlowNodeSlot {
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
        if let MathNodes::Value(v) = self {
            *v = value;
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(MathNodes::Output.template());
}
