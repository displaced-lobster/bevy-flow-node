use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    ecs::system::Resource,
};
use std::marker::PhantomData;

pub mod connection;
pub mod cursor;
pub mod node;

pub use crate::node::{
    Node,
    NodeIO,
    NodeInput,
    NodeOutput,
    NodeResolver,
    NodeType,
    OutputNode,
};

#[derive(Default)]
pub struct NodePlugins<N: NodeResolver>(PhantomData<N>);

impl<N: NodeResolver + 'static + Default + Resource> PluginGroup for NodePlugins<N> {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(connection::ConnectionPlugin)
            .add(cursor::CursorPlugin)
            .add(node::NodePlugin::<N>::default());
    }
}
