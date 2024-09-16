use super::{
    asset::required::{BuiltInAnimationAssets, PlayerAssets, TileAssets},
    board::{TerrainBoard, BOARD_HEIGHT, BOARD_WIDTH},
    component::{
        animation::{AnimatedSpriteBundle, SpriteAnimManager, SpriteAnimState, SpriteAnimation},
        BoardParentMarker, PlayCamSpeed, PlayerSpriteMarker,
    },
    input::{make_game_action_input_map, InGameActions},
    pos::TilePos,
};
use bevy::{prelude::*, render::camera::ScalingMode};
use itertools::iproduct;
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

pub fn spawn_player_camera_system(mut commands: Commands) {
    commands.spawn((
        PlayCamSpeed(5.0),
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(20.0),
                ..Camera2dBundle::default().projection
            },
            ..default()
        },
        InputManagerBundle::with_map(make_game_action_input_map()),
    ));
}

pub fn add_player_sprite_system(
    mut commands: Commands,
    player: Query<Entity, With<PlayCamSpeed>>,
    sprite_data: Res<PlayerAssets>,
    sprite_anims: Res<BuiltInAnimationAssets>,
) {
    if let Some(mut ply_ent) = player
        .get_single()
        .ok()
        .and_then(|e| commands.get_entity(e))
    {
        ply_ent.with_children(|c| {
            c.spawn((
                PlayerSpriteMarker,
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        ..default()
                    },
                    texture: sprite_data.player_animation.clone(),
                    ..default()
                },
                TextureAtlas::from(sprite_data.atlas_layout.clone()),
                AnimatedSpriteBundle {
                    state: SpriteAnimState::new(20),
                    manager: SpriteAnimManager::new(
                        [
                            &sprite_anims.player_walk_sw,
                            &sprite_anims.player_walk_w,
                            &sprite_anims.player_walk_nw,
                            &sprite_anims.player_walk_n,
                            &sprite_anims.player_walk_ne,
                            &sprite_anims.player_walk_e,
                            &sprite_anims.player_walk_se,
                            &sprite_anims.player_walk_s,
                        ]
                        .into_iter()
                        .cloned(),
                    ),
                },
            ));
        });
    }
}

pub fn player_movement_handle_fixed_system(
    time: Res<Time>,
    mut player_entity: Query<
        (&mut Transform, &PlayCamSpeed, &ActionState<InGameActions>),
        Without<PlayerSpriteMarker>,
    >,
    mut sprite_entity: Query<
        (&mut Transform, &mut SpriteAnimManager, &mut SpriteAnimState),
        With<PlayerSpriteMarker>,
    >,
) {
    let (
        Ok((mut transform, PlayCamSpeed(speed), input)),
        Ok((mut sprite_transform, mut anim_manager, mut anim_state)),
    ) = (
        player_entity.get_single_mut(),
        sprite_entity.get_single_mut(),
    )
    else {
        return;
    };
    let speed = *speed;

    let axis_pair = input.axis_pair(&InGameActions::Move);
    let delta_pos =
        axis_pair.normalize_or_zero() * speed * Vec2::new(1.0, 1.0 / 2.0) * time.delta_seconds();

    transform.translation += delta_pos.extend(0.0);

    let ply_z = -transform.translation.y + 2.0;
    sprite_transform.translation.z = ply_z;

    anim_state.paused = axis_pair.length_squared() < 0.001;
    if !anim_state.paused {
        // 360º / 8 directions, 45º per animation
        let angle = axis_pair.angle_between(Vec2::Y).to_degrees() + 179.9;
        let anim_index = (angle.max(0.0).min(360.0) / 45.0) as usize;
        if anim_index < anim_manager.animations.len() {
            anim_manager.current = anim_index;
        }
    }
}

pub fn add_terrain_board_resource(mut commands: Commands) {
    commands.insert_resource(TerrainBoard::default());
}

pub fn spawn_tile_map_system(
    mut commands: Commands,
    tile_data: Res<TerrainBoard>,
    sprite_data: Res<TileAssets>,
) {
    info!("spawning tile board");

    commands
        .spawn((SpatialBundle::INHERITED_IDENTITY, BoardParentMarker))
        .with_children(|c| {
            for (y, x) in iproduct!(0u32..BOARD_HEIGHT, 0u32..BOARD_WIDTH) {
                let tile_pos = TilePos::new(UVec2::new(x, y)).expect("improper board tile pos???");
                let tile_type = tile_data.tile(tile_pos);
                let world_pos = tile_pos.as_world_pos();
                c.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(2.0, 1.0)),
                            ..default()
                        },
                        texture: sprite_data.tile_atlas.clone(),
                        transform: Transform::from_translation(world_pos.extend(-world_pos.y)),
                        ..default()
                    },
                    TextureAtlas {
                        layout: sprite_data.atlas_layout.clone(),
                        index: tile_type.sprite_index(),
                    },
                ));
            }
        });
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
