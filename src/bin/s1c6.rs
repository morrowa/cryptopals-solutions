use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use cryptopals::cos_sim::CharFreq;
use cryptopals::io_utils::SkipNewlinesReader;
use cryptopals::{brute_force_single_byte_xor, hamming_distance, repeating_key_xor};

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        panic!("wrong number of arguments");
    }
    args.next(); // skip program name
    let filename = args.next().unwrap();
    let mut file = File::open(filename).expect("could not open ciphertext file");
    let mut skip_reader = SkipNewlinesReader::new(&mut file);
    let mut r = base64::read::DecoderReader::new(&mut skip_reader, base64::STANDARD);
    let mut ciphertext: Vec<u8> = Vec::with_capacity(3000);
    r.read_to_end(&mut ciphertext).expect("error decoding file");

    let filename = args.next().unwrap();
    let file = File::open(filename).expect("could not open CSV file");
    let mut reader = BufReader::new(file);
    let ref_freqs = CharFreq::from_csv(&mut reader).expect("could not parse CSV file");

    // ciphertext is now ready

    // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes,
    // and find the edit distance between them. Normalize this result by dividing by KEYSIZE.

    // The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed
    // perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average
    // the distances.

    let mut scores: Vec<(usize, f64)> = (2..41)
        .map(|keysize| {
            let a = &ciphertext[0..keysize];
            let b = &ciphertext[keysize..(keysize * 2)];
            let dist = hamming_distance(a, b).unwrap();
            let score = (dist as f64) / (keysize as f64);
            (keysize, score)
        })
        .collect();
    scores.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    println!("{:#?}", scores);

    // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.

    let (keysize, _) = scores[0];
    // let keysize = 4;

    // Now transpose the blocks: make a block that is the first byte of every block, and a block
    // that is the second byte of every block, and so on.

    let transposed: Vec<Vec<u8>> = (0..keysize)
        .map(|i| {
            // take every keysize'th char, starting at i
            ciphertext[i..].iter().copied().step_by(keysize).collect()
        })
        .collect();

    // Solve each block as if it was single-character XOR. You already have code to do this.

    let key: Vec<u8> = transposed
        .iter()
        .map(|block| {
            let mut results = brute_force_single_byte_xor(block, &ref_freqs);
            results.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
            if results.is_empty() {
                println!("oh shit");
            }
            // TODO: some block is getting no values at all
            results.last().unwrap().key
        })
        .collect();

    println!("{:X?}", key);

    // For each block, the single-byte XOR key that produces the best looking histogram is the
    // repeating-key XOR key byte for that block. Put them together and you have the key.

    let plaintext = String::from_utf8(repeating_key_xor(&ciphertext, &key)).unwrap();
    println!("{}", plaintext);
}
