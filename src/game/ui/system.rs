use bevy::prelude::*;

use crate::game::MainGameState;

use super::{
    BUTTON_BACK, BUTTON_BORDER, BUTTON_HOVER_BACK, BUTTON_HOVER_BORDER, BUTTON_PRESS_BACK,
    BUTTON_PRESS_BORDER,
};

pub(super) fn update_ui_display<Marker: Component + TypePath>(
    new_display: Display,
) -> impl FnMut(Query<&mut Style, With<Marker>>) {
    move |mut query| {
        info!(
            "updating display to {new_display:?} for {}",
            Marker::short_type_path()
        );
        if let Ok(mut style) = query.get_single_mut() {
            style.display = new_display;
        }
    }
}

pub(super) fn main_menu_play_button_press_system(
    mut next_game_state: ResMut<NextState<MainGameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESS_BACK.into();
                *border_color = BUTTON_PRESS_BORDER.into();
                next_game_state.set(MainGameState::InGame);
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER_BACK.into();
                *border_color = BUTTON_HOVER_BORDER.into();
            }
            Interaction::None => {
                *color = BUTTON_BACK.into();
                *border_color = BUTTON_BORDER.into();
            }
        }
    }
}
