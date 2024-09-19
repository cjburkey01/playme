mod component;
mod resource;
mod system;

pub use component::*;
pub use resource::*;

use super::MainGameState;
use bevy::prelude::*;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosInfo>()
            .init_state::<MainGameState>()
            .add_systems(
                OnEnter(MainGameState::MainMenu),
                system::spawn_outline_entity_system,
            )
            .add_systems(
                Update,
                (
                    system::update_mouse_pos_system,
                    system::update_picker_entity_pos_system,
                )
                    .chain()
                    .run_if(in_state(MainGameState::InGame)),
            );
    }
}
