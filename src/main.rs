mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lib::conversion::{self, base64, hex};
    use crate::lib::{analysis, manipulate};
    use itertools::Itertools;
    use rayon::prelude::*;

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
        let ans = manipulate::guess_xor_message_one_char(&start_bytes).unwrap();
        assert_eq!(
            ans.decoded,
            String::from("Cooking MC's like a pound of bacon")
        );
        Ok(())
    }

    #[test]
    fn should_do_pset1_challenge4() -> TestResult {
        // detect single character XOR!

        let text = include_str!("../files/pset1challenge4.txt");
        let strings: Vec<String> = text
            .par_lines()
            .filter_map(|s| hex::decode(s).ok())
            .map(|v| {
                manipulate::guess_xor_message_one_char(&v).expect("Something happened when XORing")
            })
            .map(|ans| ans.decoded)
            .collect();

        let best_string = analysis::pick_best_english_string(&strings);
        assert_eq!(best_string, "Now that the party is jumping\n");
        Ok(())
    }

    #[test]
    fn should_do_pset1_challenge5() -> TestResult {
        let strings: &str = r#"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"#;
        let pattern = b"ICE";
        let xored: Vec<_> =
            manipulate::xor_bytes_with_repeating_pattern(strings.as_bytes(), pattern);
        let results = conversion::hex::encode(&xored);
        let expected_results = "0B3637272A2B2E63622C2E69692A23693A2A3C6324202D623D63343C2A26226324272765272A282B2F20430A652E2C652A3124333A653E2B2027630C692B20283165286326302E27282F";
        assert_eq!(results, expected_results);
        Ok(())
    }

    #[test]
    fn should_do_pset1_challenge6() -> TestResult {
        /*
        Extremely helpful:
           (why does the hamming distance help with guessing the keysize?)
           https://crypto.stackexchange.com/questions/8115/repeating-key-xor-and-hamming-distance/8118#8118
        */

        #[derive(Debug, Clone, Copy)]
        struct Ranking {
            score: f64,
            keysize: usize,
        }

        let input = include_str!("../files/pset1challenge6.txt");
        let cleaned_input: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        let input_bytes = base64::decode(&cleaned_input)?;

        let mut hammed: Vec<_> = (2..=40)
            .into_iter()
            .map(|size| {
                let slice1 = &input_bytes[..size];
                let slice2 = &input_bytes[size..size * 2];
                let slice3 = &input_bytes[size * 2..size * 3];
                let slice4 = &input_bytes[size * 3..size * 4];

                let distance1 = analysis::get_hamming_distance(slice1, slice2).unwrap();
                let distance2 = analysis::get_hamming_distance(slice2, slice3).unwrap();
                let distance3 = analysis::get_hamming_distance(slice3, slice4).unwrap();

                let sum = distance1 + distance2 + distance3;
                let score = sum as f64 / (size * 3) as f64;

                Ranking {
                    score, // be SURE to normalize by keysize
                    keysize: size,
                }
            })
            .collect();

        // LOWER distance is better
        hammed.sort_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        #[derive(Debug, Clone)]
        struct Answer {
            key: String,
            decoded: String,
            english_score: f64,
        }

        // let's check the top 5.
        let mut top_ones: Vec<_> = hammed
            .iter()
            .take(5)
            .map(|answer| {
                let keysize = answer.keysize;
                // break into KEYSIZE chunks
                let chunks = input_bytes.chunks_exact(keysize).collect_vec();

                // transpose each index across chunks together
                let transposed_chunks: Vec<Vec<u8>> = (0..keysize)
                    .into_iter()
                    .map(|index| chunks.iter().map(|chunk| chunk[index]).collect())
                    .collect();

                // find the single XOR char key for those transposed chunks
                let keys = transposed_chunks
                    .iter()
                    .map(|chunk| manipulate::guess_xor_message_one_char(chunk).unwrap())
                    .collect_vec();

                let answer_key: String = keys.iter().map(|a| a.answer).collect();
                let answer = manipulate::xor_bytes_with_repeating_pattern(
                    &input_bytes,
                    answer_key.as_bytes(),
                );
                let answer_as_string = String::from_utf8(answer).unwrap_or_default();

                Answer {
                    key: answer_key,
                    english_score: analysis::english_text_score(&answer_as_string),
                    decoded: answer_as_string,
                }
            })
            .collect();

        // now let's pick the most english one! HIGHER is better
        top_ones.sort_by(|a, b| {
            b.english_score
                .partial_cmp(&a.english_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_guess = top_ones[0].clone();

        // println!("now presenting the best answer!");
        // println!("Key: {}", best_guess.key);
        // println!("Text: {}", best_guess.decoded);
        //         let first_stanza = r#"I'm back and I'm ringin' the bell
        // A rockin' on the mike while the fly girls yell
        // In ecstasy in the back of me
        // Well that's my DJ Deshay cuttin' all them Z's
        // Hittin' hard and the girlies goin' crazy
        // Vanilla's on the mike, man I'm not lazy."#;

        assert_eq!(best_guess.key, "Terminator X: Bring the noise");
        Ok(())
    }
}
