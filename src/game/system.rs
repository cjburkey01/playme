use super::{
    asset::{PlayerAssets, TileAssets},
    board::{TerrainBoard, BOARD_HEIGHT, BOARD_WIDTH},
    component::{BoardParentMarker, PlayCamSpeed, PlayerSpriteMarker},
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
                TextureAtlas {
                    layout: sprite_data.atlas_layout.clone(),
                    index: 1,
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
    mut sprite_entity: Query<&mut Transform, With<PlayerSpriteMarker>>,
) {
    let Ok((mut transform, PlayCamSpeed(speed), input)) = player_entity.get_single_mut() else {
        return;
    };
    let Ok(mut sprite_transform) = sprite_entity.get_single_mut() else {
        return;
    };
    let speed = *speed;

    let axis_pair = input.axis_pair(&InGameActions::Move);
    let delta_pos =
        axis_pair.normalize_or_zero() * speed * Vec2::new(1.0, 1.0 / 2.0) * time.delta_seconds();

    transform.translation += delta_pos.extend(0.0);

    let ply_z = -transform.translation.y + 2.0;
    sprite_transform.translation.z = ply_z;
}

pub fn on_exit_asset_load_state_system(mut commands: Commands) {
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
