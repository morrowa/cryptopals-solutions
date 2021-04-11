use std::env;
use std::fs::File;
use std::io::Read;

use cryptopals::io_utils::SkipNewlinesReader;

fn main() {
    if env::args().len() != 2 {
        panic!("wrong number of arguments");
    }
    let filename = env::args().next_back().unwrap();
    let mut file = File::open(filename).expect("could not open file");
    let mut skip_reader = SkipNewlinesReader::new(&mut file);
    let mut ciphertext: Vec<u8> = Vec::with_capacity(3000);
    let mut r = base64::read::DecoderReader::new(&mut skip_reader, base64::STANDARD);
    r.read_to_end(&mut ciphertext).expect("error decoding file");
}
