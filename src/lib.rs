use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod connection;
pub mod cursor;
pub mod node;

pub struct FlowNodePlugins;

impl PluginGroup for FlowNodePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(connection::ConnectionPlugin)
            .add(cursor::CursorPlugin)
            .add(node::FlowNodePlugin);
    }
}
