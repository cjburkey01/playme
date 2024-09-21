use crate::game::animation::SpriteAnimation;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "textures/tile_atlas.png")]
    #[asset(image(sampler = nearest))]
    pub tile_atlas: Handle<Image>,

    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 16,
        columns = 30,
        rows = 59,
        padding_x = 2,
        padding_y = 2,
        offset_x = 1,
        offset_y = 1
    ))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct UiAssets {
    #[asset(path = "textures/ui_atlas.png")]
    #[asset(image(sampler = nearest))]
    pub ui_atlas: Handle<Image>,

    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 15,
        rows = 15,
        padding_x = 2,
        padding_y = 2,
        offset_x = 1,
        offset_y = 1
    ))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "textures/player_animation.png")]
    #[asset(image(sampler = nearest))]
    pub player_animation: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 8, rows = 16))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct ObjectAssets {
    #[asset(path = "textures/object_atlas.png")]
    #[asset(image(sampler = nearest))]
    pub object_atlas: Handle<Image>,

    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 15,
        rows = 15,
        padding_x = 2,
        padding_y = 2,
        offset_x = 1,
        offset_y = 1
    ))]
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct BuiltInAnimationAssets {
    pub player_walk_sw: Handle<SpriteAnimation>,
    pub player_walk_w: Handle<SpriteAnimation>,
    pub player_walk_nw: Handle<SpriteAnimation>,
    pub player_walk_n: Handle<SpriteAnimation>,
    pub player_walk_ne: Handle<SpriteAnimation>,
    pub player_walk_e: Handle<SpriteAnimation>,
    pub player_walk_se: Handle<SpriteAnimation>,
    pub player_walk_s: Handle<SpriteAnimation>,
}

pub(super) fn load_builtin_animations_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(BuiltInAnimationAssets {
        player_walk_sw: asset_server.add(SpriteAnimation::with_frames(0..=7)),
        player_walk_w: asset_server.add(SpriteAnimation::with_frames(8..=15)),
        player_walk_nw: asset_server.add(SpriteAnimation::with_frames(16..=23)),
        player_walk_n: asset_server.add(SpriteAnimation::with_frames(24..=31)),
        player_walk_ne: asset_server.add(SpriteAnimation::with_frames(32..=39)),
        player_walk_e: asset_server.add(SpriteAnimation::with_frames(40..=47)),
        player_walk_se: asset_server.add(SpriteAnimation::with_frames(48..=55)),
        player_walk_s: asset_server.add(SpriteAnimation::with_frames(56..=63)),
    });
}
