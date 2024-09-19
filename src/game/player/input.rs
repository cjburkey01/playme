use bevy::prelude::*;
use leafwing_input_manager::{
    plugin::InputManagerPlugin,
    prelude::{InputMap, KeyboardVirtualDPad},
    Actionlike,
};

pub struct GameActionsPlugin;

impl Plugin for GameActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InGameActions>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InGameActions {
    #[actionlike(DualAxis)]
    Move,
    Action,
}

impl InGameActions {
    pub fn make_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, KeyboardVirtualDPad::WASD)
            .with_dual_axis(Self::Move, KeyboardVirtualDPad::ARROW_KEYS)
    }
}
