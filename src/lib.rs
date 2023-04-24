//! # Bevy PubNub chat
//!
//! ## Overview
//!
//! This is a simple chat plugin for [Bevy](https://bevyengine.org/) that uses [PubNub infrastructure](https://www.pubnub.com/).
//! It is based on the [PubNub Rust SDK](https://www.github.com/pubnub/rust).
//!
//! ## Getting started
//!
//! Before you can use this plugin, you need to create a PubNub account and get your keys.
//! You can do that [here](https://dashboard.pubnub.com/signup).
//!
//! Once you have your keys, you can use them to use this plugin.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bevy = "0.10"
//! bevy_pn_chat = "0.1"
//! ```
//!
//! Then, add the following to your `main.rs`:
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_pn_chat::ChatPlugin;
//!
//! fn main() {
//!    let chat = ChatPlugin::builder()
//!                 .keyset(Keyset{
//!                     publish_key: "pub-c-...",
//!                     subscribe_key: "sub-c-..."
//!                 })
//!                 .build();
//!     
//!    App::new()
//!        .add_plugins(DefaultPlugins)
//!        .add_plugin(chat)
//!        .run();
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the [MIT license](LICENSE).
//!
//! ## Disclaimer
//!
//! This is not an official PubNub product.
//! I created this plugin for fun and to learn more about Bevy engine.
