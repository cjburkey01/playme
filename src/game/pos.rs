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

    pub fn unclamped_from_world_pos(world_pos: Vec2) -> IVec2 {
        let world_pos = world_pos * 2.0 - Vec2::new(0.5, 0.5);
        let tile_x = (world_pos.x - 2.0 * world_pos.y) / 4.0;
        let tile_y = -tile_x - world_pos.y;
        Vec2::new(tile_x, tile_y).floor().as_ivec2()
    }

    #[allow(unused)]
    pub fn new_clamped(pos: IVec2) -> Self {
        Self(pos.clamp(IVec2::ZERO, BOARD_SIZE.as_ivec2()).as_uvec2())
    }

    #[allow(unused)]
    pub fn uvec(&self) -> UVec2 {
        self.0
    }

    #[allow(unused)]
    pub fn clamp_add(&self, offset: IVec2) -> Self {
        Self::new_clamped(self.0.as_ivec2() + offset)
    }

    #[allow(unused)]
    pub fn try_add(&self, offset: IVec2) -> Option<Self> {
        let pos = self.0.as_ivec2() + offset;
        let size = BOARD_SIZE.as_ivec2();
        if pos.x >= 0 && pos.y >= 0 && pos.x < size.x && pos.y < size.y {
            Some(TilePos(pos.as_uvec2()))
        } else {
            None
        }
    }

    pub fn tile_index(&self) -> usize {
        let UVec2 { x, y } = self.0;
        (BOARD_WIDTH * y + x) as usize
    }
}

impl TryFrom<WorldPos> for TilePos {
    type Error = ();

    fn try_from(WorldPos(world_pos): WorldPos) -> Result<Self, Self::Error> {
        let tile_pos = Self::unclamped_from_world_pos(world_pos);
        if tile_pos.x >= 0 && tile_pos.y >= 0 {
            let tile_pos = tile_pos.as_uvec2();
            TilePos::new(tile_pos).ok_or(())
        } else {
            Err(())
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct WorldPos(pub Vec2);

impl From<TilePos> for WorldPos {
    fn from(value: TilePos) -> Self {
        let pos = value.0.as_ivec2();
        // See `playme/help/isogrid.svg`
        Self(IVec2::new(2 * (pos.x - pos.y), -pos.x - pos.y).as_vec2() / 2.0)
    }
}
