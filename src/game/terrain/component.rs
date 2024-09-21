use crate::game::pos::TilePos;
use bevy::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
pub struct BoardParentMarker;

#[derive(Debug, Component, Copy, Clone)]
pub struct GameObjectPosition(pub TilePos);
