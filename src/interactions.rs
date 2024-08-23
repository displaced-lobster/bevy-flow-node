use bevy::prelude::*;

use crate::cursor::CursorPosition;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Clicked>().add_systems(Update, handle_click);
    }
}

#[derive(Component)]
pub enum Clickable {
    Area(Vec2),
    Radius(f32),
}

impl Clickable {
    fn clicked(&self, pos: Vec2, click_pos: Vec2) -> bool {
        match *self {
            Self::Area(area) => {
                let pos = pos - 0.5 * area;

                click_pos.x >= pos.x
                    && click_pos.x <= pos.x + area.x
                    && click_pos.y >= pos.y
                    && click_pos.y <= pos.y + area.y
            }
            Self::Radius(radius) => {
                let pos = pos - click_pos;

                pos.length() <= radius
            }
        }
    }
}

#[derive(Event)]
pub struct Clicked(pub Option<Entity>);

fn handle_click(
    cursor: Res<CursorPosition>,
    mouse_button: Res<Input<MouseButton>>,
    mut ev_click: EventWriter<Clicked>,
    query: Query<(Entity, &Clickable, &GlobalTransform)>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let click_pos = cursor.position();
        let mut clicked = query
            .iter()
            .filter(|(_, clickable, transform)| {
                clickable.clicked(transform.translation().truncate(), click_pos)
            })
            .collect::<Vec<_>>();

        clicked.sort_by(|(_, _, a_transform), (_, _, b_transform)| {
            a_transform
                .translation()
                .z
                .partial_cmp(&b_transform.translation().z)
                .unwrap()
        });

        if let Some((entity, _, _)) = clicked.pop() {
            ev_click.send(Clicked(Some(entity)));
        } else {
            ev_click.send(Clicked(None));
        }
    }
}
