use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use log::trace;
use serde::{de, ser::SerializeMap, Deserialize, Serialize};

const ALPHABET_LEN: usize = 26;
const ALPHABET: [char; ALPHABET_LEN] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
// no good way to conver [char] to [&'static str], mainly for de::Error::unknown_field
const ALPHABET_STR_SLICE: [&str; ALPHABET_LEN] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
const INIT_PERFORMANCE: f64 = 0.0;

// using f64 to store performance score because toml::Value::Float uses f64
#[derive(Debug)]

pub struct LetterPerformance([f64; ALPHABET_LEN]);

impl Default for LetterPerformance {
    fn default() -> Self {
        LetterPerformance([INIT_PERFORMANCE; ALPHABET_LEN])
    }
}

impl Index<usize> for LetterPerformance {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for LetterPerformance {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<char> for LetterPerformance {
    type Output = f64;

    fn index(&self, index: char) -> &Self::Output {
        if let Some(i) = ALPHABET.iter().position(|&letter| letter == index) {
            return &self.0[i];
        }
        panic!("Character {index} not in alphabet.")
    }
}
impl IndexMut<char> for LetterPerformance {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        if let Some(i) = ALPHABET.iter().position(|&letter| letter == index) {
            return &mut self.0[i];
        }
        panic!("Character {index} not in alphabet.")
    }
}

impl Display for LetterPerformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            ALPHABET
                .iter()
                .map(|&letter| format!("{}: {}", letter, self[letter]))
                .collect::<Vec<_>>()
                .join("\n")
                .as_str(),
        )
    }
}

impl Serialize for LetterPerformance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(ALPHABET_LEN))?;
        for (i, letter) in ALPHABET_STR_SLICE.iter().enumerate() {
            map.serialize_entry(letter, &self[i])?;
        }
        map.end()
    }
}

struct LetterPerformanceVisitor;
impl<'de> de::Visitor<'de> for LetterPerformanceVisitor {
    type Value = LetterPerformance;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of letters to floats")
    }

    fn visit_map<Access>(self, mut map: Access) -> Result<Self::Value, Access::Error>
    where
        Access: de::MapAccess<'de>,
    {
        /// Turn a `String` into `&'static str`  
        /// Looks bad but creating `de::Error` requires a ``&'static str`
        /// for erroneous fields
        fn get_static_slice(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }

        trace!("Running visit_map");
        let mut performance_data = LetterPerformance([INIT_PERFORMANCE; ALPHABET_LEN]);
        let mut seen_letter_flags = [false; ALPHABET_LEN];
        let unknown_field_err = |key: &str| Err(de::Error::unknown_field(key, &ALPHABET_STR_SLICE));

        while let Some((key_string, value)) = map.next_entry::<String, _>()? {
            let key_string = key_string.to_uppercase();
            let key = match key_string.parse::<char>() {
                Ok(chr) => chr,
                Err(_e) => return unknown_field_err(&key_string),
            };

            let index = match ALPHABET.iter().position(|&letter| letter == key) {
                Some(idx) => idx,
                None => return unknown_field_err(&key.to_string()),
            };

            if seen_letter_flags[index] {
                return Err(de::Error::duplicate_field(get_static_slice(key_string)));
            }

            performance_data[index] = value;
            seen_letter_flags[index] = true;
        }

        let missing_letters: Vec<_> = ALPHABET
            .iter()
            .enumerate()
            .filter_map(|(i, &letter)| {
                if !seen_letter_flags[i] {
                    Some(letter.to_string())
                } else {
                    None
                }
            })
            .collect();
        let missing_letters = missing_letters.join(", ");

        if !missing_letters.is_empty() {
            return Err(de::Error::missing_field(get_static_slice(missing_letters)));
        }

        Ok(performance_data)
    }
}

impl<'de> Deserialize<'de> for LetterPerformance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(LetterPerformanceVisitor)
    }
}
