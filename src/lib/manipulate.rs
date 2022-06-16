use crate::lib::conversion::hex;
use std::error::Error;

pub fn xor_hex_strings(left: &str, right: &str) -> Result<String, Box<dyn Error>> {
    if left.len() != right.len() {
        return Err("Lengths do not match".into());
    }

    let decoded_left = hex::decode(left)?;
    let decoded_right = hex::decode(right)?;

    let bytes_answer: Vec<u8> = std::iter::zip(decoded_left.into_iter(), decoded_right.into_iter())
        .map(|(left, right)| left ^ right)
        .collect();
    let decoded_answer = hex::encode(&bytes_answer);
    Ok(decoded_answer)
}

pub fn xor_bytes_with_char(bytes: &[u8], operand: u8) -> Vec<u8> {
    bytes.iter().map(|b| b ^ operand).collect()
}
