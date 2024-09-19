pub mod required;

use super::{animation::SpriteAnimation, ui::LoadingUiText, MainGameState};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use required::{PlayerAssets, TileAssets};

pub struct PlayMeAssetPlugin;

impl Plugin for PlayMeAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ProgressPlugin::new(MainGameState::Loading).continue_to(MainGameState::MainMenu),
        )
        .init_asset::<SpriteAnimation>()
        .add_loading_state(
            LoadingState::new(MainGameState::Loading)
                .load_collection::<TileAssets>()
                .load_collection::<PlayerAssets>(),
        )
        .add_systems(Startup, required::load_builtin_animations_system)
        .add_systems(
            Update,
            (
                fake_asset_hiccup_second.track_progress(),
                update_loading_progress_system,
            )
                .chain()
                .run_if(in_state(MainGameState::Loading))
                .after(LoadingStateSet(MainGameState::Loading)),
        );
    }
}

fn update_loading_progress_system(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
    mut loading_ui_text: Query<&mut Text, With<LoadingUiText>>,
) {
    let (Some(progress), Ok(mut text)) = (
        progress.map(|counter| counter.progress()),
        loading_ui_text.get_single_mut(),
    ) else {
        return;
    };

    if progress.done > *last_done {
        *last_done = progress.done;
        text.sections[1].value = progress.done.to_string();
        text.sections[3].value = progress.total.to_string();
        info!(
            "[Frame {}] Changed progress: {:?}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                .unwrap_or(0.0),
            progress
        );
    }
}

fn fake_asset_hiccup_second(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64() > 1.0 {
        info!("Long fake task is completed! Just making sure stuff is happening here");
        true.into()
    } else {
        false.into()
    }
}
