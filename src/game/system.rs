use super::component::PlayCamSpeed;
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn spawn_player_camera_system(mut commands: Commands) {
    commands.spawn((
        PlayCamSpeed(2.0),
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(10.0),
                ..default()
            },
            ..default()
        },
    ));
}
