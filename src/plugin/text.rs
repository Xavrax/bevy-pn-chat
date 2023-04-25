use bevy::{prelude::Component, text::Text};

#[derive(Component)]
pub struct InputBox {
    text: Text,
    cursor: usize,
    selection: Option<usize>,
}

//impl InputBox {
//    pub fn new() -> Self {
//        Self::default()
//    }
//}

//impl Default for InputBox {
//    fn default() -> Self {
//        Self {
//            text: Text::from_section(
//                "",
//                TextStyle {
//                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                    font_size: 40.0,
//                    color: Color::WHITE,
//                },
//            )
//            .with_alignment(TextAlignment {
//                vertical: VerticalAlign::Center,
//                horizontal: HorizontalAlign::Left,
//            }),
//            cursor: 0,
//            selection: None,
//        }
//    }
//}
