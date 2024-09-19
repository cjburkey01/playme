use bevy::prelude::*;

#[derive(Debug, Component, Copy, Clone)]
pub struct PlyCamConfig(pub f32);

impl Default for PlyCamConfig {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Default, Component, Copy, Clone)]
pub struct PlyCamVelocity(pub Vec2);

#[derive(Debug, Default, Component, Copy, Clone)]
pub struct PlyCamInputAxisPair(pub Vec2);

#[derive(Debug, Default, Bundle, Clone)]
pub struct PlayerCameraBundle {
    pub cam_config: PlyCamConfig,
    pub cam_velocity: PlyCamVelocity,
    pub axis_pair: PlyCamInputAxisPair,
}

#[derive(Debug, Component, Copy, Clone)]
pub struct PlayerSpriteMarker;
