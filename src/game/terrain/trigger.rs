use super::{GameObjectType, TILE_WORLD_WIDTH};
use crate::game::{
    asset::required::ObjectAssets,
    pos::{TilePos, WorldPos},
};
use bevy::prelude::*;

#[derive(Debug, Event, Copy, Clone, PartialEq, Eq)]
pub struct TriggerSpawnGameObject {
    pub obj_type: GameObjectType,
    pub tile_pos: TilePos,
}

pub(super) fn observe_trigger_spawn_object_system(
    trigger: Trigger<TriggerSpawnGameObject>,
    mut commands: Commands,
    object_assets: Res<ObjectAssets>,
) {
    let TriggerSpawnGameObject { obj_type, tile_pos } = *trigger.event();

    // TODO:
    commands
        .spawn((
            SpatialBundle {
                transform: WorldPos::from(tile_pos).into(),
                ..default()
            },
            tile_pos,
            obj_type,
        ))
        .with_children(|c| {
            c.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(TILE_WORLD_WIDTH)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, TILE_WORLD_WIDTH / 4.0, 0.1),
                    texture: object_assets.object_atlas.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: object_assets.atlas_layout.clone(),
                    index: obj_type.object_atlas_index(),
                },
            ));
        });
}
