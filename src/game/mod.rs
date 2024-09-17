pub mod asset;
pub mod board;
pub mod component;
pub mod input;
pub mod picking;
pub mod pos;
pub mod state;
pub mod system;
pub mod ui;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use picking::MousePosInfo;
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
            .init_resource::<MousePosInfo>()
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
            .add_systems(
                FixedUpdate,
                system::integrate_player_pos_system.run_if(in_state(MainGameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    system::handle_player_input_system,
                    picking::update_mouse_pos_system,
                )
                    .run_if(in_state(MainGameState::InGame)),
            );
    }
}
