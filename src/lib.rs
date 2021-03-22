pub mod io_utils;

use strsim::normalized_levenshtein;
use std::fmt::{Display, Formatter};
use std::fmt;

const SORTED_LETTERS: &str = " etaoinshrdlcumwfgypbvkjxqz";

fn score_string_lev(text: &str) -> Option<f64> {
    if !text.is_ascii() { return None; }
    let lower = text.to_ascii_lowercase();
    let mut counts: [u16; 256] = [0; 256];
    for b in lower.bytes() {
        counts[b as usize] += 1;
    }
    let mut by_freq: Vec<(char, u16)> = counts.iter().cloned().enumerate()
        .filter(|(_, cnt)| *cnt > 0)
        .map(|(chr, cnt)| (chr as u8 as char, cnt))
        .collect();
    by_freq.sort_by_key(|(_, cnt)| *cnt);
    let ordered_letters: String = by_freq.iter().rev().map(|(x, _)| x ).collect();
    Some(normalized_levenshtein(&ordered_letters, SORTED_LETTERS))
}

// the weight of letters from a to z
// divide by 10 to get the decimal percentage
// const WEIGHTS: [i32; 26] = [82, 15, 28, 43, 130, 22, 20, 61, 70, 1, 8, 40, 24, 67, 75, 19, 1, 60, 63, 91, 28, 10, 24, 1, 20, 1];

// /// Scores the likelihood that a string is English plaintext by computing letter frequency. Only
// /// allows printable ASCII characters (i.e. 32 through 126 inclusive). Scores range from 0.0-1.0.
// fn score_string(text: &str) -> Option<f64> {
//     if !text.is_ascii() { return None; }
//     let lower = text.to_ascii_lowercase();
//     let mut counts: [i32; 95] = [0; 95];
//     for b in lower.bytes() {
//         if b < 32 || b > 126 {
//             return None;
//         }
//         counts[(b - 32) as usize] += 1;
//     }
//     // convert counts to 10*frequencies
//     for i in 0..95 {
//         counts[i] = (counts[i] * 1000) / text.len() as i32;
//     }
//     // now we want to subtract our fixed weights, then sum the result, then abs(1/x)
//     for i in 0..26 {
//         counts[i + 65] -= WEIGHTS[i];
//     }
//     let sum: i32 = counts.iter().cloned().map(i32::abs).sum();
//     Some(1.0 / (sum + 1) as f64)
// }

pub struct BruteForceResult {
    pub key: u8,
    pub score: f64,
    pub plaintext: String,
}

fn xor(in_bytes: &[u8], key: u8) -> Vec<u8> {
    in_bytes.iter().map(|x| *x ^ key).collect()
}

/// Returns possible plaintexts unsorted
pub fn brute_force_single_byte_xor(ciphertext: &[u8]) -> Vec<BruteForceResult> {
    let mut results= Vec::new();
    for key in 0..=255 {
        String::from_utf8(xor(ciphertext, key))
            .ok()
            .map(|plaintext| {
                score_string_lev(&plaintext)
                    .map(|score| results.push(BruteForceResult { key, score, plaintext }))
            });
    }
    results
}

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() < plaintext.len());
    plaintext.iter().zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect()
}

#[derive(Debug)]
pub enum HammingError {
    DifferentLengthArgs,
}

impl Display for HammingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let text = match self {
            HammingError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(f, "{}", text)
    }
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> Result<u64, HammingError> {
    if a.len() != b.len() {
        return Err(HammingError::DifferentLengthArgs);
    }
    let dist = a.iter().zip(b.iter())
        .fold(0, |a, (x, y)| a + (x ^ y).count_ones() as u64);
    Ok(dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    const TEST_VEC: &str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    const TEST_KEY: &str = "ICE";
    const TEST_VEC_CIPHER: [u8; 74] = hex!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    #[test]
    fn test_repeating_key_xor() {
        let ciphertext = repeating_key_xor(TEST_VEC.as_bytes(), TEST_KEY.as_bytes());
        assert_eq!(ciphertext.len(), TEST_VEC_CIPHER.len());
        assert_eq!(ciphertext, TEST_VEC_CIPHER);
    }

    #[test]
    fn test_hamming() {
        let a = "this is a test";
        let b = "wokka wokka!!!";
        assert_eq!(hamming_distance(a.as_bytes(), b.as_bytes()).unwrap(), 37);
    }
}