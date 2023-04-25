use std::future;

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{Commands, EventReader, KeyCode, Query, Res},
    tasks::AsyncComputeTaskPool,
    text::Text,
};

use crate::error;

use super::{
    resources::{ChannelResource, PubNubClientResource},
    tasks::PublishTask,
    text::InputBox,
};

pub fn keyboard_handler(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    mut input: Query<(&mut InputBox, &mut Text)>,
    pubnub: Res<PubNubClientResource>,
    channel: Res<ChannelResource>,
) {
    key_evr
        .iter()
        .filter(|key| key.state.is_pressed())
        .filter_map(|key| key.key_code)
        .for_each(|key| {
            match key {
                KeyCode::Return => {
                    let thread_pool = AsyncComputeTaskPool::get();
                    input.iter_mut().for_each(|mut input| {
                        let message = input.1.sections[0].value.clone();
                        input.1.sections[0].value.clear();
                        input.0.cursor = 0;
                        input.0.selection = None;

                        let pubnub = pubnub.clone();
                        let channel = channel.clone();
                        let task = thread_pool.spawn(async move {
                            pubnub
                                .publish_message(message)
                                .channel(channel)
                                .execute_blocking()
                                .map(|_| ())
                                .map_err(Into::into)
                        });

                        commands.spawn(PublishTask(task));
                    });
                    None
                }
                KeyCode::Back => {
                    input.iter_mut().for_each(|mut input| {
                        input.1.sections[0].value.pop();
                    });
                    None
                }
                _ => characters_filter(key),
            }
            .map(|character| {
                input.iter_mut().for_each(|mut input| {
                    input.1.sections[0].value.push(character);
                });
            });
        });
}

const SERIALIZED_LETTERS_POSITION: usize = 3;
const SERIALIZED_DIGITS_POSITION: usize = 4;
const SERIALIZED_NUMPAD_POSITION: usize = 7;

fn characters_filter(key_code: KeyCode) -> Option<char> {
    special_characters_filter(&key_code).or_else(|| {
        serde_json::to_string(&key_code)
            .ok()
            .and_then(|serialized| {
                letter_filter(&serialized).or_else(|| digits_filter(&serialized))
            })
    })
}

fn letter_filter(serialized: &String) -> Option<char> {
    (serialized.len() == SERIALIZED_LETTERS_POSITION)
        .then(|| serialized.chars().nth(1))
        .flatten()
}

fn digits_filter(serialized: &String) -> Option<char> {
    serialized
        .starts_with("\"Key")
        .then(|| serialized.chars().nth(SERIALIZED_DIGITS_POSITION))
        .flatten()
        .or_else(|| {
            serialized
                .starts_with("\"Numpad")
                .then(|| serialized.chars().nth(SERIALIZED_NUMPAD_POSITION))
                .flatten()
        })
}

fn special_characters_filter(key_code: &KeyCode) -> Option<char> {
    match key_code {
        KeyCode::Space => Some(' '),
        KeyCode::Comma => Some(','),
        KeyCode::Period => Some('.'),
        KeyCode::Slash => Some('/'),
        KeyCode::Semicolon => Some(';'),
        KeyCode::Apostrophe => Some('\''),
        KeyCode::Backslash => Some('\\'),
        KeyCode::LBracket => Some('['),
        KeyCode::RBracket => Some(']'),
        KeyCode::Grave => Some('`'),
        KeyCode::Minus => Some('-'),
        KeyCode::Equals => Some('='),
        _ => None,
    }
}

#[cfg(test)]
mod should {
    use super::*;

    use test_case::test_case;

    #[test_case(KeyCode::A => Some('A'))]
    #[test_case(KeyCode::B => Some('B'))]
    #[test_case(KeyCode::C => Some('C'))]
    #[test_case(KeyCode::F1 => None)]
    #[test_case(KeyCode::F2 => None)]
    #[test_case(KeyCode::Left => None)]
    #[test_case(KeyCode::Right => None)]
    #[test_case(KeyCode::Key1 => Some('1'))]
    #[test_case(KeyCode::Key2 => Some('2'))]
    #[test_case(KeyCode::Key3 => Some('3'))]
    #[test_case(KeyCode::Numpad1 => Some('1'))]
    #[test_case(KeyCode::Numpad2 => Some('2'))]
    #[test_case(KeyCode::Numpad3 => Some('3'))]
    #[test_case(KeyCode::Space => Some(' '))]
    #[test_case(KeyCode::Comma => Some(','))]
    #[test_case(KeyCode::Period => Some('.'))]
    #[test_case(KeyCode::Slash => Some('/'))]
    #[test_case(KeyCode::Semicolon => Some(';'))]
    #[test_case(KeyCode::Apostrophe => Some('\''))]
    #[test_case(KeyCode::Backslash => Some('\\'))]
    #[test_case(KeyCode::LBracket => Some('['))]
    #[test_case(KeyCode::RBracket => Some(']'))]
    #[test_case(KeyCode::Grave => Some('`'))]
    #[test_case(KeyCode::Minus => Some('-'))]
    #[test_case(KeyCode::Equals => Some('='))]
    fn filter_not_characters_codes(key_code: KeyCode) -> Option<char> {
        characters_filter(key_code)
    }
}
