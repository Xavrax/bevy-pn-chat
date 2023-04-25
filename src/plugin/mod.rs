//! This module describes how the [`ChatPlugin`] is plugged into the Bevy engine.

use crate::{builder::ChatPluginConfig, BevyPNError};
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{EventReader, Plugin},
};
use pubnub::{
    transport::{middleware::PubNubMiddleware, TransportReqwest},
    Keyset, PubNubClient, PubNubClientBuilder,
};

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
        app.add_system(keyboard_handler);
    }
}

fn keyboard_handler(mut key_evr: EventReader<KeyboardInput>) {
    for key in key_evr.iter() {
        if key.state.is_pressed() {
            println!("{:#?}", key.key_code.unwrap());

            format!("{:#?}", key.key_code.unwrap());
        }
    }
}
