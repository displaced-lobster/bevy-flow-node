use bevy::{ecs::system::Resource, prelude::*};
use std::marker::PhantomData;

use crate::{cursor::CursorPosition, node::NodeSet};

#[derive(Default)]
pub struct NodeMenuPlugin<M: NodeMenu<N>, N: NodeSet>(PhantomData<(M, N)>);

impl<M: NodeMenu<N>, N: NodeSet> Plugin for NodeMenuPlugin<M, N> {
    fn build(&self, app: &mut App) {
        app.insert_resource(M::default())
            .insert_resource(MenuConfig::default())
            .add_event::<MenuEvent<N>>()
            .add_startup_system(setup)
            .add_system(select_menu_option::<N>.before(close_menu))
            .add_system(close_menu)
            .add_system(open_menu::<M, N>)
            .add_system(build_from_menu_select::<M, N>);
    }
}

pub trait NodeMenu<N: NodeSet>: Default + Resource {
    fn build(&self, commands: &mut Commands, node: &N);
    fn options(&self) -> Vec<(String, N)>;
}

#[derive(Component)]
struct Menu;

#[derive(Resource)]
pub struct MenuConfig {
    pub color: Color,
    pub font_size: f32,
    pub option_height: f32,
    pub width: f32,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.1, 0.1, 0.1),
            font_size: 16.0,
            option_height: 20.0,
            width: 150.0,
        }
    }
}

enum MenuEvent<N: NodeSet> {
    Selected(N),
}

#[derive(Component)]
struct MenuOption<N: NodeSet> {
    node: N,
}

#[derive(Resource)]
struct MenuResources {
    text_style: TextStyle,
}

fn setup(mut commands: Commands, assert_server: Res<AssetServer>, config: Res<MenuConfig>) {
    let text_style = TextStyle {
        font: assert_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: config.font_size,
        color: Color::WHITE,
    };

    commands.insert_resource(MenuResources { text_style });
}

fn close_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    q_menu: Query<Entity, With<Menu>>,
) {
    if mouse.just_released(MouseButton::Left)
        || mouse.just_released(MouseButton::Right)
        || keys.just_pressed(KeyCode::A)
    {
        for entity in q_menu.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn open_menu<M: NodeMenu<N>, N: NodeSet>(
    mut commands: Commands,
    config: Res<MenuConfig>,
    cursor: Res<CursorPosition>,
    keys: Res<Input<KeyCode>>,
    menu: Res<M>,
    res: Res<MenuResources>,
) {
    if keys.just_pressed(KeyCode::A) {
        let options = menu.options();
        let height = config.option_height * options.len() as f32;

        commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(config.width), Val::Px(height)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(cursor.screen_x),
                        top: Val::Px(cursor.screen_y),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for option in options.iter() {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Px(config.width),
                                    Val::Px(config.option_height),
                                ),
                                padding: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: config.color.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                option.0.clone(),
                                res.text_style.clone(),
                            ));
                        })
                        .insert(MenuOption {
                            node: option.1.clone(),
                        });
                }
            })
            .insert(Menu);
    }
}

fn select_menu_option<N: NodeSet>(
    mut commands: Commands,
    mut events: EventWriter<MenuEvent<N>>,
    q_options: Query<(Entity, &MenuOption<N>, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (entity, option, interaction) in q_options.iter() {
        match interaction {
            Interaction::Clicked => {
                commands.entity(entity).despawn_recursive();
                events.send(MenuEvent::Selected(option.node.clone()));
            }
            _ => {}
        }
    }
}

fn build_from_menu_select<M: NodeMenu<N>, N: NodeSet>(
    mut commands: Commands,
    menu: Res<M>,
    mut events: EventReader<MenuEvent<N>>,
) {
    for event in events.iter() {
        match event {
            MenuEvent::Selected(node) => {
                menu.build(&mut commands, &node);
            }
        }
    }
}
