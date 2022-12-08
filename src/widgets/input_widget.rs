use bevy::{prelude::*, text::Text2dBounds};
use std::{fmt::Display, marker::PhantomData};

use crate::{
    assets::DefaultAssets,
    connection::ConnectionEvent,
    node::{Node, NodeSet},
    widget::{SlotWidget, Widget, WidgetPlugin},
};

pub trait InputWidgetValue {
    fn pop(&mut self);
    fn push(&mut self, c: char);
    fn to_string(&self) -> String;
}

impl InputWidgetValue for String {
    fn pop(&mut self) {
        self.pop();
    }

    fn push(&mut self, c: char) {
        self.push(c);
    }

    fn to_string(&self) -> String {
        self.clone()
    }
}

#[derive(Clone, Default)]
pub struct NumberInput {
    pub value: f32,
    pub s_value: String,
}

impl Display for NumberInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s_value)
    }
}

impl From<f32> for NumberInput {
    fn from(f: f32) -> Self {
        Self {
            value: f,
            ..default()
        }
    }
}

impl Into<String> for NumberInput {
    fn into(self) -> String {
        self.s_value.clone()
    }
}

impl InputWidgetValue for NumberInput {
    fn pop(&mut self) {
        self.s_value.pop();

        if let Ok(value) = self.s_value.parse() {
            self.value = value;
        } else {
            self.value = 0.0;
        }
    }

    fn push(&mut self, c: char) {
        if c.is_digit(10) {
            self.s_value.push(c);

            if let Ok(value) = self.s_value.parse::<f32>() {
                self.value = value;
            }
        } else if c == '.' && !self.s_value.chars().any(|c| c == '.') {
            self.s_value.push(c);
        }
    }

    fn to_string(&self) -> String {
        self.s_value.clone()
    }
}

#[derive(Default)]
pub struct InputWidgetPlugin<N: NodeSet, V: InputWidgetValue>(PhantomData<(N, V)>);

impl<N: NodeSet, V: InputWidgetValue + 'static + Clone + Default + Send + Sync> Plugin
    for InputWidgetPlugin<N, V>
where
    N: SlotWidget<N, InputWidget<V>>,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(WidgetPlugin::<N, InputWidget<V>>::default())
            .add_system(input_widget_input::<V>)
            .add_system(input_widget_value::<N, V>);
    }
}

#[derive(Clone, Component, Default)]
pub struct InputWidget<V: InputWidgetValue> {
    pub active: bool,
    pub dirty: bool,
    pub size: Vec2,
    pub text_entity: Option<Entity>,
    pub value: V,
}

impl<V: InputWidgetValue + 'static + Clone + Send + Sync> Widget for InputWidget<V> {
    type WidgetValue = V;
    fn build(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        area: Vec2,
        assets: &Res<DefaultAssets>,
    ) {
        let text_style_title = TextStyle {
            font: assets.font.clone(),
            font_size: 16.0,
            color: Color::BLACK,
        };

        self.size = area;

        let child = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(self.size),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let text_entity = parent
                    .spawn(Text2dBundle {
                        text: Text::from_section("", text_style_title),
                        text_2d_bounds: Text2dBounds { size: self.size },
                        transform: Transform::from_xyz(-self.size.x / 2.0, self.size.y / 2.0, 2.0),
                        ..default()
                    })
                    .id();

                self.text_entity = Some(text_entity);
            })
            .id();

        commands.entity(entity).push_children(&[child]);
    }

    fn blur(&mut self) {
        self.active = false;
    }

    fn can_click(&self) -> bool {
        true
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

    fn get_value(&self) -> Option<V> {
        Some(self.value.clone())
    }

    fn set_value(&mut self, value: V) {
        self.value = value;
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn input_widget_input<V: InputWidgetValue + 'static + Send + Sync>(
    mut ev_char: EventReader<ReceivedCharacter>,
    mut query: Query<&mut InputWidget<V>>,
) {
    const BACKSPACE: char = '\u{0008}';

    for ev in ev_char.iter() {
        for mut widget in query.iter_mut() {
            if widget.active {
                widget.dirty = true;

                if ev.char.is_ascii_graphic() {
                    widget.value.push(ev.char);
                } else if ev.char == BACKSPACE {
                    widget.value.pop();
                }
            }
        }
    }
}

fn input_widget_value<N: NodeSet, V: InputWidgetValue + 'static + Clone + Send + Sync>(
    mut ev_conn: EventWriter<ConnectionEvent>,
    mut q_node: Query<&mut Node<N>>,
    mut q_widget: Query<(&Parent, &mut InputWidget<V>)>,
    mut q_text: Query<&mut Text>,
) where
    N: SlotWidget<N, InputWidget<V>>,
{
    for (parent, mut widget) in q_widget.iter_mut() {
        if widget.dirty() {
            widget.clean();

            if let Some(entity) = widget.text_entity {
                if let Ok(mut text) = q_text.get_mut(entity) {
                    text.sections[0].value = widget.get_value().unwrap().to_string();
                }
            }

            if let Ok(mut node) = q_node.get_mut(parent.get()) {
                (*node).set_value(widget.get_value().unwrap());
                ev_conn.send(ConnectionEvent::Propagate);
            }
        }
    }
}
