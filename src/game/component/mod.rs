use bevy::prelude::*;

pub mod animation;

#[derive(Debug, Component, Copy, Clone)]
pub struct PlayCamSpeed(pub f32);

#[derive(Debug, Component, Copy, Clone)]
pub struct PlayerSpriteMarker;

#[derive(Debug, Component, Copy, Clone)]
pub struct BoardParentMarker;
