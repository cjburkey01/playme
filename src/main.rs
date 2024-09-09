mod game;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_wasm_window_resize::WindowResizePlugin;
use game::PlayMePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(WindowResizePlugin)
        .add_plugins(PlayMePlugin)
        .run();
}
