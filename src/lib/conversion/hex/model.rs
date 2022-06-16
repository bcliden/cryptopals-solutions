use crate::lib::conversion::alphabet::Alphabet;

const DIGITOFFSET: u8 = 48;
const UPPERCASEOFFSET: u8 = 65 - 10;
const LOWERCASEOFFSET: u8 = 97 - 10;

pub struct ClassicHex;

impl Alphabet for ClassicHex {
    fn get_char_for_index(&self, index: u8) -> Option<char> {
        match index {
            0..=9 => Some((index + DIGITOFFSET) as char),
            10..=15 => Some((index + UPPERCASEOFFSET) as char),
            _ => None,
        }
    }

    fn get_index_for_char(&self, character: char) -> Option<u8> {
        let character = character as u8;
        match character {
            48..=57 => Some(character - DIGITOFFSET),
            65..=70 => Some(character - UPPERCASEOFFSET),
            97..=102 => Some(character - LOWERCASEOFFSET),
            _ => None,
        }
    }

    fn get_padding_char(&self) -> char {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_text_to_index() {
        let alphabet = ClassicHex {};
        let h = "05AF";
        let result: Vec<u8> = h
            .chars()
            .filter_map(|c| alphabet.get_index_for_char(c))
            .collect();
        assert_eq!(result, vec![0, 5, 10, 15])
    }

    #[test]
    fn should_convert_index_to_text() {
        let alphabet = ClassicHex {};
        let h: Vec<u8> = vec![0, 5, 10, 15];
        let result: String = h
            .iter()
            .filter_map(|&n| alphabet.get_char_for_index(n))
            .collect();
        assert_eq!(result, "05AF")
    }
}
