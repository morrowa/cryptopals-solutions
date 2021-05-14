pub mod cos_sim;
pub mod io_utils;

use cos_sim::CharFreq;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct BruteForceResult {
    pub key: u8,
    /// score range is 0-1 inclusive. higher scores are more likely
    pub score: f64,
    pub plaintext: String,
}

fn xor(in_bytes: &[u8], key: u8) -> Vec<u8> {
    in_bytes.iter().copied().map(|x| x ^ key).collect()
}

/// Returns possible plaintexts unsorted
pub fn brute_force_single_byte_xor(
    ciphertext: &[u8],
    reference_freqs: &CharFreq,
) -> Vec<BruteForceResult> {
    (0..=255)
        .flat_map(|key| {
            String::from_utf8(xor(ciphertext, key))
                .ok()
                .and_then(|plaintext| {
                    CharFreq::from_str(&plaintext)
                        .ok()
                        .map(|freq| BruteForceResult {
                            key,
                            score: freq.cosine_similarity(reference_freqs),
                            plaintext,
                        })
                })
        })
        .collect()
}

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    assert!(key.len() < plaintext.len());
    plaintext
        .iter()
        .zip(key.iter().cycle())
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

/// Larger number means less similar
pub fn hamming_distance(a: &[u8], b: &[u8]) -> Result<u64, HammingError> {
    if a.len() != b.len() {
        return Err(HammingError::DifferentLengthArgs);
    }
    let dist = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones() as u64)
        .sum();
    Ok(dist)
}

pub fn pkcs7_pad(buf: &mut Vec<u8>, len: usize) {
    let to_add = len - (buf.len() % len);
    for _ in 0..to_add {
        buf.push(to_add as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    const TEST_VEC: &str =
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
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

    #[test]
    fn test_pkcs7_pad() {
        let mut input: Vec<u8> = Vec::from(b"YELLOW SUBMARINE".to_owned());
        pkcs7_pad(&mut input, 20);
        assert_eq!(input, b"YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
