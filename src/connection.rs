#![allow(clippy::type_complexity)]

use bevy::{prelude::*, transform::TransformSystem};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use std::marker::PhantomData;

use crate::{
    cursor::CursorPosition,
    interactions::Clicked,
    node::{NodeInput, NodeOutput, NodeResources, NodeSet},
};

#[derive(Default)]
pub struct ConnectionPlugin<N: NodeSet>(PhantomData<N>);

impl<N: NodeSet> Plugin for ConnectionPlugin<N> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectionConfig::default())
            .add_event::<ConnectionEvent>()
            .add_plugin(ShapePlugin)
            .add_system(break_connection::<N>)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                draw_connections::<N>.after(TransformSystem::TransformPropagate),
            )
            .add_system(draw_partial_connections::<N>)
            .add_system(complete_partial_connection::<N>)
            .add_system(convert_partial_connection::<N>)
            .add_system(create_partial_connection::<N>.before(break_connection::<N>));
    }
}

#[derive(Resource)]
pub struct ConnectionConfig {
    pub connection_size: f32,
    pub connection_threshold: f32,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            connection_size: 2.0,
            connection_threshold: 10.0,
        }
    }
}

pub enum ConnectionEvent {
    Propagate,
    Created,
    Destroyed,
}

#[derive(Component)]
struct PartialConnection {
    input: Option<Entity>,
    output: Option<Entity>,
}

#[derive(Component)]
struct Connection;

fn break_connection<N: NodeSet>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    node_res: Res<NodeResources>,
    mut ev_click: EventReader<Clicked>,
    mut ev_connection: EventWriter<ConnectionEvent>,
    q_connection: Query<(Entity, &Parent), With<Connection>>,
    mut q_inputs: Query<&mut NodeInput<N>>,
    mut q_material: Query<(&Parent, &mut Handle<ColorMaterial>)>,
) {
    for ev in ev_click.iter() {
        if let Clicked(Some(entity)) = ev {
            if let Ok(mut node_input) = q_inputs.get_mut(*entity) {
                if node_input.connection.is_some() {
                    commands.spawn((
                        PartialConnection {
                            input: None,
                            output: node_input.connection,
                        },
                        ShapeBundle {
                            mode: DrawMode::Stroke(StrokeMode::new(
                                Color::WHITE,
                                config.connection_size,
                            )),
                            ..default()
                        },
                    ));

                    for (connection_entity, parent) in q_connection.iter() {
                        if parent.get() == *entity {
                            commands.entity(connection_entity).despawn_recursive();
                            break;
                        }
                    }

                    node_input.connection = None;
                    ev_connection.send(ConnectionEvent::Destroyed);

                    for (parent, mut material) in q_material.iter_mut() {
                        if parent.get() == *entity {
                            *material = node_res.material_handle_input_inactive.clone();
                        }
                    }
                }
            }
        }
    }
}

