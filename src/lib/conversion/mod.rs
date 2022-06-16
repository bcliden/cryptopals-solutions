pub mod alphabet;
pub mod base64;
pub mod hex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_back_and_forth() {
        let data = vec![0xD0, 0x0D]; // Prinny, dude! 🐧
        let encoded_hex = hex::encode(&data);
        let decoded_hex = hex::decode(encoded_hex.as_str()).unwrap();
        let encoded_base64 = base64::encode(decoded_hex.as_slice());
        let decoded_final = base64::decode(encoded_base64.as_str()).unwrap();
        // should go back and forth
        assert_eq!(decoded_final, data);
    }
}