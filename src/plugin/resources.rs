use std::ops::Deref;

use crate::TextStyle;
use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct InputBoxStyle(pub TextStyle);

impl Deref for InputBoxStyle {
    type Target = TextStyle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
