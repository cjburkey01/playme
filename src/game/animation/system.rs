use super::{SpriteAnimManager, SpriteAnimState, SpriteAnimation};
use bevy::prelude::*;

pub(super) fn update_animation_manager_system(
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

pub(super) fn animate_sprite_system(
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
