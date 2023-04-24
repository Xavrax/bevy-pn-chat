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
//! use bevy_pn_chat::{ChatPlugin, Keyset};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let chat = ChatPlugin::builder()
//!             .keyset(Keyset{
//!                publish_key: "pub-c-...",
//!                subscribe_key: "sub-c-..."
//!             })
//!             .username("John Doe")
//!             .build()?;
//! # Ok(())}
//! ```

use crate::error::{BevyPNError, Result};
use derive_builder::Builder;

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
#[derive(Debug, Clone, Builder)]
#[builder(
    pattern = "owned",
    build_fn(validate = "Self::validate", error = "BevyPNError")
)]
pub struct ChatPlugin {
    #[builder(setter(custom))]
    keyset: Keyset<String>,

    /// The channel to use.
    #[builder(setter(into), default = "\"bevy-pn-chat\".into()")]
    channel: String,

    /// The username to use.
    #[builder(setter(into), default = "\"anonymous\".into()")]
    username: String,
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
    pub fn builder() -> ChatPluginBuilder {
        ChatPluginBuilder::default()
    }
}

impl ChatPluginBuilder {
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

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn validate_if_keyset_is_empty() {
        let chat = ChatPlugin::builder()
            .keyset(Keyset {
                publish_key: "",
                subscribe_key: "",
            })
            .build();

        assert!(chat.is_err());
    }
}
