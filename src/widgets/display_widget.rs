use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{Text2dBounds, TextLayoutInfo},
};
use std::{fmt::Display, marker::PhantomData};

use crate::{
    assets::DefaultAssets,
    node::{FlowNodeEvent, FlowNodeSet},
    widget::{SlotWidget, Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct DisplayWidgetPlugin<N: FlowNodeSet + SlotWidget<N, DisplayWidget>>(PhantomData<N>);

impl<N: FlowNodeSet + SlotWidget<N, DisplayWidget>> Plugin for DisplayWidgetPlugin<N>
where
    N::NodeIO: Display,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(WidgetPlugin::<N, DisplayWidget>::default())
            .add_systems(Update, update_display_widget::<N>);
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct DisplayWidget {
    pub parent: Option<Entity>,
    pub size: Vec2,
}

impl Widget for DisplayWidget {
    type WidgetValue = String;

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

        commands.entity(entity).insert((
            Anchor::TopLeft,
            Text::from_section("", text_style_title),
            TextLayoutInfo::default(),
            Text2dBounds {
                size: Vec2::new(area.x / 2.0, area.y),
            },
            Visibility::default(),
        ));
    }

    fn set_parent(&mut self, parent: Entity) {
        self.parent = Some(parent);
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

fn update_display_widget<N: FlowNodeSet>(
    mut ev_node: EventReader<FlowNodeEvent<N>>,
    mut q_text: Query<(&DisplayWidget, &mut Text)>,
) where
    N::NodeIO: Display,
{
    for ev in ev_node.read() {
        if let FlowNodeEvent::Resolved((entity, value)) = ev {
            for (widget, mut text) in q_text.iter_mut() {
                if widget.parent == Some(*entity) {
                    text.sections[0].value = value.to_string();
                }
            }
        }
    }
}
