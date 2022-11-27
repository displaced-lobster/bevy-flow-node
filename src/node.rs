use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    text::Text2dBounds,
};
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    assets::DefaultAssets,
    connection::ConnectionEvent,
    cursor::CursorPosition,
    interactions::{Clickable, Clicked},
};

const NODE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7843551199445678407);

pub trait NodeSet: 'static + Clone + Default + Sized + Send + Sync {
    type NodeIO: Clone + Default + Send + Sync;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO;
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
            .add_system(build_node::<N>)
            .add_system(delete_node::<N>)
            .add_system(drag_node::<N>.after(activate_node))
            .add_system(resolve_output_nodes::<N>);
    }
}

#[derive(Default, Resource)]
struct ActiveNode {
    count: u32,
    index: f32,
    index_reset: bool,
    entity: Option<Entity>,
    offset: Vec2,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Node<N: NodeSet>(N);

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
        q_nodes: &Query<(Entity, &Node<N>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<N>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> N::NodeIO {
        let inputs = self.get_inputs(entity, q_nodes, q_inputs, q_outputs);

        self.0.resolve(&inputs)
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
            if let Ok((parent, _output)) = q_outputs.get(connection) {
                if let Ok((entity, node)) = q_nodes.get(parent.get()) {
                    return node.resolve(entity, q_nodes, q_inputs, q_outputs);
                }
            }
        }

        self.default.clone()
    }
}

#[derive(Component)]
pub struct NodeOutput;

