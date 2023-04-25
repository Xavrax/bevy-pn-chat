//! This module contains the `ChatPlugin` struct and its builder.
//!
//! The [`ChatPlugin`] is the main struct of this crate.
//! It is used to configure the plugin and to add it to the Bevy app.
//!
//! The [`ChatPluginBuilder`] is used to configure the plugin.
//! It is created using the [`ChatPlugin::builder()`] method.
//!
//! # Example
//!
//! ```rust
//! use bevy_pn_chat::{ChatPlugin, Keyset, TextStyle, Color};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let chat = ChatPlugin::builder()
//!             .keyset(Keyset{
//!                publish_key: "pub-c-...",
//!                subscribe_key: "sub-c-..."
//!             })
//!             .username("John Doe")
//!             .channel("my-channel")
//!             .input_style(TextStyle {
//!                 font_path: "fonts/FiraSans-Bold.ttf".into(),
//!                 font_size: 20.0,
//!                 color: Color::WHITE,
//!             })
//!             .message_style(TextStyle {
//!                 font_path: "fonts/FiraSans-Bold.ttf".into(),
//!                 font_size: 20.0,
//!                 color: Color::WHITE,
//!             })
//!             .max_messages(10)
//!             .build()?;
//! # Ok(())}
//! ```

use std::path::PathBuf;

use crate::{
    error::{BevyPNError, Result},
    ChatPlugin,
};
use bevy::prelude::Color;
use derive_builder::Builder;

/// This struct is a config for [`ChatPlugin`].
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
///
/// More examples can be found in the [`plugin`] module documentation
/// or in the [examples](https://github.com/xavrax/bevy_pn_chat.git) directory.
#[derive(Debug, Clone, Builder)]
#[builder(
    pattern = "owned",
    build_fn(
        validate = "Self::validate",
        error = "BevyPNError",
        vis = "",
        name = "internal_build"
    )
)]
pub struct ChatPluginConfig {
    #[builder(setter(custom))]
    pub(crate) keyset: Keyset<String>,

    /// The channel to use.
    #[builder(setter(into), default = "\"bevy-pn-chat\".into()")]
    pub(crate) channel: String,

    /// The username to use.
    #[builder(setter(into), default = "\"anonymous\".into()")]
    pub(crate) username: String,

    /// The maximum number of messages to display.
    /// If the number of messages exceeds this value, the oldest messages will be removed.
    /// If set to `None`, the number of messages is unlimited.
    /// Defaults to `None`.
    ///
    /// # Warning
    ///
    /// If you set this value to `None`, the memory usage will increase over time.
    #[builder(setter(strip_option), default)]
    pub(crate) max_messages: Option<usize>,

    /// Text style for the input box.
    /// Defaults to `TextStyle::default()`.
    ///
    /// It wraps directly into a [`TextStyle`].
    /// See bevy [`TextStyle`] for more information.
    ///
    /// [`TextStyle`]: https://docs.rs/bevy/0.5.0/bevy/text/struct.TextStyle.html
    #[builder(default)]
    pub(crate) input_style: TextStyle,

    /// Text style for the messages.
    /// Defaults to `TextStyle::default()`.
    ///
    /// It wraps directly into a [`TextStyle`].
    /// See bevy [`TextStyle`] for more information.
    ///
    /// [`TextStyle`]: https://docs.rs/bevy/0.5.0/bevy/text/struct.TextStyle.html
    #[builder(default)]
    pub(crate) message_style: TextStyle,

    /// Message format.
    /// Defaults to `"{username}: {message}"`.
    /// The following placeholders are available:
    /// - `{username}`: the username of the sender
    /// - `{message}`: the message
    /// - `{time}`: the time the message was sent
    /// - `{date}`: the date the message was sent
    /// - `{datetime}`: the date and time the message was sent
    /// - `{timestamp}`: the timestamp the message was sent
    /// - `{channel}`: the channel the message was sent to
    #[builder(setter(into), default = "\"{username}: {message}\".into()")]
    pub(crate) message_format: String,
}

impl ChatPluginConfigBuilder {
    /// Builds the [`ChatPluginConfig`] and returns a [`ChatPlugin`].
    ///
    /// # Errors
    ///
    /// This method returns an error if the configuration is invalid.
    pub fn build(self) -> Result<ChatPlugin> {
        ChatPlugin::try_from(self.internal_build()?)
    }

