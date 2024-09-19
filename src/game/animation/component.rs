use super::SpriteAnimation;
use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
pub struct SpriteAnimState {
    pub paused: bool,
    pub frame_num: usize,
    pub timer: Timer,
}

impl SpriteAnimState {
    pub fn new(fps: usize) -> Self {
        Self {
            paused: false,
            frame_num: 0,
            timer: Timer::from_seconds(1.0 / fps as f32, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct SpriteAnimManager {
    pub animations: Vec<Handle<SpriteAnimation>>,
    pub current: usize,
}

impl SpriteAnimManager {
    pub fn new(anims: impl IntoIterator<Item = Handle<SpriteAnimation>>) -> Self {
        Self {
            animations: anims.into_iter().collect(),
            current: 0,
        }
    }
}

// Add onto a SpriteBundle with a SpriteAtlas component
#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub state: SpriteAnimState,
    pub manager: SpriteAnimManager,
}
