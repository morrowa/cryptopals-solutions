// need a struct to represent letter frequencies
// internally, it should store the frequencies in a dense vector

pub enum CharFreqError {
    InvalidChar(u8),
}

pub struct CharFreq {
    frequencies: [f64; 70],
    magnitude: f64,
}

impl CharFreq {
    pub fn from_str(s: &str) -> Result<CharFreq, CharFreqError> {
        let mut counts: [u32; 70] = [0; 70];
        let uppercase = s.to_ascii_uppercase().into_bytes();
        let len = uppercase.len() as f64;
        for b in uppercase {
            match b {
                // tab, lf
                9..=10 => counts[b as usize - 9] += 1,
                // most chars
                32..=96 => counts[b as usize - 30] += 1,
                // { | } ~
                123..=126 => counts[b as usize - 57] += 1,
                // all others are invalid
                _ => return Err(CharFreqError::InvalidChar(b)),
            }
        }

        let mut frequencies: [f64; 70] = [0.0; 70];
        for (i, cnt) in counts.iter().enumerate() {
            frequencies[i] = (*cnt as f64) / len;
        }

        let magnitude = dot_product(&frequencies, &frequencies);

        Ok(CharFreq {
            frequencies,
            magnitude,
        })
    }

    pub fn cosine_dist(&self, other: &CharFreq) -> f64 {
        dot_product(&self.frequencies, &other.frequencies) / (self.magnitude * other.magnitude)
    }
}

// todo: this is decent, but the compiler fails to auto-vectorize
// there's a simd library on macOS and iOS that has a dot product for 8-element vectors
fn dot_product(a: &[f64; 70], b: &[f64; 70]) -> f64 {
    a.iter()
        .zip(b.iter())
        .fold(0.0, |a, (&l, &r)| l.mul_add(r, a))
    // .map(|(&l, &r)| l * r)
    // .sum()
}
