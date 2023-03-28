use bevy::{prelude::*, winit::WinitSettings};
use bevy_node_editor::{CursorCamera, NodeInput, NodeOutput, NodePlugins, NodeSet, NodeTemplate};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<TemplateNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy, Default)]
struct TemplateNodes;

impl NodeSet for TemplateNodes {
    type NodeIO = ();

    fn resolve(
        &self,
        _inputs: std::collections::HashMap<String, Option<Self::NodeIO>>,
        _output: Option<&str>,
    ) -> Self::NodeIO {
    }

    fn template(self) -> NodeTemplate<Self> {
        NodeTemplate::default()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CursorCamera));

    let input_prefix = "Input node ";
    let output_prefix = "Output node ";
    let range = 3;

    for i in -range..range {
        let item_number = i + range + 1;

        commands.spawn(NodeTemplate::<TemplateNodes> {
            position: Vec2::new(-200.0, -75.0 * i as f32),
            title: format!("{} {}", output_prefix, item_number),
            outputs: Some(vec![NodeOutput::from_label("")]),
            ..default()
        });

        commands.spawn(NodeTemplate::<TemplateNodes> {
            position: Vec2::new(200.0, -75.0 * i as f32),
            title: format!("{} {}", input_prefix, item_number),
            inputs: Some(vec![NodeInput::from_label("")]),
            ..default()
        });
    }
}
