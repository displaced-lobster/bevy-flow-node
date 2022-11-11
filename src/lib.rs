use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::marker::PhantomData;

pub mod connection;
pub mod cursor;
pub mod node;

pub use crate::node::{Node, NodeInput, NodeOutput, NodeType, OutputNode};

#[derive(Default)]
pub struct NodePlugins<T: NodeType>(PhantomData<T>);

impl<T: NodeType> PluginGroup for NodePlugins<T> {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(connection::ConnectionPlugin::<T>::default())
            .add(cursor::CursorPlugin)
            .add(node::NodePlugin::<T>::default());
    }
}
