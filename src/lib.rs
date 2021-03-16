// the weight of letters from a to z
// divide by 10 to get the decimal percentage
const WEIGHTS: [i16; 26] = [82, 15, 28, 43, 130, 22, 20, 61, 70, 1, 8, 40, 24, 67, 75, 19, 1, 60, 63, 91, 28, 10, 24, 1, 20, 1];

/// Scores the likelihood that a string is English plaintext by computing letter frequency. Only
/// allows printable ASCII characters (i.e. 32 through 126 inclusive). Scores range from 0.0-1.0.
pub fn score_string(text: &str) -> Option<f64> {
    if !text.is_ascii() { return None; }
    let lower = text.to_ascii_lowercase();
    let mut counts: [i16; 95] = [0; 95];
    for b in lower.bytes() {
        if b < 32 || b > 126 {
            return None;
        }
        counts[(b - 32) as usize] += 1;
    }
    // convert counts to 10*frequencies
    for i in 0..95 {
        counts[i] = (counts[i] * 1000) / text.len() as i16;
    }
    // now we want to subtract our fixed weights, then sum the result, then abs(1/x)
    for i in 0..26 {
        counts[i + 65] -= WEIGHTS[i];
    }
    let sum: i16 = counts.iter().cloned().map(i16::abs).sum();
    Some(1.0 / (sum + 1) as f64)
}