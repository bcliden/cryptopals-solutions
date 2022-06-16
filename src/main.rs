mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lib::conversion::{base64, hex};
    use crate::lib::{analysis, manipulate};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_do_pset1_challenge1() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let decoded_hex = hex::decode(hex_string).unwrap();

        let encoded_base64 = base64::encode(decoded_hex.as_slice());
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(encoded_base64, base64)
    }

    #[test]
    fn should_do_pset1_challenge2() -> TestResult {
        let start = "1c0111001f010100061a024b53535009181c";
        let xor_against = "686974207468652062756c6c277320657965";
        let decoded_answer = manipulate::xor_hex_strings(start, xor_against)?;
        assert_eq!(decoded_answer, "746865206B696420646F6E277420706C6179");
        Ok(())
    }

    #[test]
    fn should_do_pset1_challenge3() -> TestResult {
        // help from: https://crypto.stackexchange.com/a/30259
        let start = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let start_bytes = hex::decode(start)?;

        let mut key_byte: u8;

        let mut scores = vec![];

        for c in 0..=255 {
            key_byte = c as u8;

            let msg_bytes: Vec<u8> = manipulate::xor_bytes_with_char(&start_bytes, key_byte);
            let msg = String::from_utf8_lossy(&msg_bytes);

            // let score = analysis::english_text_score(&msg);
            let score = analysis::get_chi2_english(&msg);

            scores.push((score, msg.into_owned()));
        }

        // for "closeness score" use:
        // scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        // for chi2 use:
        scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        for entry in scores.iter().take(10) {
            println!("{:?}", entry);
        }

        Ok(())
    }
}
