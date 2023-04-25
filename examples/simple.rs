use bevy::{
    prelude::{App, Camera2dBundle, ClearColor, Color, Commands},
    DefaultPlugins,
};
use bevy_pn_chat::{ChatPlugin, Keyset, TextStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let publish_key = std::env::var("PUBNUB_PUBLISH_KEY").expect("PUB_KEY env variable is not set");
    let subscribe_key =
        std::env::var("PUBNUB_SUBSCRIBE_KEY").expect("SUB_KEY env variable is not set");

    let chat = ChatPlugin::builder()
        .keyset(Keyset {
            publish_key,
            subscribe_key,
        })
        .username("John Doe")
        .input_style(TextStyle {
            font_path: "fonts/arial_unicode.ttf".into(),
            font_size: 20.0,
            color: Color::WHITE,
        })
        .message_style(TextStyle {
            font_path: "fonts/arial_unicode.ttf".into(),
            font_size: 20.0,
            color: Color::WHITE,
        })
        .max_messages(10)
        .build()?;

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(chat)
        .add_startup_system(camera)
        .run();

    Ok(())
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
