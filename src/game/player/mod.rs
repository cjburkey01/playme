mod component;
mod input;
mod system;
mod trigger;

pub use component::*;
pub use input::*;

use super::MainGameState;
use bevy::prelude::*;

pub struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameActionsPlugin)
            .observe(trigger::on_spawn_player_observer)
            .add_systems(
                OnEnter(MainGameState::InGame),
                trigger::trigger_add_ply_sprite_system,
            )
            // Spawn the player
            .add_systems(Startup, |mut commands: Commands| {
                commands.trigger(trigger::SpawnPlayerTrigger)
            })
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
