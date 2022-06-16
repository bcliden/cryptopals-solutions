use std::io;

use super::model::ClassicHex;
use crate::lib::conversion::alphabet::Alphabet;

pub fn decode(data: &str) -> Result<Vec<u8>, io::Error> {
    let classic_alphabet = ClassicHex {};
    decode_using_alphabet(&classic_alphabet, data)
}

fn decode_using_alphabet<T: Alphabet>(alphabet: &T, data: &str) -> Result<Vec<u8>, io::Error> {
    if data.len() % 2 != 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput))
    }

    data.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map::<Result<u8, io::Error>, _>(|chars| {
            let get_or_err = |&ch| {
                alphabet
                    .get_index_for_char(ch)
                    .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
            };

            let tuple = match chars {
                [first, second] => (
                    get_or_err(first)?, 
                    get_or_err(second)?
                ),
                _ => unreachable!(),
            };

            Ok(combine_bytes(tuple))
        })
        .collect()
}

fn combine_bytes(bytes: (u8, u8)) -> u8 {
    ((bytes.0 & 0x0F) << 4) | bytes.1 & 0x0F
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_hex_string() {
        let bytes = decode("DEADBEEF").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn decodes_hex_string_in_lowercase() {
        let bytes = decode("deadbeef").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn decodes_mixed_character_string() {
        let bytes = decode("c0cac07a").unwrap();
        assert_eq!(bytes, vec![0xC0, 0xCA, 0xC0, 0x7A]);
    }

    #[test]
    fn should_return_empty_vec() {
        assert!(decode("").unwrap().is_empty());
    }

    #[test]
    fn should_fail_gracefully() {
        // spaces and characters
        assert!(decode("Doogie Howser, MD").is_err());
        // just punctuation
        assert!(decode(",./!@#$^#*!@").is_err());
        // odd-length
        assert!(decode("BADA5").is_err());
    }

    #[test]
    fn should_combine_bytes() {
        let bytes = (0x0D, 0x0F);
        assert_eq!(combine_bytes(bytes), 0xDF)
    }
}
