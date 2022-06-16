use super::model::ClassicHex;
use crate::lib::conversion::alphabet::Alphabet;

pub fn encode(data: &[u8]) -> String {
    let classic_alphabet = ClassicHex {};
    encode_using_alphabet(&classic_alphabet, data)
}

fn encode_using_alphabet<T: Alphabet>(alphabet: &T, data: &[u8]) -> String {
    data.iter()
        .map(|&b| split_byte(b))
        .flat_map(|byte_tuple| {
            std::iter::once(alphabet.get_char_for_index(byte_tuple.0).unwrap())
                .chain(std::iter::once(
                    alphabet.get_char_for_index(byte_tuple.1).unwrap(),
                ))
                .collect::<Vec<char>>()
        })
        .collect()
}

fn split_byte(byte: u8) -> (u8, u8) {
    ((byte & 0xF0) >> 4, (byte & 0x0F))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode() {
        let bytes = vec![0xCA, 0xFE];
        assert_eq!(encode(bytes.as_slice()), "CAFE");
    }

    #[test]
    fn should_encode_single_bytes() {
        let bytes = vec![0xC, 0xA, 0xF, 0xE];
        assert_eq!(encode(bytes.as_slice()), "0C0A0F0E");
    }

    #[test]
    fn should_split_bytes() {
        let byte: u8 = 0xDF_u8;
        assert_eq!(split_byte(byte), (0xD, 0xF))
    }
}