fn complete_partial_connection<T: NodeSet>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut q_connections: Query<(Entity, &mut PartialConnection)>,
    q_input: Query<(Entity, &GlobalTransform), With<NodeInput<T>>>,
    q_output: Query<(Entity, &GlobalTransform), With<NodeOutput>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for (entity, mut connection) in q_connections.iter_mut() {
            if connection.input.is_some() {
                for (entity, transform) in q_output.iter() {
                    let translation = transform.translation();

                    if (translation.x - cursor.x).abs() < config.connection_threshold
                        && (translation.y - cursor.y).abs() < config.connection_threshold
                    {
                        connection.output = Some(entity);
                        break;
                    }
                }
            } else if connection.output.is_some() {
                for (entity, transform) in q_input.iter() {
                    let translation = transform.translation();

                    if (translation.x - cursor.x).abs() < config.connection_threshold
                        && (translation.y - cursor.y).abs() < config.connection_threshold
                    {
                        connection.input = Some(entity);
                        break;
                    }
                }
            }

            if connection.input.is_none() || connection.output.is_none() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn convert_partial_connection<N: NodeSet>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    node_res: Res<NodeResources>,
    mut ev_connection: EventWriter<ConnectionEvent>,
    q_connections: Query<(Entity, &PartialConnection)>,
    q_outputs: Query<&Parent, With<NodeOutput>>,
    mut q_inputs: Query<(Entity, &Parent, &GlobalTransform, &mut NodeInput<N>)>,
    mut q_material: Query<(&Parent, &mut Handle<ColorMaterial>)>,
) {
    for (entity, connection) in q_connections.iter() {
        if let Some(input) = connection.input {
            if let Some(output) = connection.output {
                if let Ok((input_entity, input_parent, &transform, mut input)) =
                    q_inputs.get_mut(input)
                {
                    if let Ok(output_parent) = q_outputs.get(output) {
                        if input_parent.get() != output_parent.get() {
                            input.connection = Some(output);

                            let child = commands
                                .spawn((
                                    ShapeBundle {
                                        mode: DrawMode::Stroke(StrokeMode::new(
                                            Color::WHITE,
                                            config.connection_size,
                                        )),
                                        transform: Transform::from_xyz(
                                            0.0,
                                            0.0,
                                            -transform.translation().z,
                                        ),
                                        ..default()
                                    },
                                    Connection,
                                ))
                                .id();

                            commands.entity(input_entity).push_children(&[child]);

                            ev_connection.send(ConnectionEvent::Created);

                            for (parent, mut material) in q_material.iter_mut() {
                                if parent.get() == input_entity {
                                    *material = node_res.material_handle_input.clone();
                                }
                            }
                        }
                    }

                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn create_partial_connection<N: NodeSet>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    mut ev_click: EventReader<Clicked>,
    q_connections: Query<&PartialConnection>,
    q_input: Query<&NodeInput<N>>,
    q_output: Query<&NodeOutput>,
) {
    if !q_connections.is_empty() {
        return;
    }

    for ev in ev_click.iter() {
        if let Clicked(Some(entity)) = ev {
            if let Ok(node_input) = q_input.get(*entity) {
                if node_input.connection.is_none() {
                    commands.spawn((
                        PartialConnection {
                            input: Some(*entity),
                            output: None,
                        },
                        ShapeBundle {
                            mode: DrawMode::Stroke(StrokeMode::new(
                                Color::WHITE,
                                config.connection_size,
                            )),
                            ..default()
                        },
                    ));
                }
            } else if q_output.get(*entity).is_ok() {
                commands.spawn((
                    PartialConnection {
                        input: None,
                        output: Some(*entity),
                    },
                    ShapeBundle {
                        mode: DrawMode::Stroke(StrokeMode::new(
                            Color::WHITE,
                            config.connection_size,
                        )),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn draw_connections<N: NodeSet>(
    mut commands: Commands,
    mut q_connection: Query<(Entity, &Parent, &mut Path), With<Connection>>,
    mut q_input: Query<(&NodeInput<N>, &GlobalTransform)>,
    q_output: Query<&GlobalTransform, With<NodeOutput>>,
) {
    for (entity, parent, mut path) in q_connection.iter_mut() {
        let mut cleanup = false;

        if let Ok((input, input_transform)) = q_input.get_mut(parent.get()) {
            if let Some(connection) = input.connection {
                if let Ok(output_transform) = q_output.get(connection) {
                    let mut path_builder = PathBuilder::new();
                    let input_position = input_transform.translation().truncate();
                    let output_position = output_transform.translation().truncate();
                    let end = output_position - input_position;
                    let half_x = end.x / 2.0;
                    let ctrl_1 = Vec2::new(half_x, 0.0);
                    let ctrl_2 = Vec2::new(half_x, end.y);

                    path_builder.move_to(Vec2::ZERO);
                    path_builder.cubic_bezier_to(ctrl_1, ctrl_2, end);

                    let line = path_builder.build();

                    *path = ShapePath::build_as(&line);
                } else {
                    cleanup = true;
                }
            } else {
                cleanup = true;
            }
        } else {
            cleanup = true;
        }

        if cleanup {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn draw_partial_connections<N: NodeSet>(
    mut commands: Commands,
    cursor: Res<CursorPosition>,
    mut q_connections: Query<(Entity, &PartialConnection, &mut Path)>,
    q_start: Query<&GlobalTransform, Or<(With<NodeInput<N>>, With<NodeOutput>)>>,
) {
    for (entity, connection, mut path) in q_connections.iter_mut() {
        let connection_entity = if connection.input.is_some() {
            connection.input
        } else {
            connection.output
        };

        if let Some(connection_entity) = connection_entity {
            if let Ok(transform) = q_start.get(connection_entity) {
                let mut path_builder = PathBuilder::new();
                let start = transform.translation().truncate();
                let end = cursor.position();
                let half_x = (end.x - start.x) / 2.0;
                let ctrl_1 = Vec2::new(start.x + half_x, start.y);
                let ctrl_2 = Vec2::new(start.x + half_x, end.y);

                path_builder.move_to(Vec2::new(
                    transform.translation().x,
                    transform.translation().y,
                ));
                path_builder.cubic_bezier_to(ctrl_1, ctrl_2, end);

                let line = path_builder.build();

                *path = ShapePath::build_as(&line);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
