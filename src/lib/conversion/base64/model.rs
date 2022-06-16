use crate::lib::conversion::alphabet::Alphabet;

pub struct ClassicBase64;
const UPPERCASEOFFSET: i8 = 65;
const LOWERCASEOFFSET: i8 = 71;
const DIGITOFFSET: i8 = -4;

impl Alphabet for ClassicBase64 {
    /// Map a char to the index in the base64 alphabet
    fn get_char_for_index(&self, index: u8) -> Option<char> {
        let index = index as i8;

        let ascii_index = match index {
            0..=25 => index + UPPERCASEOFFSET,  // A-Z
            26..=51 => index + LOWERCASEOFFSET, // a-z
            52..=61 => index + DIGITOFFSET,     // 0-9
            62 => 43,                           // +
            63 => 47,                           // /

            _ => return None,
        } as u8;

        Some(ascii_index as char)
    }

    /// Map an index in the Base64 alphabet to a char
    fn get_index_for_char(&self, character: char) -> Option<u8> {
        let character = character as i8;
        let base64_index = match character {
            65..=90 => character - UPPERCASEOFFSET,  // A-Z
            97..=122 => character - LOWERCASEOFFSET, // a-z
            48..=57 => character - DIGITOFFSET,      // 0-9
            43 => 62,                                // +
            47 => 63,                                // /

            _ => return None,
        } as u8;

        Some(base64_index)
    }

    fn get_padding_char(&self) -> char {
        '='
    }
}
