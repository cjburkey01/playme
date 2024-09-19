use bevy::prelude::*;
use itertools::iproduct;

use crate::game::{
    board::{BOARD_HEIGHT, BOARD_WIDTH},
    pos::{TilePos, WorldPos},
};

use super::{
    asset::required::TileAssets,
    board::{TerrainBoard, TerrainTile},
    MainGameState,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TerrainState>()
            .add_systems(OnEnter(MainGameState::InGame), spawn_tile_map_system)
            .add_systems(OnExit(TerrainState::NotLoaded), despawn_tile_map_system);
    }
}

#[derive(Debug, Component, Copy, Clone)]
pub struct BoardParentMarker;

#[derive(Debug, Default, States, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerrainState {
    #[default]
    NotLoaded,
    Ready,
}

fn spawn_tile_map_system(
    mut commands: Commands,
    sprite_data: Res<TileAssets>,
    mut terrain_state: ResMut<NextState<TerrainState>>,
) {
    info!("spawning tile board");

    let tile_data = TerrainBoard::new(TerrainTile::Grass);
    terrain_state.set(TerrainState::Ready);

    commands
        .spawn((SpatialBundle::INHERITED_IDENTITY, BoardParentMarker))
        .with_children(|c| {
            for (y, x) in iproduct!(0u32..BOARD_HEIGHT, 0u32..BOARD_WIDTH) {
                let tile_pos = TilePos::new(UVec2::new(x, y)).expect("improper board tile pos???");
                let tile_type = tile_data.tile(tile_pos);
                let world_pos = WorldPos::from(tile_pos).0;
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
    commands.insert_resource(tile_data);
}

fn despawn_tile_map_system(
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
