use std::path::PathBuf;

use bevy::prelude::debug;

const FONT_ASSET_PATH: &str = "assets/fonts/";

fn get_root_path() -> PathBuf {
    let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root_path
}

pub fn get_font_path(font_type: String) -> String {
    let mut relative_font_path = String::from(FONT_ASSET_PATH);
    relative_font_path.push_str(font_type.as_str());

    let mut root_path = get_root_path();
    let mut full_path = root_path.parent().unwrap().join(relative_font_path);

    debug!("Font asset path: {}", full_path.display());

    full_path.to_str().unwrap().to_string()
}