use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    assets::DefaultAssets,
    connection::ConnectionEvent,
    cursor::CursorPosition,
    interactions::Clicked,
    template::NodeTemplate,
};

const NODE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7843551199445678407);

pub trait NodeSet: 'static + Clone + Default + Sized + Send + Sync {
    type NodeIO: Clone + Default + Send + Sync;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>, output: Option<&str>)
        -> Self::NodeIO;
    fn template(self) -> NodeTemplate<Self>;
}

pub struct NodePlugin<N: NodeSet>(PhantomData<N>);

impl<N: NodeSet> Default for NodePlugin<N> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<N: NodeSet> Plugin for NodePlugin<N> {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            NODE_SHADER_HANDLE,
            "assets/shaders/node.wgsl",
            Shader::from_wgsl
        );
        app.insert_resource(NodeConfig::default())
            .add_event::<NodeEvent<N>>()
            .add_plugin(Material2dPlugin::<NodeMaterial>::default())
            .add_startup_system(setup)
            .add_system(activate_node)
            .add_system(delete_node::<N>)
            .add_system(drag_node::<N>.after(activate_node))
            .add_system(resolve_output_nodes::<N>);
    }
}

#[derive(Default, Resource)]
pub struct ActiveNode {
    pub count: u32,
    pub index: f32,
    pub index_reset: bool,
    pub entity: Option<Entity>,
    pub offset: Vec2,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Node<N: NodeSet>(pub(crate) N);

impl<N: NodeSet> Node<N> {
    pub fn get_inputs(
        &self,
        entity: Entity,
        q_nodes: &Query<(Entity, &Node<N>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<N>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> HashMap<String, N::NodeIO> {
        let mut inputs = HashMap::new();

        for (parent, input) in q_inputs.iter() {
            if parent.get() == entity {
                inputs.insert(
                    input.label.clone(),
                    input.get_input(q_nodes, q_inputs, q_outputs),
                );
            }
        }

        inputs
    }

    pub fn resolve(
        &self,
        entity: Entity,
        output: Option<&str>,
        q_nodes: &Query<(Entity, &Node<N>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<N>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> N::NodeIO {
        let inputs = self.get_inputs(entity, q_nodes, q_inputs, q_outputs);

        self.0.resolve(&inputs, output)
    }
}

#[derive(Resource)]
pub struct NodeConfig {
    pub border_thickness: f32,
    pub color_border: Color,
    pub color_node: Color,
    pub color_title: Color,
    pub handle_size_io: f32,
    pub padding: f32,
    pub font_size_body: f32,
    pub font_size_title: f32,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            border_thickness: 2.0,
            color_border: Color::WHITE,
            color_node: Color::rgb(0.3, 0.3, 0.3),
            color_title: Color::rgb(0.004, 0.431, 0.49),
            handle_size_io: 6.0,
            padding: 5.0,
            font_size_body: 16.0,
            font_size_title: 20.0,
        }
    }
}

#[derive(Clone, Component, Default)]
pub struct NodeInput<N: NodeSet> {
    pub connection: Option<Entity>,
    pub default: N::NodeIO,
    pub label: String,
    _phantom: PhantomData<N>,
}

impl<N: NodeSet> NodeInput<N> {
    pub fn from_label(label: &str) -> Self {
        Self {
            label: label.to_string(),
            ..default()
        }
    }

    pub fn get_input(
        &self,
        q_nodes: &Query<(Entity, &Node<N>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<N>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> N::NodeIO {
        if let Some(connection) = self.connection {
            if let Ok((parent, output)) = q_outputs.get(connection) {
                if let Ok((entity, node)) = q_nodes.get(parent.get()) {
                    return node.resolve(entity, Some(&output.label), q_nodes, q_inputs, q_outputs);
                }
            }
        }

        self.default.clone()
    }
}

#[derive(Clone, Component)]
pub struct NodeOutput {
    pub label: String,
}

impl NodeOutput {
    pub fn from_label(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

pub enum NodeEvent<N: NodeSet> {
    Destroyed,
    Resolved(N::NodeIO),
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NodeMaterial {
    #[uniform(0)]
    pub active: u32,
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub color_border: Color,
    #[uniform(0)]
    pub color_title: Color,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub border_thickness: f32,
    #[uniform(0)]
    pub height_title: f32,
}

impl Material2d for NodeMaterial {
    fn fragment_shader() -> ShaderRef {
        NODE_SHADER_HANDLE.typed().into()
    }
}

#[derive(Resource)]
pub(crate) struct NodeResources {
    pub material_handle_input: Handle<ColorMaterial>,
    pub material_handle_input_inactive: Handle<ColorMaterial>,
    pub material_handle_output: Handle<ColorMaterial>,
    pub mesh_handle_io: Handle<Mesh>,
    pub text_style_body: TextStyle,
    pub text_style_title: TextStyle,
}

#[derive(Component)]
pub struct OutputNode;

fn setup(
    mut commands: Commands,
    assets: Res<DefaultAssets>,
    config: Res<NodeConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(ActiveNode::default());

    let text_style_body = TextStyle {
        font: assets.font.clone(),
        font_size: config.font_size_body,
        color: Color::WHITE,
    };
    let text_style_title = TextStyle {
        font: assets.font_bold.clone(),
        font_size: config.font_size_title,
        color: Color::WHITE,
    };

    commands.insert_resource(NodeResources {
        material_handle_input: materials.add(Color::rgb(0.0, 0.992, 0.933).into()),
        material_handle_input_inactive: materials.add(Color::rgb(0.541, 0.624, 0.62).into()),
        material_handle_output: materials.add(Color::rgb(0.992, 0.475, 0.0).into()),
        mesh_handle_io: meshes.add(shape::Circle::new(config.handle_size_io).into()),
        text_style_body,
        text_style_title,
    });
}

fn activate_node(
    cursor: Res<CursorPosition>,
    mut active_node: ResMut<ActiveNode>,
    mut materials: ResMut<Assets<NodeMaterial>>,
    mut ev_click: EventReader<Clicked>,
    mut q_node: Query<(&Handle<NodeMaterial>, &mut Transform, &GlobalTransform)>,
) {
    for ev in ev_click.iter() {
        let to_deactivate = active_node.entity;

        if let Clicked(Some(entity)) = ev {
            if let Ok((handle, mut transform, global_transform)) = q_node.get_mut(*entity) {
                active_node.offset = global_transform.translation().truncate() - cursor.position();

                if let Some(active_entity) = active_node.entity {
                    if active_entity == *entity {
                        return;
                    }
                }

                transform.translation.z = active_node.index;
                active_node.entity = Some(*entity);
                active_node.index += 10.0;
                active_node.index_reset = true;

                let mut material = materials.get_mut(handle).unwrap();

                material.active = 1;
            } else {
                active_node.entity = None;
            }
        } else {
            active_node.entity = None;
        }

        if let Some(entity) = to_deactivate {
            let (handle, _, _) = q_node.get(entity).unwrap();
            let mut material = materials.get_mut(handle).unwrap();

            material.active = 0;
        }
    }
}

fn delete_node<N: NodeSet>(
    mut commands: Commands,
    mut active_node: ResMut<ActiveNode>,
    keys: Res<Input<KeyCode>>,
    node_res: Res<NodeResources>,
    mut ev_node: EventWriter<NodeEvent<N>>,
    q_outputs: Query<(Entity, &Parent), With<NodeOutput>>,
    mut q_inputs: Query<(Entity, &mut NodeInput<N>)>,
    mut q_material: Query<(&Parent, &mut Handle<ColorMaterial>)>,
) {
    if keys.just_pressed(KeyCode::Delete) {
        if let Some(entity) = active_node.entity {
            for (output_entity, _) in q_outputs
                .iter()
                .filter(|(_, parent)| parent.get() == entity)
            {
                for (input_entity, mut input) in q_inputs.iter_mut() {
                    if let Some(output) = input.connection {
                        if output == output_entity {
                            input.connection = None;

                            for (parent, mut material) in q_material.iter_mut() {
                                if parent.get() == input_entity {
                                    *material = node_res.material_handle_input_inactive.clone();
                                }
                            }
                        }
                    }
                }
            }
            commands.entity(entity).despawn_recursive();
            active_node.count -= 1;
            active_node.entity = None;
            ev_node.send(NodeEvent::Destroyed);
        }
    }
}

fn drag_node<N: NodeSet>(
    active_node: Res<ActiveNode>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Node<N>>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || !mouse_button_input.pressed(MouseButton::Left)
    {
        return;
    }

    if let Some(entity) = active_node.entity {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation.x = cursor.x + active_node.offset.x;
            transform.translation.y = cursor.y + active_node.offset.y;
        }
    }
}

fn resolve_output_nodes<N: NodeSet>(
    mut ev_resolution: EventWriter<NodeEvent<N>>,
    mut ev_connection: EventReader<ConnectionEvent>,
    q_output: Query<(Entity, &Node<N>), With<OutputNode>>,
    q_nodes: Query<(Entity, &Node<N>), Without<OutputNode>>,
    q_inputs: Query<(&Parent, &NodeInput<N>)>,
    q_outputs: Query<(&Parent, &NodeOutput)>,
) {
    if ev_connection.iter().next().is_some() {
        for (entity, node) in q_output.iter() {
            ev_resolution.send(NodeEvent::Resolved(
                node.resolve(entity, None, &q_nodes, &q_inputs, &q_outputs),
            ));
        }
    }
}
