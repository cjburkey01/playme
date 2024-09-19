pub mod animation;
pub mod asset;
pub mod board;
pub mod input;
pub mod picking;
pub mod player;
pub mod pos;
pub mod terrain;
pub mod ui;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

pub struct PlayMePlugin;

impl Plugin for PlayMePlugin {
    fn build(&self, app: &mut App) {
        app // Add the game-related plugins
            .add_plugins((
                InputManagerPlugin::<input::InGameActions>::default(),
                asset::PlayMeAssetPlugin,
                ui::PlayMeUiPlugin,
                player::GamePlayerPlugin,
                terrain::TerrainPlugin,
                animation::SpriteAnimationPlugin,
                picking::PickingPlugin,
            ));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MainGameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}
