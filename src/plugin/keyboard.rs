use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{EventReader, KeyCode, Resource},
};

#[derive(Resource)]
struct KeyCodeBuffer {
    key_codes: Vec<KeyCode>,
}

pub fn keyboard_handler(mut key_evr: EventReader<KeyboardInput>) {
    key_evr
        .iter()
        .filter(|key| key.state.is_pressed())
        .filter_map(|key| key.key_code)
        .filter_map(characters_filter)
        .for_each(|key| {
            println!("{:#?}", key);

            format!("{:#?}", key);
        });
}

const SERIALIZED_LETTERS_POSITION: usize = 3;
const SERIALIZED_DIGITS_POSITION: usize = 4;
const SERIALIZED_NUMPAD_POSITION: usize = 7;

fn characters_filter(key_code: KeyCode) -> Option<char> {
    serde_json::to_string(&key_code)
        .ok()
        .and_then(|serialized| letter_filter(&serialized).or_else(|| digits_filter(&serialized)))
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

#[cfg(test)]
mod should {
    use super::*;

    use test_case::test_case;

    #[test_case(KeyCode::A => Some('a'))]
    #[test_case(KeyCode::B => Some('b'))]
    #[test_case(KeyCode::C => Some('c'))]
    #[test_case(KeyCode::F1 => None)]
    #[test_case(KeyCode::F2 => None)]
    #[test_case(KeyCode::Left => None)]
    fn filter_not_characters_codes(key_code: KeyCode) -> Option<char> {
        characters_filter(key_code)
    }
}
