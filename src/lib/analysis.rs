const ASCII_UPPERCASE: usize = 65;
const ASCII_LOWERCASE: usize = 97;
// via https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
// http://en.algoritmy.net/article/40379/Letter-frequency-English
const ENGLISH_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z, space
];

/// The LARGER the number, the more it resembles english (more rudimentary)
pub fn english_text_score(string: &str) -> f64 {
    // help from: https://crypto.stackexchange.com/a/30259
    let mut counts: [f64; 27] = [0.0; 27];

    string.chars().for_each(|c| match c {
        'a'..='z' => counts[c as usize - ASCII_LOWERCASE] += 1.0,
        'A'..='Z' => counts[c as usize - ASCII_UPPERCASE] += 1.0,
        ' ' => counts[26] += 1.0,
        _ => {} // not ascii perhaps?
    });

    let mut score = 0.0;
    for i in 0..27 {
        score += (counts[i]) * ENGLISH_FREQ[i];
    }

    score
}

/// The SMALLER the number, the more it resembles english.
pub fn get_chi2_english(string: &str) -> f64 {
    // help from: https://crypto.stackexchange.com/a/30259
    let mut counts: [f64; 27] = [0.0; 27];
    let mut ignored_chars: usize = 0;

    string.chars().for_each(|c| match c {
        'a'..='z' => counts[c as usize - ASCII_LOWERCASE] += 1.0,
        'A'..='Z' => counts[c as usize - ASCII_UPPERCASE] += 1.0,
        ' ' => counts[26] += 1.0,
        c if c as u8 == 9 || c as u8 == 10 || c as u8 == 13 => ignored_chars += 1,
        c if c.is_ascii_punctuation() || c.is_ascii_digit() => ignored_chars += 1,
        _ => ignored_chars += 1, // not ascii perhaps?
    });

    let mut chi2: f64 = 0.0;
    let length = string.len() - ignored_chars;

    for i in 0..27 {
        let observed = counts[i];
        let expected = length as f64 * ENGLISH_FREQ[i];
        let difference = observed - expected;
        chi2 += (difference * difference) / expected;
    }

    chi2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_closeness_works() {
        let good_str = "Cooking MC's like a pound of bacon";
        let bad_str = "Ieeacdm*GI-y*fcao*k*zeku";

        let bad_str_score = english_text_score(bad_str);
        let good_str_score = english_text_score(good_str);

        // a higher score is more like english
        assert!(good_str_score > bad_str_score);
    }

    #[test]
    fn chi2_works() {
        let good_str = "Cooking MC's like a pound of bacon";
        let bad_str = "Ieeacdm*GI-y*fcao*k*zeku";

        let bad_str_chi2 = get_chi2_english(bad_str);
        let good_str_chi2 = get_chi2_english(good_str);

        // a LOWER chi2 is more like english
        assert!(good_str_chi2 < bad_str_chi2);
    }
}
