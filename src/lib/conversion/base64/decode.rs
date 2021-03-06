use std::io;

use super::model::ClassicBase64;
use crate::lib::conversion::alphabet::Alphabet;

pub fn decode(data: &str) -> Result<Vec<u8>, std::io::Error> {
    let classic_alphabet = &ClassicBase64 {};
    decode_using_alphabet(classic_alphabet, data)
}

pub fn decode_using_alphabet<T: Alphabet>(
    alphabet: &T,
    data: &str,
) -> Result<Vec<u8>, std::io::Error> {
    if data.chars().count() % 4 != 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let mut result: Vec<_> = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| original(alphabet, chunk))
        .flat_map(stitch)
        .collect();

    /*
        Only strip ending zeroes AFTER all chunks are together.
    */

    let final_zeroes_index = result
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &el)| el != 0x0000)
        .map(|(idx, _)| idx + 1)
        .unwrap_or_else(|| result.len());
    result.drain(final_zeroes_index..);

    Ok(result)
}

fn original<T: Alphabet>(alphabet: &T, chunk: &[char]) -> Vec<u8> {
    chunk
        .iter()
        .filter(|&&character| character != alphabet.get_padding_char())
        .map(|&character| {
            alphabet
                .get_index_for_char(character)
                .expect("Unable to find character in alphabet")
        })
        .collect()
}

/// Turn an array of six-byte clusters into proper u8s
fn stitch(bytes: Vec<u8>) -> Vec<u8> {
    let length = bytes.len();
    let out = match length {
        2 => vec![
            // six bytes from first + two bytes from second
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            // remaining four bytes
            (bytes[1] & 0b00001111) << 4,
        ],
        3 => vec![
            // six bytes from first + two bytes from second
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            // four bytes from second, four bytes from third
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            // remaining four bytes
            (bytes[2] & 0b00000011) << 6,
        ],
        4 => vec![
            // six bytes from first + two bytes from second
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            // four bytes from second, four bytes from third
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            // remaining two bytes + all six bytes in third
            (bytes[2] & 0b00000011) << 6 | bytes[3] & 0b00111111,
        ],

        _ => unreachable!(),
    };

    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use std::io;

    #[test]
    fn decode_one() {
        let encoded = String::from("YQ==");
        let expected = "a".as_bytes();

        assert!(decode(&encoded).is_ok());
        assert_eq!(decode(&encoded).unwrap(), expected);
    }

    #[test]
    fn decode_two() {
        let encoded = String::from("YWI=");
        let expected = "ab".as_bytes();
        assert!(decode(&encoded).is_ok());
        assert_eq!(decode(&encoded).unwrap(), expected);
    }

    #[test]
    fn decode_three() {
        let encoded = String::from("YWJj");
        let expected = "abc".as_bytes();
        assert!(decode(&encoded).is_ok());
        assert_eq!(decode(&encoded).unwrap(), expected);
    }

    #[test]
    fn invalid_data() {
        let encoded = String::from("d91jd");
        assert!(decode(&encoded).is_err());
        assert_eq!(
            decode(&encoded).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }

    #[test]
    fn should_handle_zeroes_midstring() {
        let encoded = String::from("0A0=");
        let bytes = decode(&encoded).unwrap();
        assert_eq!(bytes, vec![0xd0, 0x0d]);
        assert_eq!(bytes, externalbase64::decode(encoded).unwrap());
    }

    #[test]
    fn should_match_decode_from_rust_base64() {
        let input = include_str!("../../../../files/pset1challenge6.txt");
        let cleaned_input: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();

        let our_bytes = decode(&cleaned_input).unwrap();
        let their_bytes = externalbase64::decode(cleaned_input).unwrap();

        let test_ours = our_bytes.iter().collect_vec();
        let test_theirs = their_bytes.iter().collect_vec();

        assert_eq!(test_ours, test_theirs);
    }
}
