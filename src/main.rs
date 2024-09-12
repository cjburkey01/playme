mod game;

use bevy::{asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use game::PlayMePlugin;
use wasm_bindgen::prelude::*;

fn main() {
    App::new()
        // Core plugins
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
        // HTML progress screen
        .add_systems(Update, hide_progress_screen.run_if(run_once()))
        // Start game!
        .run();
}

#[wasm_bindgen]
extern "C" {
    fn hide_progress_screen();
}
