pub mod asset;
pub mod board;
pub mod component;
pub mod input;
pub mod pos;
pub mod state;
pub mod system;
pub mod ui;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use state::MainGameState;

pub struct PlayMePlugin;

impl Plugin for PlayMePlugin {
    fn build(&self, app: &mut App) {
        app // Add the game-related plugins
            .add_plugins((
                InputManagerPlugin::<input::InGameActions>::default(),
                asset::PlayMeAssetPlugin,
                ui::PlayMeUiPlugin,
            ))
            .init_state::<MainGameState>()
            // Spawn the player
            .add_systems(Startup, system::spawn_player_camera_system)
            .add_systems(
                OnEnter(MainGameState::InGame),
                (
                    system::add_terrain_board_resource,
                    system::add_player_sprite_system,
                ),
            )
            // Add the system(s) that run when the terrain tile map is added to
            // the world
            .add_systems(
                Update,
                system::spawn_tile_map_system.run_if(resource_added::<board::TerrainBoard>),
            )
            // Handle sprite frame animations
            .add_systems(
                Update,
                (
                    system::update_animation_manager_system,
                    system::animate_sprite_system,
                )
                    .chain(),
            )
            // Handle player movement when the tile board exists.
            // TODO: ADD BETTER GAME STATE HANDLER!
            .add_systems(
                Update,
                system::player_movement_handle_fixed_system
                    .run_if(in_state(MainGameState::InGame))
                    .run_if(resource_exists::<board::TerrainBoard>),
            );
    }
}
