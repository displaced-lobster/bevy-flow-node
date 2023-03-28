use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
    text::Text2dBounds,
};
use std::marker::PhantomData;

use crate::{
    interactions::Clickable,
    node::{
        ActiveNode,
        FlowNode,
        FlowNodeConfig,
        FlowNodeInput,
        FlowNodeMaterial,
        FlowNodeOutput,
        FlowNodeResources,
        FlowNodeSet,
        OutputNode,
    },
};

#[derive(Default)]
pub struct FlowNodeTemplatePlugin<N: FlowNodeSet> {
    _phantom: PhantomData<N>,
}

impl<N: FlowNodeSet> Plugin for FlowNodeTemplatePlugin<N> {
    fn build(&self, app: &mut App) {
        app.add_system(build_node::<N>);
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct FlowNodeSlot {
    pub height: f32,
    pub width: f32,
}

impl FlowNodeSlot {
    pub fn new(height: f32) -> Self {
        Self {
            height,
            ..default()
        }
    }
}

#[derive(Component)]
pub struct FlowNodeTemplate<N: FlowNodeSet> {
    pub inputs: Option<Vec<FlowNodeInput<N>>>,
    pub node: N,
    pub outputs: Option<Vec<FlowNodeOutput>>,
    pub position: Vec2,
    pub slot: Option<FlowNodeSlot>,
    pub title: String,
    pub width: f32,
}

impl<N: FlowNodeSet> Default for FlowNodeTemplate<N> {
    fn default() -> Self {
        Self {
            inputs: None,
            node: N::default(),
            position: Vec2::ZERO,
            outputs: None,
            slot: None,
            title: "Node".to_string(),
            width: 200.0,
        }
    }
}

fn build_node<N: FlowNodeSet>(
    mut commands: Commands,
    config: Res<FlowNodeConfig>,
    resources: Res<FlowNodeResources>,
    mut active_node: ResMut<ActiveNode>,
    mut materials: ResMut<Assets<FlowNodeMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &FlowNodeTemplate<N>)>,
) {
    for (entity, template) in query.iter() {
        let n_inputs = if let Some(inputs) = &template.inputs {
            inputs.len()
        } else {
            0
        };
        let n_outputs = if let Some(outputs) = &template.outputs {
            outputs.len()
        } else {
            0
        };
        let slot_height = if let Some(slot) = template.slot {
            slot.height + 2.0 * config.padding
        } else {
            0.0
        };

        let height_io = config.font_size_body + config.padding * 2.0;
        let height_body = height_io * (n_inputs + n_outputs) as f32 + slot_height;
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
                    material: materials.add(FlowNodeMaterial {
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
                    text_anchor: Anchor::TopRight,
                    text_2d_bounds: Text2dBounds { size: bounds_title },
                    transform: Transform::from_xyz(offset_x, offset_y, 2.0),
                    ..default()
                });

                offset_y -= height_title;

                if let Some(outputs) = &template.outputs {
                    for output in outputs {
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
                            (*output).clone(),
                            Clickable::Radius(config.handle_size_io),
                        ));

                        parent.spawn(Text2dBundle {
                            text: Text::from_section(
                                output.label.clone(),
                                resources.text_style_body.clone(),
                            )
                            .with_alignment(TextAlignment::Right),
                            text_anchor: Anchor::TopLeft,
                            text_2d_bounds: Text2dBounds { size: bounds_io },
                            transform: Transform::from_xyz(
                                node_size.x / 2.0 - config.handle_size_io - config.padding,
                                offset_y - config.font_size_body + config.handle_size_io * 2.0,
                                1.0,
                            ),
                            ..default()
                        });

                        offset_y -= height_io;
                    }
                } else {
                    output = true;
                }

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
                            text_anchor: Anchor::TopRight,
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
            .insert(FlowNode(template.node.clone()))
            .remove::<FlowNodeTemplate<N>>();

        if output {
            commands.entity(entity).insert(OutputNode);
        }

        active_node.count += 1;
        active_node.index += 10.0;
    }
}
