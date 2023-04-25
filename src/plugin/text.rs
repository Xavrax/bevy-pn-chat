use bevy::{
    prelude::{Component, Resource, Transform},
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};

#[derive(Component, Default)]
pub struct InputBox {
    pub cursor: usize,
    pub selection: Option<usize>,
}

impl InputBox {
    pub fn new() -> Self {
        Self::default()
    }
}
