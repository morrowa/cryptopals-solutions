use aes::{Aes128, Block, BlockDecrypt, NewBlockCipher};
use hex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use cryptopals::hamming_distance;

const BLOCK_SIZE: usize = 16;
const KEY: &[u8; BLOCK_SIZE] = b"YELLOW SUBMARINE";

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("wrong number of arguments");
    }
    args.next(); // skip program name
    let filename = args.next().unwrap();
    let mut file = File::open(filename).expect("could not open input file");
    let reader = BufReader::new(&file);

    // I know this is the length of each line
    // let mut buf: [u8; 160] = [0; 160];
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     hex::decode_to_slice(line, &mut buf).unwrap();
        // TODO: decide if this line is probably ECB-encrypted English
        // should I try with their favorite key (YELLOW SUBMARINE)?
        // if the rest of the file is random noise, then there will be patterns in the ECB that aren't in the noise
        // so the hamming distance between the blocks should be the lowest, right?
    // }
    let mut scores: Vec<(usize, f64)> = reader.lines().enumerate().map(|(num, line)| {
        let ciphertext = hex::decode(line.unwrap()).unwrap();
        assert_eq!(ciphertext.len() % BLOCK_SIZE, 0);
        let chunks: Vec<&[u8]> = ciphertext.chunks_exact(BLOCK_SIZE).collect();
        let (&first, rest) = chunks.split_first().unwrap();
        let dist = rest.iter().fold(0, |a, &other| a + hamming_distance(first, other).unwrap());
        let avg_dist: f64 = dist as f64 / (chunks.len() - 1) as f64;
        (num, avg_dist)
    }).collect();
    scores.sort_by(|(_, l), (_, r)| l.partial_cmp(r).unwrap());
    println!("{:?}", scores);

    // let's try decrypting the top two
    let cipher = Aes128::new(Block::from_slice(KEY));
    for (num, _) in scores.iter().take(10) {
        file.seek(SeekFrom::Start(0)).expect("could not seek");
        let reader = BufReader::new(&file);
        let mut ciphertext = hex::decode(reader.lines().nth(*num).unwrap().unwrap()).unwrap();
        for chunk in ciphertext.chunks_exact_mut(BLOCK_SIZE) {
            let mut block = Block::from_mut_slice(chunk);
            cipher.decrypt_block(&mut block);
        }
        match String::from_utf8(ciphertext) {
            Ok(s) => println!("plaintext: {}", s),
            Err(_) => println!("line {} was not UTF-8", num),
        }
    }
}

// easiest way to do this is recursion
// fn average_distance(s: &[u8], chunk_size: usize) -> Result<f64, &'static str> {
//     todo!()
// }