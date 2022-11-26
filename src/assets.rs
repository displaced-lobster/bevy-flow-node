use bevy::prelude::*;

#[derive(Resource)]
pub struct DefaultAssets {
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
}

impl FromWorld for DefaultAssets {
    fn from_world(world: &mut World) -> Self {
        let font = world.get_resource_mut::<Assets<Font>>().unwrap().add(
            Font::try_from_bytes(include_bytes!("assets/fonts/FiraMono-Medium.ttf").to_vec())
                .unwrap(),
        );
        let font_bold = world.get_resource_mut::<Assets<Font>>().unwrap().add(
            Font::try_from_bytes(include_bytes!("assets/fonts/FiraSans-Bold.ttf").to_vec())
                .unwrap(),
        );

        Self { font, font_bold }
    }
}

pub struct DefaultAssetsPlugin;

impl Plugin for DefaultAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultAssets>();
    }
}
