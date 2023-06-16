use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::core::constants;

pub fn setup_plugins() -> PluginGroupBuilder {
    info!("Setting up plugins..");

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: [constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT].into(),
            title: constants::WINDOW_TITLE.to_string(),
            ..default()
        }),
        ..default()
    };

    DefaultPlugins.set(window_plugin)
}
