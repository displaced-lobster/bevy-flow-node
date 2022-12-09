use bevy::prelude::*;
use std::marker::PhantomData;

use crate::{
    assets::DefaultAssets,
    interactions::{Clickable, Clicked},
    node::{Node, NodeSet},
    template::NodeSlot,
};

pub trait Widget: Clone + Component {
    type WidgetValue;

    fn blur(&mut self) {}
    fn build(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        area: Vec2,
        assets: &Res<DefaultAssets>,
    );
    fn can_click(&self) -> bool {
        false
    }
    fn clean(&mut self) {}
    fn dirty(&self) -> bool {
        false
    }
    fn focus(&mut self) {}
    fn size(&self) -> Vec2;
    fn get_value(&self) -> Option<Self::WidgetValue> {
        None
    }
    fn set_parent(&mut self, _parent: Entity) {}
    fn set_value(&mut self, _value: Self::WidgetValue) {}
}

pub trait SlotWidget<N: NodeSet, W: Widget> {
    fn get_widget(&self) -> Option<W>;
    fn set_value(&mut self, _value: W::WidgetValue) {}
}

#[derive(Default)]
pub struct WidgetPlugin<N: NodeSet + SlotWidget<N, W>, W: Widget>(PhantomData<(N, W)>);

impl<N: NodeSet + SlotWidget<N, W>, W: Widget> Plugin for WidgetPlugin<N, W> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveWidget::<N, W>::default())
            .add_system(focus_blur_widget::<N, W>)
            .add_system(build_widget::<N, W>)
            .add_system(slot_widget::<N, W>);
    }
}

#[derive(Resource)]
struct ActiveWidget<N: NodeSet, W: Widget> {
    entity: Option<Entity>,
    _phantom: PhantomData<(N, W)>,
}

impl<N: NodeSet, W: Widget> Default for ActiveWidget<N, W> {
    fn default() -> Self {
        Self {
            entity: None,
            _phantom: PhantomData,
        }
    }
}

fn focus_blur_widget<N: NodeSet, W: Widget>(
    mut active_widget: ResMut<ActiveWidget<N, W>>,
    mut ev_click: EventReader<Clicked>,
    mut query: Query<(Entity, &mut W), With<Clickable>>,
) {
    for ev in ev_click.iter() {
        let mut needs_blur = false;

        if let Clicked(Some(entity)) = ev {
            if let Ok((_, mut widget)) = query.get_mut(*entity) {
                active_widget.entity = Some(*entity);
                widget.focus();
            } else {
                needs_blur = true;
            }
        } else {
            needs_blur = true;
        }

        if needs_blur && active_widget.entity.is_some() {
            if let Ok((_, mut widget)) = query.get_mut(active_widget.entity.unwrap()) {
                widget.blur();
            }
            active_widget.entity = None;
        }
    }
}

fn build_widget<N: NodeSet, W: Widget>(
    mut commands: Commands,
    assets: Res<DefaultAssets>,
    mut q_widget: Query<(Entity, &mut W, &NodeSlot)>,
) {
    for (entity, mut widget, slot) in q_widget.iter_mut() {
        widget.build(
            entity,
            &mut commands,
            Vec2::new(slot.width, slot.height),
            &assets,
        );

        commands.entity(entity).remove::<NodeSlot>();

        if widget.can_click() {
            commands
                .entity(entity)
                .insert(Clickable::Area(widget.size()));
        }
    }
}

fn slot_widget<N: NodeSet + SlotWidget<N, W>, W: Widget>(
    mut commands: Commands,
    q_node: Query<&Node<N>>,
    q_slot: Query<(Entity, &Parent), (With<NodeSlot>, Without<W>)>,
) {
    for (entity, parent) in q_slot.iter() {
        if let Ok(node) = q_node.get(parent.get()) {
            if let Some(mut widget) = node.0.get_widget() {
                widget.set_parent(parent.get());
                commands.entity(entity).insert(widget);
            }
        }
    }
}
