use cryptopals::aes_cbc;
use cryptopals::io_utils::SkipNewlinesReader;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("wrong number of args");
    }
    args.next(); // skip program name
    let filename = args.next().unwrap();
    let mut file = File::open(filename).expect("cannot open input");
    let mut skip_reader = SkipNewlinesReader::new(&mut file);
    let mut r = base64::read::DecoderReader::new(&mut skip_reader, base64::STANDARD);
    let mut ciphertext: Vec<u8> = Vec::with_capacity(3000);
    r.read_to_end(&mut ciphertext).expect("error decoding file");

    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];

    let plaintext = aes_cbc::decrypt(key, &iv, &ciphertext);
    match String::from_utf8(plaintext) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("cannot UTF-8 decode plaintext: {}", e),
    }
}
