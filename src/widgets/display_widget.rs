use bevy::{
    prelude::*,
    text::{Text2dBounds, Text2dSize},
};
use std::{fmt::Display, marker::PhantomData};

use crate::{
    assets::DefaultAssets,
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
        entity: Entity,
        commands: &mut Commands,
        area: Vec2,
        assets: &Res<DefaultAssets>,
    ) {
        let text_style_title = TextStyle {
            font: assets.font.clone(),
            font_size: 16.0,
            color: Color::WHITE,
        };

        self.size = area;

        commands
            .entity(entity)
            .insert((
                Text::from_section("", text_style_title),
                Text2dSize::default(),
                Text2dBounds { size: Vec2::new(area.x / 2.0, area.y) },
                Visibility::default(),
                ComputedVisibility::default(),
            ));
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn update_display_widget<N: NodeSet>(
    mut ev_node: EventReader<NodeEvent<N>>,
    mut q_text: Query<&mut Text, With<DisplayWidget>>,
) where
    N::NodeIO: Display,
{
    for ev in ev_node.iter() {
        #[allow(irrefutable_let_patterns)]
        if let NodeEvent::Resolved(value) = ev {
            for mut text in q_text.iter_mut() {
                text.sections[0].value = value.to_string();
            }
        }
    }
}
