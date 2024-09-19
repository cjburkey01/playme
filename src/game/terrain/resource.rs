use super::TerrainTile;
use crate::game::{pos::TilePos, terrain::BOARD_AREA};
use bevy::prelude::*;

#[derive(Resource)]
pub struct TerrainBoard(Vec<TerrainTile>);

impl Default for TerrainBoard {
    fn default() -> Self {
        Self::new(default())
    }
}

impl TerrainBoard {
    pub fn new(tile: TerrainTile) -> Self {
        Self(vec![tile; BOARD_AREA as usize])
    }

    pub fn tile(&self, tile_pos: TilePos) -> TerrainTile {
        self.tiles()[tile_pos.tile_index()]
    }

    #[allow(unused)]
    pub fn tile_mut(&mut self, tile_pos: TilePos) -> &mut TerrainTile {
        &mut self.tiles_mut()[tile_pos.tile_index()]
    }

    pub fn tiles(&self) -> &[TerrainTile] {
        &self.0
    }

    #[allow(unused)]
    pub fn tiles_mut(&mut self) -> &mut [TerrainTile] {
        &mut self.0
    }
}
