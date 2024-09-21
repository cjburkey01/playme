use crate::game::asset::required::UiAssets;

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
        .with_children(move |c| {
            init_loading_ui(c);
        });
}

pub(super) fn init_ui_with_assets(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    parent: Query<Entity, With<MainUiContainer>>,
) {
    let Ok(entity) = parent.get_single() else {
        return;
    };
    let ui_assets = ui_assets.clone();

    if let Some(mut entity) = commands.get_entity(entity) {
        entity.with_children(move |c| {
            init_main_menu_ui(c, ui_assets);
        });
    }
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

fn init_main_menu_ui(cmd: &mut ChildBuilder, ui_assets: UiAssets) {
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
                    padding: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: UiImage::new(ui_assets.ui_atlas.clone()),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(13.0),
                max_corner_scale: 4.0,
                ..default()
            }),
            TextureAtlas {
                index: 1,
                layout: ui_assets.atlas_layout.clone(),
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
