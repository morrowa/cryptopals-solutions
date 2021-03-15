use strsim::normalized_levenshtein;

const SORTED_LETTERS: &'static str = "etaoinsrhdlucmfywgpbvkxqjz";

pub fn score_string(text: &str) -> f64 {
    let lower = text.to_ascii_lowercase();
    let mut counts: [u16; 256] = [0; 256];
    for b in lower.bytes() {
        counts[b as usize] += 1;
    }
    let mut printable_by_freq: Vec<(u16, char)> = counts[32..127].iter().cloned().enumerate().filter(|(_, x)| *x > 0).map(|(i, x)| (x, (32 + i as u8) as char)).collect();
    printable_by_freq.sort_by_key(|x| x.0);
    let ordered_letters: String = printable_by_freq.iter().rev().map(|(_, x)| x ).collect();
    normalized_levenshtein(&ordered_letters, SORTED_LETTERS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_score() {
        let test_str = "The quick red fox jumps over the lazy brown dog.";
        println!("score: {}", score_string(test_str));
    }
}