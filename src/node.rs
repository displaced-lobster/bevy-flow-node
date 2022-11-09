use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    text::Text2dBounds,
};

use crate::cursor::CursorPosition;

pub struct FlowNodePlugin;

impl Plugin for FlowNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FlowNodeConfig::default())
            .add_startup_system(setup)
            .add_system(activate_flow_node)
            .add_system(build_flow_node)
            .add_system(drag_flow_node);
    }
}

#[derive(Default)]
struct ActiveFlowNode {
    entity: Option<Entity>,
    offset: Vec2,
}

#[derive(Component)]
struct FlowNode;

pub struct FlowNodeConfig {
    pub handle_size_io: f32,
    pub handle_size_title: f32,
    pub padding: f32,
    pub font_size_body: f32,
    pub font_size_title: f32,
}

impl Default for FlowNodeConfig {
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
struct FlowNodeHandle;

#[derive(Component, Default)]
pub struct FlowNodeInput {
    pub connection: Option<Entity>,
}

#[derive(Component)]
pub struct FlowNodeOutput;

struct FlowNodeResources {
    material_handle_input: Handle<ColorMaterial>,
    material_handle_output: Handle<ColorMaterial>,
    material_handle_title: Handle<ColorMaterial>,
    mesh_handle: Handle<Mesh>,
    mesh_handle_io: Handle<Mesh>,
    text_style_body: TextStyle,
    text_style_title: TextStyle,
}

#[derive(Component)]
pub struct FlowNodeTemplate {
    pub activate: bool,
    pub position: Vec2,
    pub title: String,
    pub width: f32,
    pub n_inputs: usize,
    pub n_outputs: usize,
}

impl Default for FlowNodeTemplate {
    fn default() -> Self {
        Self {
            activate: false,
            position: Vec2::ZERO,
            title: "Flow Node".to_string(),
            width: 200.0,
            n_inputs: 0,
            n_outputs: 0,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<FlowNodeConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(ActiveFlowNode::default());

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

    commands.insert_resource(FlowNodeResources {
        material_handle_input: materials.add(Color::YELLOW.into()),
        material_handle_output: materials.add(Color::RED.into()),
        material_handle_title: materials.add(Color::PURPLE.into()),
        mesh_handle: meshes.add(shape::Circle::new(config.handle_size_title).into()),
        mesh_handle_io: meshes.add(shape::Circle::new(config.handle_size_io).into()),
        text_style_body,
        text_style_title,
    });
}

fn activate_flow_node(
    mut active_node: ResMut<ActiveFlowNode>,
    config: Res<FlowNodeConfig>,
    cursor: Res<CursorPosition>,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<(&Parent, &GlobalTransform), With<FlowNodeHandle>>,
    q_parent: Query<&GlobalTransform>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (parent, transform) in query.iter() {
            let translation = transform.translation();

            if (translation.x - cursor.x).abs() < config.handle_size_title && (translation.y - cursor.y).abs() < config.handle_size_title {
                let transform = q_parent.get(parent.get()).unwrap();
                let translation = transform.translation();

                active_node.entity = Some(parent.get());
                active_node.offset = Vec2::new(
                    translation.x - cursor.x,
                    translation.y - cursor.y,
                );
                break;
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        active_node.entity = None;
        active_node.offset = Vec2::ZERO;
    }
}

fn build_flow_node(
    mut commands: Commands,
    config: Res<FlowNodeConfig>,
    resources: Res<FlowNodeResources>,
    mut active_node: ResMut<ActiveFlowNode>,
    query: Query<(Entity, &FlowNodeTemplate)>,
) {
    for (template_entity, template) in query.iter() {
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

        let entity = commands
            .spawn_bundle(SpriteBundle {
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
                    .insert(FlowNodeHandle);

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

                for _ in 0..template.n_inputs {
                    parent
                        .spawn_bundle(MaterialMesh2dBundle {
                            material: resources.material_handle_input.clone(),
                            mesh: bevy::sprite::Mesh2dHandle(resources.mesh_handle_io.clone()),
                            transform: Transform::from_xyz(
                                offset_x + config.handle_size_io,
                                offset_y - config.handle_size_io - config.padding,
                                1.0,
                            ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TransformBundle::default())
                                .insert(FlowNodeInput::default());
                        });


                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section("Input", resources.text_style_body.clone()),
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

                offset_y = offset_y_body;

                let offset_x = config.padding;

                for _ in 0..template.n_outputs {
                    parent
                        .spawn_bundle(MaterialMesh2dBundle {
                            material: resources.material_handle_output.clone(),
                            mesh: bevy::sprite::Mesh2dHandle(resources.mesh_handle_io.clone()),
                            transform: Transform::from_xyz(
                                offset_x + node_size.x / 2.0 - 2.0 * config.handle_size_io - config.padding,
                                offset_y - config.handle_size_io - config.padding,
                                2.0,
                            ),
                            ..default()
                        })
                        .insert(FlowNodeOutput);

                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section("Output", resources.text_style_body.clone()),
                        text_2d_bounds: Text2dBounds { size: bounds_io },
                        transform: Transform::from_xyz(
                            offset_x + config.padding,
                            offset_y - config.font_size_body + config.handle_size_io * 2.0,
                            1.0,
                        ),
                        ..default()
                    });

                    offset_y -= height_body / 2.0;
                }
            })
            .insert(FlowNode)
            .id();

        if template.activate {
            active_node.entity = Some(entity);
            active_node.offset = Vec2::ZERO;
        }

        commands.entity(template_entity).despawn_recursive();
    }
}

fn drag_flow_node(
    active_node: Res<ActiveFlowNode>,
    cursor: Res<CursorPosition>,
    mut query: Query<&mut Transform, With<FlowNode>>,
) {
    if let Some(entity) = active_node.entity {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation.x = cursor.x + active_node.offset.x;
            transform.translation.y = cursor.y + active_node.offset.y;
        }
    }
}
