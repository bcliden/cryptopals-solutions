use std::iter::FromIterator;

use super::model::ClassicBase64;
use crate::lib::conversion::alphabet::Alphabet;

pub fn encode(data: &[u8]) -> String {
    let classic_alphabet = &ClassicBase64 {};
    encode_using_alphabet(classic_alphabet, data)
}

pub fn encode_using_alphabet<T: Alphabet>(alphabet: &T, data: &[u8]) -> String {
    let encoded = data
        .chunks(3)
        .map(split)
        .flat_map(|chunk| encode_chunk(alphabet, &chunk));

    String::from_iter(encoded)
}

/// Turn u8 chunks into divided six-byte chunks
fn split(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        1 => vec![
            // Grab first six,
            &chunk[0] >> 2,
            // then last two + 4 zeroes
            (&chunk[0] & 0b00000011) << 4,
        ],

        2 => vec![
            // Grab first six,
            &chunk[0] >> 2,
            // Last two + first four of next chunk,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            // Last four + two zeroes
            (&chunk[1] & 0b00001111) << 2,
        ],

        3 => vec![
            // Grab first six,
            &chunk[0] >> 2,
            // Last two + first four of next chunk,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            // last four of second chunk + two leftmost bits of next byte
            (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
            // use last six bits
            &chunk[2] & 0b00111111,
        ],

        _ => unreachable!(),
    }
}

fn encode_chunk<T: Alphabet>(alphabet: &T, chunk: &[u8]) -> Vec<char> {
    let mut out = vec![alphabet.get_padding_char(); 4];
    for i in 0..chunk.len() {
        if let Some(c) = alphabet.get_char_for_index(chunk[i]) {
            out[i] = c
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char() {
        let input_str = "a";
        let expected = "YQ==";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn test_two_chars() {
        let input_str = "ab";
        let expected = "YWI=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn test_three_chars() {
        let input_str = "abc";
        let expected = "YWJj";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }

    #[test]
    fn tests_short_string() {
        let input_str = "Hello, world!";
        let expected = "SGVsbG8sIHdvcmxkIQ==";

        let input = input_str.as_bytes();

        assert_eq!(encode(input), expected);
    }

    #[test]
    fn test_longer_string() {
        let input_str = "And here be a bit longer text. Let's see how it goes!";
        let expected = "QW5kIGhlcmUgYmUgYSBiaXQgbG9uZ2VyIHRleHQuIExldCdzIHNlZSBob3cgaXQgZ29lcyE=";

        let input_data = input_str.as_bytes();

        assert_eq!(encode(input_data), expected);
    }
}
