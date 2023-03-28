use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::marker::PhantomData;

pub mod assets;
pub mod camera;
pub mod connection;
pub mod cursor;
pub mod interactions;
pub mod menu;
pub mod node;
pub mod template;
pub mod widget;
pub mod widgets;

pub use crate::{
    camera::PanCameraPlugin,
    cursor::CursorCamera,
    menu::{FlowNodeMenu, FlowNodeMenuPlugin},
    node::{FlowNode, FlowNodeEvent, FlowNodeInput, FlowNodeOutput, FlowNodeSet},
    template::{FlowNodeSlot, FlowNodeTemplate},
    widget::{SlotWidget, Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct FlowNodePlugins<N: FlowNodeSet>(PhantomData<N>);

impl<N: FlowNodeSet> PluginGroup for FlowNodePlugins<N> {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(assets::DefaultAssetsPlugin)
            .add(connection::ConnectionPlugin::<N>::default())
            .add(cursor::CursorPlugin)
            .add(interactions::InteractionPlugin)
            .add(node::FlowNodePlugin::<N>::default())
            .add(template::FlowNodeTemplatePlugin::<N>::default())
    }
}
