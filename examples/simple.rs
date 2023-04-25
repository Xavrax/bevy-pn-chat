use bevy::{
    prelude::{App, Color},
    DefaultPlugins,
};
use bevy_pn_chat::{ChatPlugin, Keyset, TextStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat = ChatPlugin::builder()
        .keyset(Keyset {
            publish_key: "pub-c-...",
            subscribe_key: "sub-c-...",
        })
        .username("John Doe")
        .input_style(TextStyle {
            font_path: "fonts/FiraSans-Bold.ttf".into(),
            font_size: 20.0,
            color: Color::WHITE,
        })
        .message_style(TextStyle {
            font_path: "fonts/FiraSans-Bold.ttf".into(),
            font_size: 20.0,
            color: Color::WHITE,
        })
        .max_messages(10)
        .build()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(chat)
        .run();

    Ok(())
}
