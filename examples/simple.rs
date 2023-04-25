use bevy::{prelude::App, DefaultPlugins};
use bevy_pn_chat::{ChatPlugin, Keyset};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat = ChatPlugin::builder()
        .keyset(Keyset {
            publish_key: "pub-c-...",
            subscribe_key: "sub-c-...",
        })
        .username("John Doe")
        .build()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(chat)
        .run();

    Ok(())
}
