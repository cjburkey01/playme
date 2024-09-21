use crate::game::{
    animation::{SpriteAnimManager, SpriteAnimState},
    pos::WorldPos,
    terrain::TILE_WORLD_WIDTH,
};

use super::{InGameActions, PlayerSpriteMarker, PlyCamConfig, PlyCamInputAxisPair, PlyCamVelocity};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

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

    sprite_transform.translation.z =
        WorldPos(transform.translation.xy()).world_with_z().z + 5.0 * TILE_WORLD_WIDTH / 8.0 + 0.05;

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
