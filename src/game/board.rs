use bevy::prelude::*;

use super::pos::TilePos;

pub const BOARD_WIDTH: u32 = 25;
pub const BOARD_HEIGHT: u32 = 25;
pub const BOARD_AREA: u32 = BOARD_WIDTH * BOARD_HEIGHT;
pub const BOARD_SIZE: UVec2 = UVec2 {
    x: BOARD_WIDTH,
    y: BOARD_HEIGHT,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TerrainTile {
    #[default]
    Ocean,
    Grass,
}

impl TerrainTile {
    pub fn sprite_index(&self) -> usize {
        match self {
            TerrainTile::Ocean => 0,
            TerrainTile::Grass => 10,
        }
    }
}

#[derive(Resource)]
pub struct TerrainBoard(Vec<TerrainTile>);

impl Default for TerrainBoard {
    fn default() -> Self {
        Self(vec![TerrainTile::default(); BOARD_AREA as usize])
    }
}

impl TerrainBoard {
    pub fn tile(&self, tile_pos: TilePos) -> TerrainTile {
        self.tiles()[tile_pos.tile_index()]
    }

    pub fn tile_mut(&mut self, tile_pos: TilePos) -> &mut TerrainTile {
        &mut self.tiles_mut()[tile_pos.tile_index()]
    }

    pub fn tiles(&self) -> &[TerrainTile] {
        &self.0
    }

    pub fn tiles_mut(&mut self) -> &mut [TerrainTile] {
        &mut self.0
    }
}
