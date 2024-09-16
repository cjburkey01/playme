use super::state::MainGameState;
use bevy::{
    color::palettes::css::{CORNFLOWER_BLUE, WHITE, YELLOW},
    prelude::*,
    reflect::TypePath,
};

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
            (init_ui, update_ui_display::<LoadingUiParent>(Display::Flex)).chain(),
        )
        .add_systems(
            OnExit(MainGameState::Loading),
            update_ui_display::<LoadingUiParent>(Display::None),
        )
        .add_systems(
            OnEnter(MainGameState::MainMenu),
            update_ui_display::<MainMenuUiParent>(Display::Flex),
        )
        .add_systems(
            OnExit(MainGameState::MainMenu),
            update_ui_display::<MainMenuUiParent>(Display::None),
        )
        .add_systems(Update, main_menu_play_button_press_system);
    }
}

#[derive(Debug, Component, Copy, Clone)]
pub struct MainUiContainer;

#[derive(Debug, Component, Copy, Clone, Reflect)]
pub struct LoadingUiParent;

#[derive(Debug, Component, Copy, Clone)]
pub struct LoadingUiText;

#[derive(Debug, Component, Copy, Clone, Reflect)]
pub struct MainMenuUiParent;

#[derive(Debug, Component, Copy, Clone)]
pub struct MainMenuPlayButton;

fn update_ui_display<Marker: Component + TypePath>(
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

fn main_menu_play_button_press_system(
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

fn init_ui(mut commands: Commands) {
    commands
        .spawn((
            MainUiContainer,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    ..full_size_node_style()
                },
                ..default()
            },
        ))
        .with_children(|c| {
            init_loading_ui(c);
            init_main_menu_ui(c);
        });
}

fn init_loading_ui(cmd: &mut ChildBuilder) {
    cmd.spawn((
        LoadingUiParent,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                ..full_size_node_style()
            },
            ..default()
        },
    ))
    .with_children(|c| {
        c.spawn(
            TextBundle::from_sections([TextSection::new(
                "Please Wait!",
                TextStyle {
                    font_size: 40.0,
                    color: WHITE.into(),
                    ..default()
                },
            )])
            .with_text_justify(JustifyText::Center),
        );

        c.spawn((
            LoadingUiText,
            TextBundle::from_sections([
                TextSection::new(
                    "Loaded ",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 20.0,
                        color: YELLOW.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    " of ",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 20.0,
                        color: CORNFLOWER_BLUE.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    " assets :)",
                    TextStyle {
                        font_size: 20.0,
                        color: WHITE.into(),
                        ..default()
                    },
                ),
            ])
            .with_text_justify(JustifyText::Center),
        ));
    });
}

fn init_main_menu_ui(cmd: &mut ChildBuilder) {
    cmd.spawn((
        MainMenuUiParent,
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                ..full_size_node_style()
            },
            ..default()
        },
    ))
    .with_children(|c| {
        c.spawn(
            TextBundle::from_sections([TextSection::new(
                "Welcome!",
                TextStyle {
                    font_size: 40.0,
                    color: WHITE.into(),
                    ..default()
                },
            )])
            .with_text_justify(JustifyText::Center),
        );

        c.spawn((
            MainMenuPlayButton,
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn(TextBundle::from_section(
                "Play!",
                TextStyle {
                    font_size: 26.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

fn full_size_node_style() -> Style {
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
