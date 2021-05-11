use aes::{Aes128, Block, BlockDecrypt, NewBlockCipher};
use std::env;
use std::fs::File;
use cryptopals::io_utils::SkipNewlinesReader;
use std::io::Read;

const BLOCK_LEN: usize = 16;
const KEY: &[u8; BLOCK_LEN] = b"YELLOW SUBMARINE";

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("wrong number of arguments");
    }
    args.next(); // skip program name
    let filename = args.next().unwrap();
    let mut file = File::open(filename).expect("could not open input file");
    let mut skip_reader = SkipNewlinesReader::new(&mut file);
    let mut r = base64::read::DecoderReader::new(&mut skip_reader, base64::STANDARD);
    let mut ciphertext: Vec<u8> = Vec::with_capacity(4000);
    r.read_to_end(&mut ciphertext).expect("error decoding file");
    if ciphertext.len() % BLOCK_LEN != 0 {
        panic!("ciphertext is not a multiple of the block length");
    }

    let key = Block::from_slice(KEY);
    let cipher: Aes128 = Aes128::new(key);
    // we're gonna mutate ciphertext in place
    for chunk in ciphertext.chunks_exact_mut(BLOCK_LEN) {
        let mut block = Block::from_mut_slice(chunk);
        cipher.decrypt_block(&mut block);
    }

    let plaintext = String::from_utf8(ciphertext).expect("plaintext was not UTF-8");
    println!("{}", plaintext);
}