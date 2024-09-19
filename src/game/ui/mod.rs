mod component;
mod init;
mod system;

pub use component::*;

use super::{terrain::TerrainState, MainGameState};
use bevy::prelude::*;

pub const BUTTON_BACK: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVER_BACK: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_PRESS_BACK: Color = Color::srgb(0.35, 0.75, 0.35);

pub const BUTTON_BORDER: Color = Color::srgb(0.05, 0.05, 0.05);
pub const BUTTON_HOVER_BORDER: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_PRESS_BORDER: Color = Color::srgb(0.25, 0.65, 0.25);

pub struct PlayMeUiPlugin;

impl Plugin for PlayMeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                init::init_ui,
                system::update_ui_display::<LoadingUiParent>(Display::Flex),
            )
                .chain(),
        )
        .add_systems(
            OnExit(MainGameState::Loading),
            system::update_ui_display::<LoadingUiParent>(Display::None),
        )
        .add_systems(
            OnEnter(MainGameState::MainMenu),
            system::update_ui_display::<MainMenuUiParent>(Display::Flex),
        )
        // EXIT CURRENTLY ONCE GENERATION IS DONE
        .add_systems(
            OnEnter(TerrainState::Ready),
            system::update_ui_display::<MainMenuUiParent>(Display::None),
        )
        .add_systems(Update, system::main_menu_play_button_press_system);
    }
}

pub fn full_size_node_style() -> Style {
    Style {
        display: Display::None,
        position_type: PositionType::Absolute,
        top: Val::ZERO,
        left: Val::ZERO,
        right: Val::ZERO,
        bottom: Val::ZERO,
        ..default()
    }
}
