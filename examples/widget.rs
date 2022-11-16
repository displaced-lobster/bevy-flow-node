use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    widget::{ReceiveWidgetValue, Widget, WidgetPlugin},
    widgets::{TextInputWidget, TextInputWidgetPlugin},
    NodeEvent, NodePlugins, NodeSlot, Nodes,
};
use std::{collections::HashMap, fmt::Display, ops::AddAssign};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<IONodes>::default())
        .add_plugin(TextInputWidgetPlugin::<IONodes>::default())
        .add_plugin(WidgetPlugin::<IONodes, DisplayWidget>::default())
        .add_startup_system(setup)
        .add_system(update_display_widget)
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

impl Nodes for IONodes {
    type NodeIO = NodeString;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO {
        match self {
            IONodes::Input(s) => NodeString(s.clone()),
            IONodes::Output => {
                println!("Output: {}", inputs["input"]);
                inputs["input"].clone()
            }
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

#[derive(Component, Clone, Copy, Default)]
struct DisplayWidget {
    size: Vec2,
}

impl Widget<IONodes> for DisplayWidget {
    fn build(
        &mut self,
        commands: &mut Commands,
        area: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let text_style_title = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        };

        self.size = area;

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section("Hello World", text_style_title),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            })
            .id()
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let start_value = "Hello World!".to_string();

    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(NodeTemplate::<IONodes> {
            position: Vec2::new(-220.0, 0.0),
            title: "Input".to_string(),
            output_label: Some("output".to_string()),
            node: IONodes::Input(start_value.clone()),
            slot: Some(NodeSlot {
                height: 20.0,
                ..default()
            }),
            ..default()
        })
        .insert(TextInputWidget::<IONodes> {
            size: Vec2::new(200.0, 20.0),
            value: NodeString(start_value),
            ..default()
        });

    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(NodeTemplate::<IONodes> {
            position: Vec2::new(220.0, 0.0),
            title: "Output".to_string(),
            inputs: Some(vec![NodeIOTemplate {
                label: "input".to_string(),
                ..default()
            }]),
            node: IONodes::Output,
            slot: Some(NodeSlot {
                height: 20.0,
                ..default()
            }),
            ..default()
        })
        .insert(DisplayWidget::default());
}

fn update_display_widget(
    mut ev_node: EventReader<NodeEvent<IONodes>>,
    mut q_text: Query<(&Parent, &mut Text)>,
    q_widget: Query<Entity, With<DisplayWidget>>,
) {
    for ev in ev_node.iter() {
        if let NodeEvent::Resolved(value) = ev {
            for entity in q_widget.iter() {
                for (parent, mut text) in q_text.iter_mut() {
                    if parent.get() == entity {
                        text.sections[0].value = value.to_string();
                    }
                }
            }
        }
    }
}
