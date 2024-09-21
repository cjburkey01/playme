mod component;
mod game_obj;
mod resource;
mod system;
mod tile;
mod trigger;

pub use component::*;
pub use game_obj::*;
pub use resource::*;
pub use tile::*;
pub use trigger::*;

use super::{pos::TilePos, MainGameState};
use bevy::prelude::*;

pub const BOARD_WIDTH: u32 = 25;
pub const BOARD_HEIGHT: u32 = 25;
pub const BOARD_AREA: u32 = BOARD_WIDTH * BOARD_HEIGHT;
pub const BOARD_SIZE: UVec2 = UVec2::new(BOARD_WIDTH, BOARD_HEIGHT);
pub const TILE_WORLD_WIDTH: f32 = 2.0;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TerrainState>()
            .observe(trigger::observe_trigger_spawn_object_system)
            .add_systems(
                OnEnter(MainGameState::InGame),
                system::start_loading_terrain_system,
            )
            .add_systems(
                Update,
                (system::spawn_tile_map_system, spawn_tmp_object_system)
                    .chain()
                    .run_if(resource_added::<TerrainBoard>),
            )
            .add_systems(
                OnExit(TerrainState::NotLoaded),
                system::despawn_tile_map_system,
            );
    }
}

fn spawn_tmp_object_system(mut commands: Commands) {
    commands.trigger(TriggerSpawnGameObject {
        obj_type: GameObjectType::Tree(TreeVariant::GreenBirch),
        tile_pos: TilePos::new_clamped(IVec2::ONE),
    })
}

#[derive(Debug, Default, States, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerrainState {
    #[default]
    NotLoaded,
    StartLoading,
    Ready,
}
