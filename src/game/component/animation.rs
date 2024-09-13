use bevy::prelude::*;

// Asset
// Because I wanna make overly-complicated
#[derive(Debug, Default, Asset, Clone, Reflect)]
pub struct SpriteAnimation {
    pub frames: Vec<usize>,
}

impl SpriteAnimation {
    pub fn new(frames: Vec<usize>) -> Self {
        Self { frames }
    }

    pub fn with_frames(sprite_indices: impl IntoIterator<Item = usize>) -> Self {
        Self {
            frames: sprite_indices.into_iter().collect(),
        }
    }

    pub fn add_frame(&mut self, sprite_index: usize) -> &mut Self {
        self.frames.push(sprite_index);
        self
    }

    pub fn add_frames(&mut self, sprite_indices: impl IntoIterator<Item = usize>) -> &mut Self {
        for index in sprite_indices {
            self.frames.push(index);
        }
        self
    }
}

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
