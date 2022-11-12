#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use std::marker::PhantomData;

use crate::{
    cursor::CursorPosition,
    node::{NodeInput, NodeOutput, Nodes},
};

#[derive(Default)]
pub struct ConnectionPlugin<T: Nodes>(PhantomData<T>);

impl<T: Nodes> Plugin for ConnectionPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectionConfig::default())
            .add_event::<ConnectionEvent>()
            .add_plugin(ShapePlugin)
            .add_system(break_connection::<T>)
            .add_system(draw_connections::<T>)
            .add_system(draw_partial_connections::<T>)
            .add_system(complete_partial_connection::<T>)
            .add_system(convert_partial_connection::<T>)
            .add_system(create_partial_connection::<T>);
    }
}

pub struct ConnectionConfig {
    pub connection_size: f32,
    pub threshold_radius: f32,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            connection_size: 2.0,
            threshold_radius: 6.0,
        }
    }
}

pub enum ConnectionEvent {
    Created,
    Destroyed,
}

#[derive(Component)]
struct PartialConnection {
    input: Option<Entity>,
    output: Option<Entity>,
}

fn break_connection<T: Nodes>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_connection: EventWriter<ConnectionEvent>,
    mut q_inputs: Query<(Entity, &mut NodeInput<T>, &GlobalTransform)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, mut node_input, transform) in q_inputs.iter_mut() {
        if node_input.connection.is_none() {
            continue;
        }

        let translation = transform.translation();

        if (translation.x - cursor.x).abs() < config.threshold_radius
            && (translation.y - cursor.y).abs() < config.threshold_radius
        {
            commands
                .spawn()
                .insert(PartialConnection {
                    input: None,
                    output: node_input.connection,
                })
                .insert_bundle(ShapeBundle {
                    mode: DrawMode::Stroke(StrokeMode::new(Color::WHITE, config.connection_size)),
                    ..default()
                });

            commands
                .entity(entity)
                .insert(DrawMode::Stroke(StrokeMode::new(Color::BLACK, 0.0)));
            node_input.connection = None;
            ev_connection.send(ConnectionEvent::Destroyed);

            return;
        }
    }
}

fn complete_partial_connection<T: Nodes>(
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

                    if (translation.x - cursor.x).abs() < config.threshold_radius
                        && (translation.y - cursor.y).abs() < config.threshold_radius
                    {
                        connection.output = Some(entity);
                        break;
                    }
                }
            } else if connection.output.is_some() {
                for (entity, transform) in q_input.iter() {
                    let translation = transform.translation();

                    if (translation.x - cursor.x).abs() < config.threshold_radius
                        && (translation.y - cursor.y).abs() < config.threshold_radius
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

fn convert_partial_connection<T: Nodes>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    mut ev_connection: EventWriter<ConnectionEvent>,
    q_connections: Query<(Entity, &PartialConnection)>,
    q_outputs: Query<&Parent, With<NodeOutput>>,
    mut q_inputs: Query<(Entity, &Parent, &mut NodeInput<T>, &Transform)>,
) {
    for (entity, connection) in q_connections.iter() {
        if let Some(input) = connection.input {
            if let Some(output) = connection.output {
                if let Ok((input_entity, input_parent, mut input, transform)) =
                    q_inputs.get_mut(input)
                {
                    if let Ok(output_parent) = q_outputs.get(output) {
                        if input_parent.get() != output_parent.get() {
                            input.connection = Some(output);

                            commands.entity(input_entity).insert_bundle(ShapeBundle {
                                mode: DrawMode::Stroke(StrokeMode::new(
                                    Color::WHITE,
                                    config.connection_size,
                                )),
                                transform: *transform,
                                ..default()
                            });

                            ev_connection.send(ConnectionEvent::Created);
                        }
                    }

                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn create_partial_connection<T: Nodes>(
    mut commands: Commands,
    config: Res<ConnectionConfig>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_connections: Query<&PartialConnection>,
    q_input: Query<(Entity, &NodeInput<T>, &GlobalTransform)>,
    q_output: Query<(Entity, &GlobalTransform), With<NodeOutput>>,
) {
    if !q_connections.is_empty() || !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, node_input, transform) in q_input.iter() {
        if node_input.connection.is_some() {
            continue;
        }

        let translation = transform.translation();

        if (translation.x - cursor.x).abs() < config.threshold_radius
            && (translation.y - cursor.y).abs() < config.threshold_radius
        {
            commands
                .spawn()
                .insert(PartialConnection {
                    input: Some(entity),
                    output: None,
                })
                .insert_bundle(ShapeBundle {
                    mode: DrawMode::Stroke(StrokeMode::new(Color::WHITE, config.connection_size)),
                    ..default()
                });

            return;
        }
    }

    for (entity, trasnform) in q_output.iter() {
        let translation = trasnform.translation();

        if (translation.x - cursor.x).abs() < config.threshold_radius
            && (translation.y - cursor.y).abs() < config.threshold_radius
        {
            commands
                .spawn()
                .insert(PartialConnection {
                    input: None,
                    output: Some(entity),
                })
                .insert_bundle(ShapeBundle {
                    mode: DrawMode::Stroke(StrokeMode::new(Color::WHITE, config.connection_size)),
                    ..default()
                });

            return;
        }
    }
}

fn draw_connections<T: Nodes>(
    mut q_input: Query<(&NodeInput<T>, &GlobalTransform, &mut Path)>,
    q_output: Query<&GlobalTransform, With<NodeOutput>>,
) {
    for (input, input_transform, mut path) in q_input.iter_mut() {
        if let Some(entity) = input.connection {
            if let Ok(output_transform) = q_output.get(entity) {
                let mut path_builder = PathBuilder::new();
                let input_position = input_transform.translation().truncate();
                let output_position = output_transform.translation().truncate();

                path_builder.move_to(Vec2::ZERO);
                path_builder.line_to(output_position - input_position);

                let line = path_builder.build();

                *path = ShapePath::build_as(&line);
            }
        }
    }
}

fn draw_partial_connections<T: Nodes>(
    mut commands: Commands,
    cursor: Res<CursorPosition>,
    mut q_connections: Query<(Entity, &PartialConnection, &mut Path)>,
    q_start: Query<&GlobalTransform, Or<(With<NodeInput<T>>, With<NodeOutput>)>>,
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

                path_builder.move_to(Vec2::new(
                    transform.translation().x,
                    transform.translation().y,
                ));
                path_builder.line_to(cursor.position());

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
