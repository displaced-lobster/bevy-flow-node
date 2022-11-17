use bevy::{prelude::*, text::Text2dBounds};
use std::{marker::PhantomData, ops::AddAssign};

use crate::{
    connection::ConnectionEvent,
    node::{Node, Nodes},
    widget::{ReceiveWidgetValue, Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct TextInputWidgetPlugin<N: Nodes>(PhantomData<N>);

impl<N: Nodes> Plugin for TextInputWidgetPlugin<N>
where
    N: ReceiveWidgetValue<N>,
    N::NodeIO: AddAssign<char> + Into<String>,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(WidgetPlugin::<N, TextInputWidget<N>>::default())
            .add_system(text_widget_input::<N>)
            .add_system(text_widget_value::<N>);
    }
}

#[derive(Clone, Component, Default)]
pub struct TextInputWidget<N: Nodes> {
    pub active: bool,
    pub dirty: bool,
    pub size: Vec2,
    pub text_entity: Option<Entity>,
    pub value: N::NodeIO,
}

impl<N: Nodes> Widget<N> for TextInputWidget<N> {
    fn build(
        &mut self,
        commands: &mut Commands,
        area: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let text_style_title = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::BLACK,
        };

        self.size = area;

        commands
            .spawn(SpatialBundle::default())
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(self.size),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        self.size.x / 2.0 - 5.0,
                        -self.size.y / 2.0,
                        1.0,
                    ),
                    ..default()
                });

                let text_entity = parent
                    .spawn(Text2dBundle {
                        text: Text::from_section("", text_style_title),
                        text_2d_bounds: Text2dBounds { size: self.size },
                        transform: Transform::from_xyz(0.0, 0.0, 2.0),
                        ..default()
                    })
                    .id();

                self.text_entity = Some(text_entity);
            })
            .id()
    }

    fn blur(&mut self) {
        self.active = false;
    }

    fn clean(&mut self) {
        self.dirty = false;
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn focus(&mut self) {
        self.active = true;
    }

    fn get_value(&self) -> N::NodeIO {
        self.value.clone()
    }

    fn set_value(&mut self, value: N::NodeIO) {
        self.value = value.clone();
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn text_widget_input<N: Nodes>(
    mut ev_char: EventReader<ReceivedCharacter>,
    mut query: Query<&mut TextInputWidget<N>>,
) where
    N::NodeIO: AddAssign<char>,
{
    for ev in ev_char.iter() {
        for mut widget in query.iter_mut() {
            if widget.active {
                widget.dirty = true;
                widget.value += ev.char;
            }
        }
    }
}

fn text_widget_value<N: Nodes>(
    mut ev_conn: EventWriter<ConnectionEvent>,
    mut q_node: Query<&mut Node<N>>,
    mut q_widget: Query<(&Parent, &mut TextInputWidget<N>)>,
    mut q_text: Query<&mut Text>,
) where
    N: ReceiveWidgetValue<N>,
    N::NodeIO: Into<String>,
{
    for (parent, mut widget) in q_widget.iter_mut() {
        if widget.dirty() {
            widget.clean();

            if let Some(entity) = widget.text_entity {
                if let Ok(mut text) = q_text.get_mut(entity) {
                    text.sections[0].value = widget.get_value().into();
                }
            }

            if let Ok(mut node) = q_node.get_mut(parent.get()) {
                node.node.receive_value(widget.get_value());
                ev_conn.send(ConnectionEvent::Propagate);
            }
        }
    }
}
