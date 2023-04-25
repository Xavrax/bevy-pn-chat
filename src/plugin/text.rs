use bevy::{prelude::Component, text::Text};

#[derive(Component)]
pub struct InputBox {
    text: Text,
    cursor: usize,
    selection: Option<usize>,
}
