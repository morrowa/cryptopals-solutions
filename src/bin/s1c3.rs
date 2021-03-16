use cryptopals::score_string;
use hex;

fn xor(in_bytes: &[u8], key: u8) -> Vec<u8> {
    in_bytes.iter().map(|x| x ^ key).collect()
}

fn main() {
    let in_bytes = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let mut results: Vec<(u8, f64, String)> = Vec::new();
    for i in 0..=255 {
        String::from_utf8(xor(&in_bytes, i))
            .into_iter()
            .for_each(|s| {
                match score_string(&s) {
                    Ok(score) => results.push((i, score, s)),
                    Err(_) => (),
                };
            });
    }
    results.sort_unstable_by(|x, y| x.1.partial_cmp(&y.1).unwrap().reverse());
    println!("Results in order:");
    for (key, score, text) in results {
        println!("{:#x} ({:.3}) {}", key, score, text);
    }
}