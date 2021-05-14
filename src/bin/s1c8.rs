use aes::{Aes128, BlockDecrypt, NewBlockCipher};
use hex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use cryptopals::cos_sim::CharFreq;

const BLOCK_SIZE: usize = 16;
const KEY: &[u8; BLOCK_SIZE] = b"YELLOW SUBMARINE";

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        panic!("wrong number of arguments");
    }
    args.next(); // skip program name
    let filename = args.next().unwrap();
    let file = File::open(filename).expect("could not open input file");
    let reader = BufReader::new(&file);

    let csv_filename = args.next().unwrap();
    let csv_file = File::open(csv_filename).expect("could not open CSV file");
    let csv_reader = BufReader::new(csv_file);
    let ref_freqs = CharFreq::from_csv(csv_reader).expect("could not read CSV file");

    let cipher = Aes128::new(KEY.into());
    let mut possible_plaintexts: Vec<(usize, f64, String)> = reader.lines().enumerate().flat_map(|(num, line)| {
        let mut ciphertext = hex::decode(line.unwrap()).unwrap();
        for chunk in ciphertext.chunks_exact_mut(BLOCK_SIZE) {
            cipher.decrypt_block(chunk.into());
        }
        String::from_utf8(ciphertext).ok().and_then(|plain| {
            println!("a plaintext was utf-8");
            CharFreq::from_str(&plain).map(|f| (num, f.cosine_similarity(&ref_freqs), plain)).ok()
        })
    }).collect();
    possible_plaintexts.sort_unstable_by(|(_, a, _), (_, b, _)| a.partial_cmp(b).unwrap());

    match possible_plaintexts.first() {
        Some((num, score, line)) => println!("Line {} ({}): {}", num, score, line),
        None => println!("No results"),
    }
}