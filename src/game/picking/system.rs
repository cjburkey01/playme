use bevy::prelude::*;

use super::{MousePosInfo, PickerTileOutlineEntity};
use crate::game::{
    asset::required::UiAssets,
    pos::{TilePos, WorldPos},
};

pub(super) fn update_mouse_pos_system(
    mut mouse_pos_res: ResMut<MousePosInfo>,
    player_camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    if let (Ok((camera, pos)), Some(cursor_pos)) = (
        player_camera.get_single(),
        windows.get_single().ok().and_then(Window::cursor_position),
    ) {
        if let Some(point) = camera.viewport_to_world_2d(pos, cursor_pos) {
            *mouse_pos_res = MousePosInfo {
                world: point,
                tile: TilePos::unclamped_from_world_pos(point),
                board_tile: TilePos::try_from(WorldPos(point)).ok(),
            };
        }
    }
}

pub(super) fn spawn_outline_entity_system(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        PickerTileOutlineEntity,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * 2.0),
                ..default()
            },
            texture: ui_assets.ui_atlas.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 900.0),
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            layout: ui_assets.atlas_layout.clone(),
            index: 0,
        },
    ));
}

pub(super) fn update_picker_entity_pos_system(
    mouse_pos_res: Res<MousePosInfo>,
    mut tracker: Query<(&mut Transform, &mut Visibility), With<PickerTileOutlineEntity>>,
) {
    if let Ok((mut transform, mut visibility)) = tracker.get_single_mut() {
        *visibility = match mouse_pos_res.board_tile {
            Some(pos) => {
                transform.translation = WorldPos::from(pos).0.extend(transform.translation.z);
                default()
            }
            None => Visibility::Hidden,
        };
    }
}
