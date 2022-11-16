use bevy::prelude::*;
use std::marker::PhantomData;

use crate::{
    cursor::CursorPosition,
    node::{NodeSlot, Nodes},
};

pub trait Widget<N: Nodes>: Clone + Component {
    fn blur(&mut self) {}
    fn build(
        &mut self,
        commands: &mut Commands,
        area: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Entity;
    fn clean(&mut self) {}
    fn dirty(&self) -> bool {
        false
    }
    fn focus(&mut self) {}
    fn size(&self) -> Vec2;
    fn get_value(&self) -> N::NodeIO {
        N::NodeIO::default()
    }
    fn set_value(&mut self, _value: N::NodeIO) {}
}

pub trait ReceiveWidgetValue<N: Nodes> {
    fn receive_value(&mut self, value: N::NodeIO);
}

#[derive(Default)]
pub struct WidgetPlugin<N: Nodes, W: Widget<N>>(PhantomData<(N, W)>);

impl<N: Nodes, W: Widget<N>> Plugin for WidgetPlugin<N, W> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveWidget::default())
            .add_system(focus_widget::<N, W>)
            .add_system(build_widget::<N, W>)
            .add_system(blur_widget::<N, W>)
            .add_system(slot_widget::<N, W>);
    }
}

#[derive(Default)]
struct ActiveWidget {
    entity: Option<Entity>,
}

fn focus_widget<N: Nodes, W: Widget<N>>(
    cursor: Res<CursorPosition>,
    mouse_button: Res<Input<MouseButton>>,
    mut active_widget: ResMut<ActiveWidget>,
    mut query: Query<(Entity, &mut W, &GlobalTransform)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, mut widget, transform) in query.iter_mut() {
        let size = widget.size();
        let position = transform.translation().truncate() - 0.5 * size;

        if cursor.x >= position.x
            && cursor.x <= position.x + size.x
            && cursor.y >= position.y
            && cursor.y <= position.y + size.y
        {
            active_widget.entity = Some(entity);
            widget.focus();
            return;
        }
    }
}

fn build_widget<N: Nodes, W: Widget<N>>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_widget: Query<(Entity, &mut W, &NodeSlot)>,
) {
    for (entity, mut widget, slot) in q_widget.iter_mut() {
        let widget_entity = widget.build(
            &mut commands,
            Vec2::new(slot.width, slot.height),
            &asset_server,
        );

        commands
            .entity(entity)
            .push_children(&[widget_entity])
            .remove::<NodeSlot>();
    }
}

fn blur_widget<N: Nodes, W: Widget<N>>(
    cursor: Res<CursorPosition>,
    mouse_button: Res<Input<MouseButton>>,
    mut active_widget: ResMut<ActiveWidget>,
    mut query: Query<(&mut W, &GlobalTransform)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) || active_widget.entity.is_none() {
        return;
    }

    if let Ok((mut widget, transform)) = query.get_mut(active_widget.entity.unwrap()) {
        let size = widget.size();
        let position = transform.translation().truncate() - 0.5 * size;

        if !(cursor.x >= position.x
            && cursor.x <= position.x + size.x
            && cursor.y >= position.y
            && cursor.y <= position.y + size.y)
        {
            active_widget.entity = None;
            widget.blur();
            return;
        }
    }
}

fn slot_widget<N: Nodes, W: Widget<N>>(
    mut commands: Commands,
    q_widget: Query<(Entity, &W), Without<NodeSlot>>,
    q_slot: Query<(Entity, &Parent), With<NodeSlot>>,
) {
    for (entity, widget) in q_widget.iter() {
        for (slot_entity, parent) in q_slot.iter() {
            if parent.get() == entity {
                commands.entity(slot_entity).insert(widget.clone());

                commands.entity(entity).remove::<W>();
            }
        }
    }
}
