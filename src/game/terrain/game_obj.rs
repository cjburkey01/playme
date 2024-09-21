use bevy::prelude::*;

#[derive(Debug, Component, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameObjectType {
    Tree(TreeVariant),
}

impl GameObjectType {
    pub fn object_atlas_index(&self) -> usize {
        match self {
            Self::Tree(tree) => tree.object_atlas_index(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TreeVariant {
    GreenTree,
}

impl TreeVariant {
    pub fn object_atlas_index(&self) -> usize {
        match self {
            Self::GreenTree => 0,
        }
    }
}
