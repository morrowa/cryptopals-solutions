use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum CharFreqError {
    InvalidChar(u8),
    IoError(io::Error),
    InvalidCsv(&'static str),
}

impl Display for CharFreqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CharFreqError::InvalidChar(c) => {
                f.write_fmt(format_args!("invalid character: {:#X}", c))
            }
            CharFreqError::IoError(e) => f.write_fmt(format_args!("IO error: {}", e)),
            CharFreqError::InvalidCsv(e) => f.write_fmt(format_args!("invalid CSV: {}", e)),
        }
    }
}

impl error::Error for CharFreqError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CharFreqError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

pub struct CharFreq {
    frequencies: [f64; 70],
    magnitude: f64,
}

impl CharFreq {
    pub fn from_csv<R: io::BufRead>(r: R) -> Result<CharFreq, CharFreqError> {
        let mut counts: [u32; 70] = [0; 70];
        for line in r.lines() {
            let line = line.map_err(|e| CharFreqError::IoError(e))?;
            let mut parts = line.split(',');
            let chr = parts
                .next()
                .ok_or(CharFreqError::InvalidCsv("not enough columns"))?;
            let cnt = parts
                .next()
                .ok_or(CharFreqError::InvalidCsv("not enough columns"))?;
            if parts.next().is_some() {
                return Err(CharFreqError::InvalidCsv("too many columns"));
            }
            let chr: u8 = chr
                .parse()
                .map_err(|_| CharFreqError::InvalidCsv("invalid char"))?;
            let cnt: u32 = cnt
                .parse()
                .map_err(|_| CharFreqError::InvalidCsv("invalid count"))?;
            match chr {
                // tab, lf
                9..=10 => counts[chr as usize - 9] += cnt,
                // most chars
                32..=96 => counts[chr as usize - 30] += cnt,
                // { | } ~
                123..=126 => counts[chr as usize - 57] += cnt,
                // all others are invalid
                _ => return Err(CharFreqError::InvalidChar(chr)),
            }
        }

        let len = counts.iter().sum::<u32>() as f64;

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

    /// Higher number means more similar
    pub fn cosine_similarity(&self, other: &CharFreq) -> f64 {
        dot_product(&self.frequencies, &other.frequencies) / (self.magnitude * other.magnitude)
    }
}

// todo: this is decent, but the compiler fails to auto-vectorize
// there's a simd library on macOS and iOS that has a dot product for 8-element vectors
// although, this is probably the most accurate since we use FMA for each operation
// still seems prone to catastrophic cancellation since we may have very different numbers
// maybe not different enough... we'll be within a few orders of magnitude, probably
fn dot_product(a: &[f64; 70], b: &[f64; 70]) -> f64 {
    a.iter()
        .zip(b.iter())
        .fold(0.0, |a, (&l, &r)| l.mul_add(r, a))
}
