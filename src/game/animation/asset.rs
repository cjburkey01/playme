use bevy::prelude::*;

// Asset
// Because I wanna make overly-complicated
#[derive(Debug, Default, Asset, Clone, Reflect)]
pub struct SpriteAnimation {
    pub frames: Vec<usize>,
}

impl SpriteAnimation {
    #[allow(unused)]
    pub fn new(frames: Vec<usize>) -> Self {
        Self { frames }
    }

    pub fn with_frames(sprite_indices: impl IntoIterator<Item = usize>) -> Self {
        Self {
            frames: sprite_indices.into_iter().collect(),
        }
    }

    #[allow(unused)]
    pub fn add_frame(&mut self, sprite_index: usize) -> &mut Self {
        self.frames.push(sprite_index);
        self
    }

    #[allow(unused)]
    pub fn add_frames(&mut self, sprite_indices: impl IntoIterator<Item = usize>) -> &mut Self {
        for index in sprite_indices {
            self.frames.push(index);
        }
        self
    }
}
