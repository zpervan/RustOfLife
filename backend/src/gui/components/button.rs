use bevy::prelude::*;

use crate::core::path::*;
use crate::gui::components::constants::BUTTON_COLOR;

#[derive(Debug)]
pub struct Button {
    pub button_ui: ButtonBundle,
    pub button_text: TextBundle,
}

impl Button {
    pub fn new(text: String, asset_server: &Res<AssetServer>) -> Button {
        let button_style: Style = Style {
            size: Size::default(),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_ui = ButtonBundle {
            style: button_style,
            ..default()
        };

        let button_text_style = TextStyle {
            font: asset_server.load(get_font_path("SpaceMonoNerdFont-Regular.ttf".to_string())),
            font_size: 40.0,
            color: BUTTON_COLOR,
        };

        let button_text = TextBundle::from_section(text, button_text_style);

        Button { button_ui, button_text }
    }
}
