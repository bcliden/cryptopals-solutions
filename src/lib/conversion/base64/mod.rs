/*

Base64 conversion code. Gleefully worked through and studied on 6/12/22

Thank you to:
https://tiemenwaterreus.com/posts/implementing-base64-in-rust/

*/
mod decode;
mod encode;
mod model;

pub use decode::decode;
pub use encode::encode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_back_and_forth_dood() {
        let data = vec![0xD0, 0x0D]; // Prinny, dude! 🐧
        let encoded = encode(&data);
        assert_eq!(encoded, "0A0=");
        let decoded = decode(encoded.as_str()).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn should_convert_back_and_forth_deadbeef() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let encoded = encode(&data);
        assert_eq!(encoded, "3q2+7w==");
        assert_eq!(decode(dbg!(encoded).as_str()).unwrap(), data);
    }
}
