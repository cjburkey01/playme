pub mod asset;
pub mod board;
pub mod component;
pub mod pos;
pub mod system;
pub mod ui;

use bevy::prelude::*;

pub struct PlayMePlugin;

impl Plugin for PlayMePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((asset::PlayMeAssetPlugin, ui::PlayMeUiPlugin))
            .add_systems(PostStartup, system::spawn_player_camera_system);
    }
}
