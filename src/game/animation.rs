use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_animation_manager_system, animate_sprite_system).chain(),
        );
    }
}

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

pub fn update_animation_manager_system(
    mut commands: Commands,
    mut managers: Query<(
        Entity,
        Option<&mut Handle<SpriteAnimation>>,
        &SpriteAnimManager,
    )>,
) {
    for (entity, current_anim, manager) in managers.iter_mut() {
        if manager.current < manager.animations.len() {
            let anim_in_manager = &manager.animations[manager.current];

            match current_anim {
                Some(mut current_anim) => {
                    if anim_in_manager != current_anim.as_ref() {
                        *current_anim = anim_in_manager.clone();
                    }
                }
                None => {
                    if let Some(mut entity) = commands.get_entity(entity) {
                        entity.insert(anim_in_manager.clone());
                    }
                }
            }
        }
    }
}

pub fn animate_sprite_system(
    time: Res<Time>,
    atlas_info: Res<Assets<TextureAtlasLayout>>,
    anim_infos: Res<Assets<SpriteAnimation>>,
    mut sprites: Query<(
        Ref<Handle<SpriteAnimation>>,
        &mut SpriteAnimState,
        &mut TextureAtlas,
    )>,
) {
    for (anim_info, mut anim_state, mut atlas) in sprites.iter_mut() {
        if anim_state.paused {
            continue;
        }
        let timer = anim_state.timer.tick(time.delta());
        // Check animation when animation handle changes regardless of timer
        if anim_info.is_changed() || timer.just_finished() {
            let (Some(atlas_info), Some(anim_info)) = (
                atlas_info.get(&atlas.layout),
                anim_infos.get(anim_info.into_inner()),
            ) else {
                continue;
            };

            let frame_num = (anim_state.frame_num + 1) % anim_info.frames.len();
            let sprite_index = anim_info.frames[frame_num].min(atlas_info.textures.len());

            anim_state.frame_num = frame_num;
            atlas.index = sprite_index;
        }
    }
}
