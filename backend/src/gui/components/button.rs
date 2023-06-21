use bevy::prelude::*;

use crate::core::path::*;
use crate::gui::components::constants::*;

pub fn default_button_style() -> Style {
    Style {
        size: Size::default(),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn button_text(text: String, asset_server: Res<AssetServer>) -> TextBundle {
    TextBundle::from_section(text, TextStyle {
        font: asset_server.load(get_font_path("SpaceMonoNerdFont-Regular.ttf".to_string())),
        font_size: 40.0,
        color: BUTTON_NORMAL_COLOR,
    })
}
