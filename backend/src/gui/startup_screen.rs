use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::gui::components::button::Button;
use crate::gui::components::constants::*;

pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting application");

    commands.spawn(Camera2dBundle::default());

    let start_button = Button::new("Start".to_string(), &asset_server);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(start_button.button_ui)
                .with_children(|parent| { parent.spawn(start_button.button_text); });
        });
}