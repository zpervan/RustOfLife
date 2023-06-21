use bevy::{prelude::*, winit::WinitSettings};

use crate::core::*;
use crate::gui::*;

mod core;
mod gui;

fn main() {
    App::new()
        .add_plugins(bootstrap::setup_plugins())
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(startup_screen::initialize)
        .add_system(startup_screen::update)
        .run();
}
