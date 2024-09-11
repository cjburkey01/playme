use super::component::{LoadingUiParent, LoadingUiText};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

pub struct PlayMeAssetPlugin;

impl Plugin for PlayMeAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(AssetState::Loading).continue_to(AssetState::Finished))
            .init_state::<AssetState>()
            .add_loading_state(
                LoadingState::new(AssetState::Loading)
                    .load_collection::<TileAssets>()
                    .load_collection::<PlayerAssets>(),
            )
            .add_systems(
                Update,
                (fake_asset_hiccup_second.track_progress(), print_progress)
                    .chain()
                    .run_if(in_state(AssetState::Loading))
                    .after(LoadingStateSet(AssetState::Loading)),
            )
            .add_systems(OnEnter(AssetState::Finished), hide_loading_ui);
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetState {
    #[default]
    Loading,
    Finished,
}

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "textures/tile_atlas.png")]
    #[asset(image(sampler = nearest))]
    pub tile_atlas: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 16, columns = 30, rows = 59, padding_x = 2, padding_y = 2, offset_x = 1, offset_y = 1))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "textures/player_animation.png")]
    #[asset(image(sampler = nearest))]
    pub player_animation: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 8, rows = 16))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
    mut loading_ui_text: Query<&mut Text, With<LoadingUiText>>,
) {
    let Some(progress) = progress.map(|counter| counter.progress()) else {
        return;
    };
    let Ok(mut text) = loading_ui_text.get_single_mut() else {
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

fn hide_loading_ui(mut loading_ui: Query<&mut Style, With<LoadingUiParent>>) {
    for mut style in loading_ui.iter_mut() {
        style.display = Display::None;
    }
}
