extern crate hex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use cryptopals::cos_sim::CharFreq;
use cryptopals::{brute_force_single_byte_xor, BruteForceResult};

// We know that the final string will be ASCII. I checked all non-ASCII (but valid UTF-8) strings,
// and they were all garbage.
// So we only need to check ASCII strings.
fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        panic!("wrong number of arguments");
    }
    args.next();
    let filename = args.next().unwrap();
    let csv_filename = args.next().unwrap();
    let csv_file = File::open(csv_filename).expect("could not open CSV file");
    let csv_reader = BufReader::new(csv_file);
    let ref_reqs = CharFreq::from_csv(csv_reader).expect("could not parse CSV");

    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);
    let mut results: Vec<(usize, BruteForceResult)> = reader
        .lines()
        .enumerate()
        .map(|(no, line)| {
            brute_force_single_byte_xor(&hex::decode(line.unwrap()).unwrap(), &ref_reqs)
                .into_iter()
                .map(move |r| (no, r))
        })
        .flatten()
        .collect();
    results.sort_unstable_by(|x, y| x.1.score.partial_cmp(&y.1.score).unwrap().reverse());

    println!("Top result:");
    let top = results.first().unwrap();
    let r = &top.1;
    println!(
        "Line {} ({:#x}) ({:.4}) {}",
        top.0,
        r.key,
        r.score,
        r.plaintext.escape_default()
    );
}
