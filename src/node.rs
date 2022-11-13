use bevy::{prelude::*, sprite::MaterialMesh2dBundle, text::Text2dBounds};
use std::{collections::HashMap, marker::PhantomData};

use crate::{connection::ConnectionEvent, cursor::CursorPosition};

pub trait Nodes: 'static + Clone + Copy + Default + Sized + Send + Sync {
    type NodeIO: Clone + Copy + Default + Send + Sync;

    fn resolve(&self, inputs: &HashMap<String, Self::NodeIO>) -> Self::NodeIO;
}

pub struct NodePlugin<T: Nodes>(PhantomData<T>);

impl<T: Nodes> Default for NodePlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Nodes> Plugin for NodePlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(NodeConfig::default())
            .add_startup_system(setup)
            .add_system(activate_node)
            .add_system(build_node::<T>)
            .add_system(drag_node::<T>)
            .add_system(resolve_output_nodes::<T>);
    }
}

#[derive(Default)]
struct ActiveNode {
    entity: Option<Entity>,
    offset: Vec2,
}

#[derive(Component, Default)]
pub struct Node<T: Nodes> {
    node: T,
}

impl<T: Nodes> Node<T> {
    pub fn get_inputs(
        &self,
        entity: Entity,
        q_nodes: &Query<(Entity, &Node<T>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<T>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> HashMap<String, T::NodeIO> {
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
        q_nodes: &Query<(Entity, &Node<T>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<T>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> T::NodeIO {
        let inputs = self.get_inputs(entity, q_nodes, q_inputs, q_outputs);

        self.node.resolve(&inputs)
    }
}

pub struct NodeConfig {
    pub handle_size_io: f32,
    pub handle_size_title: f32,
    pub padding: f32,
    pub font_size_body: f32,
    pub font_size_title: f32,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            handle_size_io: 6.0,
            handle_size_title: 10.0,
            padding: 5.0,
            font_size_body: 16.0,
            font_size_title: 20.0,
        }
    }
}

#[derive(Component)]
struct NodeHandle;

#[derive(Component, Default)]
pub struct NodeInput<T: Nodes> {
    pub connection: Option<Entity>,
    pub default: T::NodeIO,
    pub label: String,
    _phantom: PhantomData<T>,
}

impl<T: Nodes> NodeInput<T> {
    pub fn get_input(
        &self,
        q_nodes: &Query<(Entity, &Node<T>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<T>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> T::NodeIO {
        if let Some(connection) = self.connection {
            if let Ok((parent, _output)) = q_outputs.get(connection) {
                if let Ok((entity, node)) = q_nodes.get(parent.get()) {
                    return node.resolve(entity, q_nodes, q_inputs, q_outputs);
                }
            }
        }

        self.default
    }
}

#[derive(Component)]
pub struct NodeOutput;

struct NodeResources {
    material_handle_input: Handle<ColorMaterial>,
    material_handle_output: Handle<ColorMaterial>,
    material_handle_title: Handle<ColorMaterial>,
    mesh_handle: Handle<Mesh>,
    mesh_handle_io: Handle<Mesh>,
    text_style_body: TextStyle,
    text_style_title: TextStyle,
}

pub struct NodeIOTemplate {
    pub label: String,
}

impl Default for NodeIOTemplate {
    fn default() -> Self {
        Self {
            label: "I/O".to_string(),
        }
    }
}

#[derive(Component)]
pub struct NodeTemplate<T: Nodes> {
    pub activate: bool,
    pub inputs: Option<Vec<NodeIOTemplate>>,
    pub node: T,
    pub output_label: Option<String>,
    pub position: Vec2,
    pub title: String,
    pub width: f32,
}

impl<T: Nodes> Default for NodeTemplate<T> {
    fn default() -> Self {
        Self {
            activate: false,
            inputs: None,
            node: T::default(),
            position: Vec2::ZERO,
            output_label: None,
            title: "Node".to_string(),
            width: 200.0,
        }
    }
}

#[derive(Component)]
pub struct OutputNode;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<NodeConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(ActiveNode::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style_body = TextStyle {
        font: font.clone(),
        font_size: config.font_size_body,
        color: Color::WHITE,
    };
    let text_style_title = TextStyle {
        font,
        font_size: config.font_size_title,
        color: Color::WHITE,
    };

    commands.insert_resource(NodeResources {
        material_handle_input: materials.add(Color::YELLOW.into()),
        material_handle_output: materials.add(Color::RED.into()),
        material_handle_title: materials.add(Color::PURPLE.into()),
        mesh_handle: meshes.add(shape::Circle::new(config.handle_size_title).into()),
        mesh_handle_io: meshes.add(shape::Circle::new(config.handle_size_io).into()),
        text_style_body,
        text_style_title,
    });
}

fn activate_node(
    mut active_node: ResMut<ActiveNode>,
    config: Res<NodeConfig>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<(&Parent, &GlobalTransform), With<NodeHandle>>,
    q_parent: Query<&GlobalTransform>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (parent, transform) in query.iter() {
            let translation = transform.translation();

            if (translation.x - cursor.x).abs() < config.handle_size_title
                && (translation.y - cursor.y).abs() < config.handle_size_title
            {
                let transform = q_parent.get(parent.get()).unwrap();
                let translation = transform.translation();

                active_node.entity = Some(parent.get());
                active_node.offset = Vec2::new(translation.x - cursor.x, translation.y - cursor.y);
                break;
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        active_node.entity = None;
        active_node.offset = Vec2::ZERO;
    }
}

fn build_node<T: Nodes>(
    mut commands: Commands,
    config: Res<NodeConfig>,
    resources: Res<NodeResources>,
    mut active_node: ResMut<ActiveNode>,
    query: Query<(Entity, &NodeTemplate<T>)>,
) {
    for (entity, template) in query.iter() {
        let n_io = 2;
        let height_body = (config.font_size_body + config.padding) * n_io as f32;
        let height_title = config.font_size_title + config.padding * 2.0;
        let height = height_body + height_title;
        let node_size = Vec2::new(template.width, height);
        let width_interior = template.width - 2.0 * config.padding;
        let bounds_title = Vec2::new(width_interior, config.font_size_title);
        let bounds_io = Vec2::new(width_interior / 2.0, config.font_size_body);
        let offset_x = -node_size.x / 2.0 + config.padding;
        let mut offset_y = node_size.y / 2.0 - config.padding;
        let mut output = false;

        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(node_size.x, node_size.y)),
                    ..default()
                },
                transform: Transform::from_xyz(template.position.x, template.position.y, 0.0),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(MaterialMesh2dBundle {
                        material: resources.material_handle_title.clone(),
                        mesh: bevy::sprite::Mesh2dHandle(resources.mesh_handle.clone()),
                        transform: Transform::from_xyz(
                            offset_x + config.handle_size_title,
                            offset_y - config.handle_size_title,
                            1.0,
                        ),
                        ..default()
                    })
                    .insert(NodeHandle);

                parent.spawn_bundle(Text2dBundle {
                    text: Text::from_section(&template.title, resources.text_style_title.clone()),
                    text_2d_bounds: Text2dBounds { size: bounds_title },
                    transform: Transform::from_xyz(
                        offset_x + config.handle_size_title * 2.0 + config.padding,
                        offset_y,
                        1.0,
                    ),
                    ..default()
                });

                offset_y -= height_title / 2.0 + config.padding;

                let offset_y_body = offset_y;

                if let Some(inputs) = &template.inputs {
                    for io_template in inputs.iter() {
                        parent
                            .spawn_bundle(SpatialBundle {
                                transform: Transform::from_xyz(
                                    offset_x + config.handle_size_io,
                                    offset_y - config.handle_size_io - config.padding,
                                    1.0,
                                ),
                                ..default()
                            })
                            .insert(NodeInput::<T> {
                                label: io_template.label.clone(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn_bundle(MaterialMesh2dBundle {
                                    material: resources.material_handle_input.clone(),
                                    mesh: bevy::sprite::Mesh2dHandle(
                                        resources.mesh_handle_io.clone(),
                                    ),
                                    ..default()
                                });
                            });

                        parent.spawn_bundle(Text2dBundle {
                            text: Text::from_section(
                                io_template.label.clone(),
                                resources.text_style_body.clone(),
                            ),
                            text_2d_bounds: Text2dBounds { size: bounds_io },
                            transform: Transform::from_xyz(
                                offset_x + 2.0 * config.handle_size_io + config.padding,
                                offset_y - config.font_size_body + config.handle_size_io * 2.0,
                                1.0,
                            ),
                            ..default()
                        });

                        offset_y -= height_body / 2.0;
                    }
                }

                offset_y = offset_y_body;

                let offset_x = config.padding;

                if let Some(label) = &template.output_label {
                    parent
                        .spawn_bundle(MaterialMesh2dBundle {
                            material: resources.material_handle_output.clone(),
                            mesh: bevy::sprite::Mesh2dHandle(resources.mesh_handle_io.clone()),
                            transform: Transform::from_xyz(
                                offset_x + node_size.x / 2.0
                                    - 2.0 * config.handle_size_io
                                    - config.padding,
                                offset_y - config.handle_size_io - config.padding,
                                2.0,
                            ),
                            ..default()
                        })
                        .insert(NodeOutput);

                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(label.clone(), resources.text_style_body.clone()),
                        text_2d_bounds: Text2dBounds { size: bounds_io },
                        transform: Transform::from_xyz(
                            offset_x + config.padding,
                            offset_y - config.font_size_body + config.handle_size_io * 2.0,
                            1.0,
                        ),
                        ..default()
                    });

                    offset_y -= height_body / 2.0;
                } else {
                    output = true;
                }
            })
            .insert(Node {
                node: template.node,
            })
            .remove::<NodeTemplate<T>>();

        if output {
            commands.entity(entity).insert(OutputNode);
        }

        if template.activate {
            active_node.entity = Some(entity);
            active_node.offset = Vec2::ZERO;
        }
    }
}

fn drag_node<T: Nodes>(
    active_node: Res<ActiveNode>,
    cursor: Res<CursorPosition>,
    mut query: Query<&mut Transform, With<Node<T>>>,
) {
    if let Some(entity) = active_node.entity {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation.x = cursor.x + active_node.offset.x;
            transform.translation.y = cursor.y + active_node.offset.y;
        }
    }
}

fn resolve_output_nodes<T: Nodes>(
    mut ev_connection: EventReader<ConnectionEvent>,
    q_output: Query<(Entity, &Node<T>), With<OutputNode>>,
    q_nodes: Query<(Entity, &Node<T>), Without<OutputNode>>,
    q_inputs: Query<(&Parent, &NodeInput<T>)>,
    q_outputs: Query<(&Parent, &NodeOutput)>,
) {
    if ev_connection.iter().next().is_some() {
        for (entity, node) in q_output.iter() {
            node.resolve(entity, &q_nodes, &q_inputs, &q_outputs);
        }
    }
}
