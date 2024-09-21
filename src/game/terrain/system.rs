use super::TerrainState;
use crate::game::{
    asset::required::TileAssets,
    pos::{TilePos, WorldPos},
    terrain::{
        BoardParentMarker, TerrainBoard, TerrainTile, BOARD_HEIGHT, BOARD_WIDTH, TILE_WORLD_WIDTH,
    },
};
use bevy::prelude::*;

pub(super) fn start_loading_terrain_system(mut commands: Commands) {
    info!("create tile board");

    commands.insert_resource(NextState::Pending(TerrainState::StartLoading));
    commands.insert_resource(TerrainBoard::new(TerrainTile::Grass));
}

pub(super) fn spawn_tile_map_system(
    mut commands: Commands,
    sprite_data: Res<TileAssets>,
    tile_data: Res<TerrainBoard>,
    mut terrain_state: ResMut<NextState<TerrainState>>,
) {
    info!("spawning terrain tile board");

    terrain_state.set(TerrainState::Ready);

    commands
        .spawn((SpatialBundle::INHERITED_IDENTITY, BoardParentMarker))
        .with_children(|c| {
            for (y, x) in itertools::iproduct!(0u32..BOARD_HEIGHT, 0u32..BOARD_WIDTH) {
                let tile_pos = TilePos::new(UVec2::new(x, y)).expect("improper board tile pos???");
                let tile_type = tile_data.tile(tile_pos);
                c.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_WORLD_WIDTH, TILE_WORLD_WIDTH / 2.0)),
                            ..default()
                        },
                        texture: sprite_data.tile_atlas.clone(),
                        transform: WorldPos::from(tile_pos).into(),
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

pub(super) fn despawn_tile_map_system(
    mut commands: Commands,
    board_query: Query<Entity, With<BoardParentMarker>>,
) {
    for board_entity in board_query.iter() {
        if let Some(mut entity) = commands.get_entity(board_entity) {
            entity.despawn();
        }
    }
    commands.remove_resource::<TerrainBoard>();
}
