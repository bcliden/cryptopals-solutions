use crate::lib::{analysis, conversion::hex};
use std::{borrow::Borrow, error::Error};

use super::analysis::pick_best_english_string;

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

/// Given a hex string, figure out the secret message that was XOR'd against a single u8
pub fn guess_xor_message_one_char(hex_string: &str) -> Result<String, Box<dyn Error>> {
    let bytes = hex::decode(hex_string)?;

    let strings: Vec<_> = (0..=255_u8)
        .into_iter()
        .map(|n| xor_bytes_with_char(&bytes, n))
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        .collect();

    let best = pick_best_english_string(&strings);
    Ok(String::from(best))
}
