use bevy::{prelude::*, render::camera::ScalingMode};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};

use super::{
    animation::{AnimatedSpriteBundle, SpriteAnimManager, SpriteAnimState},
    asset::required::{BuiltInAnimationAssets, PlayerAssets},
    input::InGameActions,
    MainGameState,
};

pub struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.observe(on_spawn_player_observer)
            .add_systems(
                OnEnter(MainGameState::InGame),
                trigger_add_ply_sprite_system,
            )
            // Spawn the player
            .add_systems(Startup, trigger_spawn_ply_system)
            .add_systems(
                FixedUpdate,
                integrate_player_pos_system.run_if(in_state(MainGameState::InGame)),
            )
            .add_systems(
                Update,
                handle_player_input_system.run_if(in_state(MainGameState::InGame)),
            );
    }
}

#[derive(Debug, Component, Copy, Clone)]
pub struct PlyCamConfig(pub f32);

impl Default for PlyCamConfig {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Default, Component, Copy, Clone)]
pub struct PlyCamVelocity(pub Vec2);

#[derive(Debug, Default, Component, Copy, Clone)]
pub struct PlyCamInputAxisPair(pub Vec2);

#[derive(Debug, Default, Bundle, Clone)]
pub struct PlayerCameraBundle {
    pub cam_config: PlyCamConfig,
    pub cam_velocity: PlyCamVelocity,
    pub axis_pair: PlyCamInputAxisPair,
}

#[derive(Debug, Component, Copy, Clone)]
pub struct PlayerSpriteMarker;

pub fn handle_player_input_system(
    mut player_entity: Query<(
        &mut PlyCamVelocity,
        &mut PlyCamInputAxisPair,
        &PlyCamConfig,
        &ActionState<InGameActions>,
    )>,
) {
    if let Ok((mut velocity, mut input_pair, PlyCamConfig(speed), input)) =
        player_entity.get_single_mut()
    {
        input_pair.0 = input.axis_pair(&InGameActions::Move);
        velocity.0 = *speed * input_pair.0.normalize_or_zero() * Vec2::new(1.0, 1.0 / 2.0);
    }
}

pub fn integrate_player_pos_system(
    time: Res<Time<Fixed>>,
    mut player_entity: Query<
        (&mut Transform, &PlyCamVelocity, &PlyCamInputAxisPair),
        Without<PlayerSpriteMarker>,
    >,
    mut sprite_entity: Query<
        (&mut Transform, &mut SpriteAnimManager, &mut SpriteAnimState),
        With<PlayerSpriteMarker>,
    >,
) {
    let (
        Ok((mut transform, PlyCamVelocity(cam_velocity), PlyCamInputAxisPair(axis_pair))),
        Ok((mut sprite_transform, mut anim_manager, mut anim_state)),
    ) = (
        player_entity.get_single_mut(),
        sprite_entity.get_single_mut(),
    )
    else {
        return;
    };

    let delta_pos = *cam_velocity * time.delta_seconds();
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
