use cryptopals::brute_force_single_byte_xor;
use hex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use cryptopals::cos_sim::CharFreq;

fn main() {
    let mut args = env::args();
    args.next();
    let csv_filename = args.next().expect("wrong number of arguments");
    let csv_file = File::open(&csv_filename).expect("could not open CSV file");
    let csv_reader = BufReader::new(csv_file);
    let ref_freqs = CharFreq::from_csv(csv_reader).expect("could not parse CSV");
    let in_bytes =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();
    println!("Results in order:");
    let mut results = brute_force_single_byte_xor(&in_bytes, &ref_freqs);
    results.sort_unstable_by(|x, y| x.score.partial_cmp(&y.score).unwrap().reverse());
    for r in results {
        println!("{:#x} ({:.4}) {}", r.key, r.score, r.plaintext);
    }
}
