use bevy::prelude::*;
use std::{fmt::Display, marker::PhantomData};

use crate::{
    node::{NodeEvent, NodeSet},
    widget::{Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct DisplayWidgetPlugin<N: NodeSet>(PhantomData<N>);

impl<N: NodeSet> Plugin for DisplayWidgetPlugin<N>
where
    N::NodeIO: Display,
{
    fn build(&self, app: &mut App) {
        app.add_plugin(WidgetPlugin::<N, DisplayWidget>::default())
            .add_system(update_display_widget::<N>);
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct DisplayWidget {
    pub size: Vec2,
}

impl<N: NodeSet> Widget<N> for DisplayWidget {
    fn build(
        &mut self,
        commands: &mut Commands,
        area: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let text_style_title = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        };

        self.size = area;

        commands
            .spawn(Text2dBundle {
                text: Text::from_section("", text_style_title),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..default()
            })
            .id()
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn update_display_widget<N: NodeSet>(
    mut ev_node: EventReader<NodeEvent<N>>,
    mut q_text: Query<(&Parent, &mut Text)>,
    q_widget: Query<Entity, With<DisplayWidget>>,
) where
    N::NodeIO: Display,
{
    for ev in ev_node.iter() {
        #[allow(irrefutable_let_patterns)]
        if let NodeEvent::Resolved(value) = ev {
            for entity in q_widget.iter() {
                for (parent, mut text) in q_text.iter_mut() {
                    if parent.get() == entity {
                        text.sections[0].value = value.to_string();
                    }
                }
            }
        }
    }
}
