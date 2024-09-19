use bevy::prelude::*;

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
