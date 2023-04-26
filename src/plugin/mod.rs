//! This module describes how the [`ChatPlugin`] is plugged into the Bevy engine.

use crate::{builder::ChatPluginConfig, BevyPNError};
use bevy::{
    prelude::{AssetServer, Commands, Plugin, Res, Transform},
    text::{Text2dBundle, TextStyle},
};
use keyboard::keyboard_handler;
use pubnub::{
    transport::middleware::PubNubMiddleware, transport::reqwest::blocking::TransportReqwest,
    Keyset, PubNubClient, PubNubClientBuilder,
};

use self::{
    messages::message_handler,
    resources::{
        ChannelResource, ChatMessageStyle, InputBoxStyle, MessageFormat, PubNubClientResource,
        PubNubSubscribeResource,
    },
    tasks::tasks_handler,
    text::InputBox,
};

mod keyboard;
mod messages;
mod resources;
mod tasks;
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
        let pubnub = PubNubClientBuilder::with_reqwest_blocking_transport()
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
            .insert_resource(ChatMessageStyle(self.config.message_style.clone()))
            .insert_resource(PubNubClientResource(self.pubnub.clone()))
            .insert_resource(PubNubSubscribeResource {
                subscribe_key: self.config.keyset.subscribe_key.clone(),
                channel: self.config.channel.clone(),
                tt: "0".into(),
                tr: "0".into(),
                user_id: self.config.username.clone(),
            })
            .insert_resource(MessageFormat(self.config.message_format.clone()))
            .insert_resource(ChannelResource(self.config.channel.clone()))
            .add_startup_system(plugin_startup)
            .add_system(keyboard_handler)
            .add_system(tasks_handler)
            .add_startup_system(message_handler);
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
