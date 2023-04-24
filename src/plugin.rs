//! This module describes how the [`ChatPlugin`] is plugged into the Bevy engine.

use crate::builder::ChatPluginConfig;
use bevy::prelude::Plugin;

/// This struct is a plugin for Bevy engine.
///
/// It is used to configure the plugin and to add it to the Bevy app.
///
/// # Example
///
/// ```rust
/// use bevy_pn_chat::{ChatPlugin, Keyset};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let chat = ChatPlugin::builder()
///             .keyset(Keyset{
///                publish_key: "pub-c-...",
///                subscribe_key: "sub-c-..."
///             })
///             .username("John Doe")
///             .build()?;
/// # Ok(())}
/// ```
pub struct ChatPlugin {
    config: ChatPluginConfig,
}

impl From<ChatPluginConfig> for ChatPlugin {
    fn from(config: ChatPluginConfig) -> Self {
        Self { config }
    }
}

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}
