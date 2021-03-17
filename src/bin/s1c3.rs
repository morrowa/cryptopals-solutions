use cryptopals::brute_force_single_byte_xor;
use hex;

fn main() {
    let in_bytes = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    println!("Results in order:");
    let mut results = brute_force_single_byte_xor(&in_bytes);
    results.sort_unstable_by(|x, y| x.score.partial_cmp(&y.score).unwrap().reverse());
    for r in results {
        println!("{:#x} ({:.4}) {}", r.key, r.score, r.plaintext);
    }
}