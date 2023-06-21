use bevy::prelude::*;

use crate::gui::components::button::*;
use crate::gui::components::constants::*;

pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Starting application");

    commands.spawn(Camera2dBundle::default());

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
                .spawn(ButtonBundle {
                    style: default_button_style(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(button_text("START".to_string(), asset_server));
                });
        });
}

pub fn update(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BUTTON_PRESSED_COLOR.into();
            }

            Interaction::Hovered => {
                *color = BUTTON_HOVERED_COLOR.into();
            }

            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        }
    }
}