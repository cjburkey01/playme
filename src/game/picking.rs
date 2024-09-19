use super::{
    pos::{TilePos, WorldPos},
    MainGameState,
};
use bevy::prelude::*;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosInfo>()
            .init_state::<MainGameState>()
            .add_systems(
                Update,
                update_mouse_pos_system.run_if(in_state(MainGameState::InGame)),
            );
    }
}

#[derive(Debug, Default, Resource, Copy, Clone)]
pub struct MousePosInfo {
    #[allow(unused)]
    pub world: Vec2,
    #[allow(unused)]
    pub tile: IVec2,
    pub board_tile: Option<TilePos>,
}

pub fn update_mouse_pos_system(
    mut mouse_pos_res: ResMut<MousePosInfo>,
    player_camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
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

            gizmos.circle_2d(point, 6., Color::WHITE);
            if let Some(board_tile) = &mouse_pos_res.board_tile {
                gizmos.rect_2d(
                    WorldPos::from(*board_tile).0,
                    Rot2::IDENTITY,
                    Vec2::ONE,
                    Color::WHITE,
                )
            }
        }
    }
}