    /// The keyset used to connect to PubNub.
    pub fn keyset<T>(mut self, keyset: Keyset<T>) -> Self
    where
        T: Into<String>,
    {
        self.keyset = Some(Keyset {
            publish_key: keyset.publish_key.into(),
            subscribe_key: keyset.subscribe_key.into(),
        });

        self
    }

    fn validate(&self) -> Result<()> {
        self.keyset
            .as_ref()
            .and_then(|keyset| {
                (keyset.publish_key.is_empty() || keyset.subscribe_key.is_empty()).then(|| {
                    Err(BevyPNError::Config {
                        message: "Keyset is empty".into(),
                    })
                })
            })
            .unwrap_or(Ok(()))?;

        self.channel
            .as_ref()
            .and_then(|channel| {
                channel.is_empty().then(|| {
                    Err(BevyPNError::Config {
                        message: "Channel is empty".into(),
                    })
                })
            })
            .unwrap_or(Ok(()))?;

        self.username
            .as_ref()
            .and_then(|username| {
                username.is_empty().then(|| {
                    Err(BevyPNError::Config {
                        message: "Username is empty".into(),
                    })
                })
            })
            .unwrap_or(Ok(()))?;

        self.message_format
            .as_ref()
            .and_then(|message_format| {
                message_format.is_empty().then(|| {
                    Err(BevyPNError::Config {
                        message: "Message format is empty".into(),
                    })
                })
            })
            .unwrap_or(Ok(()))?;

        Ok(())
    }
}

/// This struct is used to configure the [`ChatPlugin`].
///
/// It provides methods to set the keyset for the PubNub infrastructure.
///
/// # Example
///
/// ```rust
/// use bevy_pn_chat::Keyset;
///
/// Keyset {
///    publish_key: "pub-c-...",
///    subscribe_key: "sub-c-..."
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Keyset<S>
where
    S: Into<String>,
{
    /// The publish key for the PubNub infrastructure.
    pub publish_key: S,

    /// The subscribe key for the PubNub infrastructure.
    pub subscribe_key: S,
}

/// This struct is used to configure the text style for the [`ChatPlugin`].
/// It wraps directly into a [`TextStyle`].
///
/// See bevy [`TextStyle`] for more information.
/// [`TextStyle`]: https://docs.rs/bevy/0.5.0/bevy/text/struct.TextStyle.html
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// The font path to use.
    ///
    /// This is a path to a font file.
    /// It uses Bevy's asset management system to load the font.
    ///
    /// Defaults to an empty path.
    ///
    /// # Warning
    ///
    /// If not path provided, then your messages will not be displayed.
    pub font_path: PathBuf,

    /// The font size to use.
    pub font_size: f32,

    /// The color to use.
    ///
    /// It uses Bevy's [`Color`] struct.
    ///
    /// [`Color`]: https://docs.rs/bevy/0.5.0/bevy/color/struct.Color.html
    pub color: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_path: "".into(),
            font_size: 20.0,
            color: Color::WHITE,
        }
    }
}

impl ChatPlugin {
    /// Creates a new [`ChatPluginBuilder`].
    ///
    /// This is used to configure the plugin.
    /// The [`ChatPluginBuilder`] is then used to create the [`ChatPlugin`].
    ///
    /// # Example
    /// ```rust
    /// use bevy_pn_chat::{ChatPlugin, Keyset};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let chat = ChatPlugin::builder()
    ///           .keyset(Keyset{
    ///               publish_key: "pub-c-...",
    ///               subscribe_key: "sub-c-..."
    ///           })
    ///           .username("John Doe")
    ///           .build()?;
    /// # Ok(())}
    /// ```
    pub fn builder() -> ChatPluginConfigBuilder {
        ChatPluginConfigBuilder::default()
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn validate_if_keyset_is_empty() {
        let chat = ChatPluginConfigBuilder::default()
            .keyset(Keyset {
                publish_key: "",
                subscribe_key: "",
            })
            .internal_build();

        assert!(chat.is_err());
    }

    #[test]
    fn validate_if_channel_is_empty() {
        let chat = ChatPluginConfigBuilder::default()
            .channel("")
            .internal_build();

        assert!(chat.is_err());
    }

    #[test]
    fn validate_if_username_is_empty() {
        let chat = ChatPluginConfigBuilder::default()
            .username("")
            .internal_build();

        assert!(chat.is_err());
    }

    #[test]
    fn validate_if_message_format_is_empty() {
        let chat = ChatPluginConfigBuilder::default()
            .message_format("")
            .internal_build();

        assert!(chat.is_err());
    }
}
