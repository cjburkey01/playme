use crate::game::pos::TilePos;
use bevy::prelude::*;

#[derive(Debug, Default, Resource, Copy, Clone)]
pub struct MousePosInfo {
    #[allow(unused)]
    pub world: Vec2,
    #[allow(unused)]
    pub tile: IVec2,
    pub board_tile: Option<TilePos>,
}