pub enum NodeEvent<N: NodeSet> {
    Destroyed,
    Resolved(N::NodeIO),
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NodeMaterial {
    #[uniform(0)]
    active: u32,
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    color_border: Color,
    #[uniform(0)]
    color_title: Color,
    #[uniform(0)]
    size: Vec2,
    #[uniform(0)]
    border_thickness: f32,
    #[uniform(0)]
    height_title: f32,
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

#[derive(Component, Clone, Copy, Default)]
pub struct NodeSlot {
    pub height: f32,
    pub width: f32,
}

impl NodeSlot {
    pub fn new(height: f32) -> Self {
        Self {
            height,
            ..default()
        }
    }
}

#[derive(Component)]
pub struct NodeTemplate<N: NodeSet> {
    pub inputs: Option<Vec<NodeInput<N>>>,
    pub node: N,
    pub output_label: Option<String>,
    pub position: Vec2,
    pub slot: Option<NodeSlot>,
    pub title: String,
    pub width: f32,
}

impl<N: NodeSet> Default for NodeTemplate<N> {
    fn default() -> Self {
        Self {
            inputs: None,
            node: N::default(),
            position: Vec2::ZERO,
            output_label: None,
            slot: None,
            title: "Node".to_string(),
            width: 200.0,
        }
    }
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
            if let Some(active_entity) = active_node.entity {
                if active_entity == *entity {
                    return;
                }
            }

            if let Ok((handle, mut transform, global_transform)) = q_node.get_mut(*entity) {
                transform.translation.z = active_node.index;
                active_node.entity = Some(*entity);
                active_node.index += 10.0;
                active_node.index_reset = true;
                active_node.offset = global_transform.translation().truncate() - cursor.position();

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

fn build_node<N: NodeSet>(
    mut commands: Commands,
    config: Res<NodeConfig>,
    resources: Res<NodeResources>,
    mut active_node: ResMut<ActiveNode>,
    mut materials: ResMut<Assets<NodeMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &NodeTemplate<N>)>,
) {
    for (entity, template) in query.iter() {
        let n_io = if let Some(inputs) = &template.inputs {
            inputs.len()
        } else {
            0
        };
        let slot_height = if let Some(slot) = template.slot {
            slot.height + 2.0 * config.padding
        } else {
            0.0
        };

        let height_io = config.font_size_body + config.padding * 2.0;
        let height_body = height_io * (n_io + 1) as f32 + slot_height;
        let height_title = config.font_size_title + config.padding * 2.0;
        let height = height_body + height_title + 2.0;
        let node_size = Vec2::new(template.width, height);
        let width_interior = template.width - 2.0 * config.padding;
        let bounds_title = Vec2::new(width_interior, config.font_size_title);
        let bounds_io = Vec2::new(width_interior, config.font_size_body);
        let offset_x = -node_size.x / 2.0 + config.padding;
        let mut offset_y = node_size.y / 2.0 - config.padding;
        let mut output = false;

        commands
            .entity(entity)
            .insert((
                MaterialMesh2dBundle {
                    material: materials.add(NodeMaterial {
                        color: config.color_node,
                        color_border: config.color_border,
                        color_title: config.color_title,
                        size: node_size,
                        border_thickness: config.border_thickness,
                        height_title,
                        ..default()
                    }),
                    mesh: Mesh2dHandle(
                        meshes.add(
                            shape::Quad {
                                size: node_size,
                                ..default()
                            }
                            .into(),
                        ),
                    ),
                    transform: Transform::from_xyz(
                        template.position.x,
                        template.position.y,
                        active_node.index,
                    ),
                    ..default()
                },
                Clickable::Area(node_size),
            ))
            .with_children(|parent| {
                parent.spawn(SpatialBundle {
                    transform: Transform::from_xyz(0.0, (node_size.y - height_title) / 2.0, 1.0),
                    ..default()
                });

                parent.spawn(Text2dBundle {
                    text: Text::from_section(&template.title, resources.text_style_title.clone()),
                    text_2d_bounds: Text2dBounds { size: bounds_title },
                    transform: Transform::from_xyz(offset_x, offset_y, 2.0),
                    ..default()
                });

                offset_y -= height_title;

                if let Some(label) = &template.output_label {
                    parent.spawn((
                        MaterialMesh2dBundle {
                            material: resources.material_handle_output.clone(),
                            mesh: Mesh2dHandle(resources.mesh_handle_io.clone()),
                            transform: Transform::from_xyz(
                                node_size.x / 2.0,
                                offset_y - config.handle_size_io - config.padding,
                                2.0,
                            ),
                            ..default()
                        },
                        NodeOutput,
                        Clickable::Radius(config.handle_size_io),
                    ));

                    parent.spawn(Text2dBundle {
                        text: Text::from_section(label.clone(), resources.text_style_body.clone())
                            .with_alignment(TextAlignment::TOP_RIGHT),
                        text_2d_bounds: Text2dBounds { size: bounds_io },
                        transform: Transform::from_xyz(
                            node_size.x / 2.0 - config.handle_size_io - config.padding,
                            offset_y - config.font_size_body + config.handle_size_io * 2.0,
                            1.0,
                        ),
                        ..default()
                    });
                } else {
                    output = true;
                }

                offset_y -= height_io;

                if let Some(inputs) = &template.inputs {
                    for input in inputs.iter() {
                        parent
                            .spawn((
                                SpatialBundle {
                                    transform: Transform::from_xyz(
                                        offset_x - config.handle_size_io,
                                        offset_y - config.handle_size_io - config.padding,
                                        1.0,
                                    ),
                                    ..default()
                                },
                                (*input).clone(),
                                Clickable::Radius(config.handle_size_io),
                            ))
                            .with_children(|parent| {
                                parent.spawn(MaterialMesh2dBundle {
                                    material: resources.material_handle_input_inactive.clone(),
                                    mesh: Mesh2dHandle(resources.mesh_handle_io.clone()),
                                    ..default()
                                });
                            });

                        parent.spawn(Text2dBundle {
                            text: Text::from_section(
                                input.label.clone(),
                                resources.text_style_body.clone(),
                            ),
                            text_2d_bounds: Text2dBounds { size: bounds_io },
                            transform: Transform::from_xyz(
                                offset_x + config.padding,
                                offset_y - config.font_size_body + config.handle_size_io * 2.0,
                                1.0,
                            ),
                            ..default()
                        });

                        offset_y -= height_io;
                    }
                }

                if let Some(slot) = template.slot {
                    let mut slot = slot;

                    slot.width = width_interior;
                    parent.spawn((
                        SpatialBundle {
                            transform: Transform::from_xyz(0.0, offset_y - slot.height / 2.0, 1.0),
                            ..default()
                        },
                        slot,
                    ));
                }
            })
            .insert(Node(template.node.clone()))
            .remove::<NodeTemplate<N>>();

        if output {
            commands.entity(entity).insert(OutputNode);
        }

        active_node.count += 1;
        active_node.index += 10.0;
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
                node.resolve(entity, &q_nodes, &q_inputs, &q_outputs),
            ));
        }
    }
}
