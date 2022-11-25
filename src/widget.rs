use bevy::prelude::*;
use std::marker::PhantomData;

use crate::{
    interactions::{Clickable, Clicked},
    node::{NodeSet, NodeSlot},
};

pub trait Widget<N: NodeSet>: Clone + Component {
    fn blur(&mut self) {}
    fn build(
        &mut self,
        commands: &mut Commands,
        area: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Entity;
    fn can_click(&self) -> bool {
        false
    }
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

pub trait ReceiveWidgetValue<N: NodeSet> {
    fn receive_value(&mut self, value: N::NodeIO);
}

#[derive(Default)]
pub struct WidgetPlugin<N: NodeSet, W: Widget<N>>(PhantomData<(N, W)>);

impl<N: NodeSet, W: Widget<N>> Plugin for WidgetPlugin<N, W> {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveWidget::<N, W>::default())
            .add_system(focus_blur_widget::<N, W>)
            .add_system(build_widget::<N, W>)
            .add_system(slot_widget::<N, W>);
    }
}

#[derive(Resource)]
struct ActiveWidget<N: NodeSet, W: Widget<N>> {
    entity: Option<Entity>,
    _phantom: PhantomData<(N, W)>,
}

impl<N: NodeSet, W: Widget<N>> Default for ActiveWidget<N, W> {
    fn default() -> Self {
        Self {
            entity: None,
            _phantom: PhantomData,
        }
    }
}

fn focus_blur_widget<N: NodeSet, W: Widget<N>>(
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

fn build_widget<N: NodeSet, W: Widget<N>>(
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

        if widget.can_click() {
            commands
                .entity(entity)
                .insert(Clickable::Area(widget.size()));
        }
    }
}

fn slot_widget<N: NodeSet, W: Widget<N>>(
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
