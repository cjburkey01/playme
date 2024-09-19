use super::{
    full_size_node_style, LoadingUiParent, LoadingUiText, MainMenuPlayButton, MainMenuUiParent,
    MainUiContainer,
};
use bevy::{color::palettes::css::*, prelude::*};

pub(super) fn init_ui(mut commands: Commands) {
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
