use std::error::Error;
use std::fmt;

use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
pub enum StringScoreError {
    IllegalChars,
}

impl fmt::Display for StringScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "string contains non-ASCII or unprintable characters")
    }
}

impl Error for StringScoreError {}

const SORTED_LETTERS: &'static str = "etaoinsrhdlucmfywgpbvkxqjz";

/// Scores the likelihood that a string is English plaintext by computing letter frequency. Only
/// allows printable ASCII characters (i.e. 32 through 126 inclusive).
pub fn score_string(text: &str) -> Result<f64, StringScoreError> {
    if !text.is_ascii() { return Err(StringScoreError::IllegalChars); }
    let lower = text.to_ascii_lowercase();
    let mut counts: [u16; 256] = [0; 256];
    for b in lower.bytes() {
        if b < 32 || b > 126 {
            return Err(StringScoreError::IllegalChars);
        }
        counts[b as usize] += 1;
    }
    let mut printable_by_freq: Vec<(u16, char)> = counts[32..127].iter().cloned().enumerate().filter(|(_, x)| *x > 0).map(|(i, x)| (x, (32 + i as u8) as char)).collect();
    printable_by_freq.sort_by_key(|x| x.0);
    let ordered_letters: String = printable_by_freq.iter().rev().map(|(_, x)| x ).collect();
    Ok(normalized_levenshtein(&ordered_letters, SORTED_LETTERS))
}