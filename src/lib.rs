use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::marker::PhantomData;

pub mod connection;
pub mod cursor;
pub mod node;

pub use crate::node::{Node, NodeInput, NodeOutput, Nodes, OutputNode};

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
