mod component;
mod input;
mod system;

pub use component::*;
pub use input::*;

use super::{
    animation::{AnimatedSpriteBundle, SpriteAnimManager, SpriteAnimState},
    asset::required::{BuiltInAnimationAssets, PlayerAssets},
    MainGameState,
};
use bevy::{prelude::*, render::camera::ScalingMode};
use leafwing_input_manager::InputManagerBundle;

pub struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameActionsPlugin)
            .observe(on_spawn_player_observer)
            .add_systems(
                OnEnter(MainGameState::InGame),
                trigger_add_ply_sprite_system,
            )
            // Spawn the player
            .add_systems(Startup, trigger_spawn_ply_system)
            .add_systems(
                FixedUpdate,
                system::integrate_player_pos_system.run_if(in_state(MainGameState::InGame)),
            )
            .add_systems(
                Update,
                system::handle_player_input_system.run_if(in_state(MainGameState::InGame)),
            );
    }
}

#[derive(Debug, Event)]
pub struct SpawnPlayerTrigger;

fn trigger_spawn_ply_system(world: &mut World) {
    world.trigger(SpawnPlayerTrigger);
}

fn on_spawn_player_observer(_trigger: Trigger<SpawnPlayerTrigger>, mut commands: Commands) {
    commands
        .spawn((
            PlayerCameraBundle {
                cam_config: PlyCamConfig(5.0),
                ..default()
            },
            Camera2dBundle {
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedVertical(20.0),
                    ..Camera2dBundle::default().projection
                },
                ..default()
            },
            InputManagerBundle::with_map(InGameActions::make_input_map()),
        ))
        .observe(on_add_player_sprite_observer);
}

#[derive(Debug, Event)]
pub struct AddPlayerSpriteTrigger;

fn trigger_add_ply_sprite_system(mut commands: Commands, plys: Query<Entity, With<PlyCamConfig>>) {
    for ply_entity in plys.iter() {
        commands.trigger_targets(AddPlayerSpriteTrigger, ply_entity);
    }
}

pub fn on_add_player_sprite_observer(
    trigger: Trigger<AddPlayerSpriteTrigger>,
    mut commands: Commands,
    sprite_data: Res<PlayerAssets>,
    sprite_anims: Res<BuiltInAnimationAssets>,
) {
    // Not triggered on the player entity
    if trigger.entity() == Entity::PLACEHOLDER {
        warn!("trigger entity was PLACEHOLDER!");
        return;
    }

    if let Some(mut ply_ent) = commands.get_entity(trigger.entity()) {
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
