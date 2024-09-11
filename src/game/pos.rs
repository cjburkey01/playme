use super::board::{BOARD_HEIGHT, BOARD_SIZE, BOARD_WIDTH};
use bevy::prelude::*;

// Position of a tile on the game board.
#[derive(Default, Debug, Component, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TilePos(UVec2);

impl TilePos {
    pub fn new(pos: UVec2) -> Option<Self> {
        if pos.x < BOARD_WIDTH && pos.y < BOARD_HEIGHT {
            Some(Self(pos))
        } else {
            None
        }
    }

    pub fn new_clamped(pos: IVec2) -> Self {
        Self(pos.clamp(IVec2::ZERO, BOARD_SIZE.as_ivec2()).as_uvec2())
    }

    pub fn uvec(&self) -> UVec2 {
        self.0
    }

    pub fn clamp_add(&self, offset: IVec2) -> Self {
        Self::new_clamped(self.0.as_ivec2() + offset)
    }

    pub fn try_add(&self, offset: IVec2) -> Option<Self> {
        let pos = self.0.as_ivec2() + offset;
        let size = BOARD_SIZE.as_ivec2();
        if pos.x >= 0 && pos.y >= 0 && pos.x < size.x && pos.y < size.y {
            Some(TilePos(pos.as_uvec2()))
        } else {
            None
        }
    }

    pub fn as_world_pos(&self) -> Vec2 {
        let pos = self.0.as_ivec2();
        // See `playme/help/isogrid.svg`
        IVec2::new(2 * (pos.x - pos.y), -pos.x - pos.y).as_vec2() / 2.0
    }

    pub fn tile_index(&self) -> usize {
        let UVec2 { x, y } = self.0;
        (BOARD_WIDTH * y + x) as usize
    }
}
