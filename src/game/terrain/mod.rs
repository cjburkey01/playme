mod component;
mod resource;
mod system;
mod tile;

pub use component::*;
pub use resource::*;
pub use tile::*;

use super::MainGameState;
use bevy::prelude::*;

pub const BOARD_WIDTH: u32 = 25;
pub const BOARD_HEIGHT: u32 = 25;
pub const BOARD_AREA: u32 = BOARD_WIDTH * BOARD_HEIGHT;
pub const BOARD_SIZE: UVec2 = UVec2 {
    x: BOARD_WIDTH,
    y: BOARD_HEIGHT,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TerrainState>()
            .add_systems(
                OnEnter(MainGameState::InGame),
                system::start_loading_terrain_system,
            )
            .add_systems(
                Update,
                system::spawn_tile_map_system.run_if(resource_added::<TerrainBoard>),
            )
            .add_systems(
                OnExit(TerrainState::NotLoaded),
                system::despawn_tile_map_system,
            );
    }
}

#[derive(Debug, Default, States, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerrainState {
    #[default]
    NotLoaded,
    StartLoading,
    Ready,
}
