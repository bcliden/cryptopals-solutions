mod decode;
mod encode;
mod model;

pub use decode::decode;
pub use encode::encode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_back_and_forth() {
        let data = vec![0xD0, 0x0D, 0xCA, 0xBE]; // Prinny, dude! 🐧
        let encoded = encode(&data);
        assert_eq!(encoded, "DOODCABE");    
        assert_eq!(decode(encoded.as_str()).unwrap(), data);
    }
}