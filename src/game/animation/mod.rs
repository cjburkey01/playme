mod asset;
mod component;
mod system;

pub use asset::*;
pub use component::*;

use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                system::update_animation_manager_system,
                system::animate_sprite_system,
            )
                .chain(),
        );
    }
}
