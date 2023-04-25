//! This module describes how the [`ChatPlugin`] is plugged into the Bevy engine.

use crate::{builder::ChatPluginConfig, BevyPNError};
use bevy::{
    prelude::{AssetServer, Commands, Plugin, Res, Transform},
    text::{Text2dBundle, TextStyle},
};
use keyboard::keyboard_handler;
use pubnub::{
    transport::{middleware::PubNubMiddleware, TransportReqwest},
    Keyset, PubNubClient, PubNubClientBuilder,
};

use self::{resources::InputBoxStyle, text::InputBox};

mod keyboard;
mod resources;
mod text;

/// This struct is a plugin for Bevy engine.
///
/// It is used to configure the plugin and to add it to the Bevy app.
///
/// # Example
///
/// ```rust no_run
/// use bevy::prelude::{App, DefaultPlugins};
/// use bevy_pn_chat::{ChatPlugin, Keyset};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let chat = ChatPlugin::builder()
///             .keyset(Keyset{
///                publish_key: "pub-c-...",
///                subscribe_key: "sub-c-..."
///             })
///             .username("John Doe")
///             .build()?;
///
///  App::new()
///      .add_plugins(DefaultPlugins)
///      .add_plugin(chat)
///      .run();
///
/// # Ok(())}
/// ```
pub struct ChatPlugin {
    // TODO: it has to be kept in memory because of lack of subscription implementation
    config: ChatPluginConfig,

    pubnub: PubNubClient<PubNubMiddleware<TransportReqwest>>,
}

impl TryFrom<ChatPluginConfig> for ChatPlugin {
    type Error = BevyPNError;

    fn try_from(config: ChatPluginConfig) -> Result<Self, Self::Error> {
        let pubnub = PubNubClientBuilder::with_reqwest_transport()
            .with_keyset(Keyset {
                subscribe_key: config.keyset.subscribe_key.clone(),
                publish_key: Some(config.keyset.publish_key.clone()),
                secret_key: None,
            })
            .with_user_id(config.username.clone())
            .build()
            .map_err(|error| BevyPNError::Config {
                message: error.to_string(),
            })?;

        Ok(Self { config, pubnub })
    }
}

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(InputBoxStyle(self.config.input_style.clone()))
            .add_startup_system(plugin_startup)
            .add_system(keyboard_handler);
    }
}

fn plugin_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    style: Res<InputBoxStyle>,
) {
    let font = asset_server.load(style.font_path.to_str().unwrap_or(""));

    commands.spawn((
        InputBox::default(),
        Text2dBundle {
            text: bevy::text::Text::from_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: style.font_size,
                    color: style.color,
                },
            )
            .with_alignment(bevy::text::TextAlignment::Left),
            transform: Transform::from_xyz(30.0, 30.0, 0.0),
            ..Default::default()
        },
    ));
}
