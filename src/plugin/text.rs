use bevy::{
    prelude::{Component, Resource},
    text::{Text, TextAlignment, TextStyle},
};
#[derive(Component)]
pub struct InputBox {
    pub text: Text,
    pub cursor: usize,
    pub selection: Option<usize>,
}

impl InputBox {
    pub fn new(style: TextStyle) -> Self {
        Self {
            text: Text::from_section("", style).with_alignment(TextAlignment::Left),
            cursor: 0,
            selection: None,
        }
    }
}
