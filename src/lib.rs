use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::marker::PhantomData;

pub mod connection;
pub mod cursor;
pub mod menu;
pub mod node;
pub mod widget;
pub mod widgets;

pub use crate::{
    menu::{NodeMenu, NodeMenuPlugin},
    node::{Node, NodeEvent, NodeInput, NodeOutput, NodeSlot, Nodes, OutputNode},
    widget::{Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct NodePlugins<T: Nodes>(PhantomData<T>);

impl<T: Nodes> PluginGroup for NodePlugins<T> {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(connection::ConnectionPlugin::<T>::default())
            .add(cursor::CursorPlugin)
            .add(node::NodePlugin::<T>::default());
    }
}
