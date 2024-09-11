mod game;

use bevy::{asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use game::PlayMePlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn hide_progress_screen();
}

fn main() {
    hide_progress_screen();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#main-canvas".to_string()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_plugins(PlayMePlugin)
        .run();
}
