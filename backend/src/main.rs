use bevy::prelude::*;

use crate::core::*;
use crate::gui::*;

mod core;
mod gui;

fn main() {
    App::new()
        .add_plugins(bootstrap::setup_plugins())
        .add_startup_system(startup_screen::initialize)
        .run();
}
