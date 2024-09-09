use super::component::{LoadingUiParent, LoadingUiText};
use bevy::{
    color::palettes::css::{CORNFLOWER_BLUE, WHITE, YELLOW},
    prelude::*,
};

pub struct PlayMeUiPlugin;

impl Plugin for PlayMeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_ui);
    }
}

fn init_ui(mut commands: Commands) {
    commands
        .spawn((
            LoadingUiParent,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::ZERO,
                    left: Val::ZERO,
                    right: Val::ZERO,
                    bottom: Val::ZERO,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Stretch,
                    ..default()
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
