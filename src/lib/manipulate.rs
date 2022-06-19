use crate::lib::{conversion::hex};
use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;

use super::analysis::pick_best_english_string;

#[derive(Debug)]
pub struct XoxSingleCharAnswer {
    pub answer: char,
    pub original: String,
    pub decoded: String,
}

pub fn xor_hex_strings(left: &str, right: &str) -> Result<String, Box<dyn Error>> {
    if left.len() != right.len() {
        return Err("Lengths do not match".into());
    }

    let decoded_left = hex::decode(left)?;
    let decoded_right = hex::decode(right)?;

    let bytes_answer: Vec<u8> = decoded_left
        .into_par_iter()
        .zip(decoded_right)
        .map(|(left, right)| left ^ right)
        .collect();
    let decoded_answer = hex::encode(&bytes_answer);
    Ok(decoded_answer)
}

pub fn xor_bytes_with_char(bytes: &[u8], operand: u8) -> Vec<u8> {
    bytes.par_iter().map(|b| b ^ operand).collect()
}

/// Given a byte slice, figure out the secret message that was XOR'd against a single u8
pub fn guess_xor_message_one_char(bytes: &[u8]) -> Result<XoxSingleCharAnswer, Box<dyn Error>> {
    let strings: Vec<_> = (0..=255_u8)
        .into_par_iter()
        .map(|n| (n, xor_bytes_with_char(bytes, n)))
        .map(|(byte, bytes)| (byte as char, String::from_utf8_lossy(&bytes).to_string()))
        .collect();
    let decoded_strings = strings
        .iter()
        .map(|(_, string)| string.clone())
        .collect_vec();

    let best_sentence = pick_best_english_string(&decoded_strings);
    let best_entry = strings
        .iter()
        .find(|(_, ans)| ans == best_sentence)
        .unwrap();

    let answer = XoxSingleCharAnswer {
        answer: best_entry.0,
        original: String::from_utf8_lossy(bytes).to_string(),
        decoded: best_entry.1.clone(),
    };

    Ok(answer)
}

pub fn xor_bytes_with_repeating_pattern(bytes: &[u8], pattern: &[u8]) -> Vec<u8> {
    let repeating_pattern = pattern.iter().cycle();

    std::iter::zip(bytes.iter(), repeating_pattern)
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_get_xor_single_char() -> TestResult {
        let original = "Doggie doggie what now?";
        let xored = xor_bytes_with_char(original.as_bytes(), b't');

        let answer = guess_xor_message_one_char(&xored)?;

        assert_eq!(answer.decoded, original);
        assert_eq!(answer.answer, 't');
        Ok(())
    }
}
