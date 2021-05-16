use std::env;
use std::fs;
use std::fs::File;
use cryptopals::aes_cbc;
use std::io::Write;

fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        panic!("wrong number of args");
    }
    args.next(); // skip program name
    let plaintext = fs::read(args.next().unwrap()).expect("could not read plaintext");
    let mut out_file = File::create(args.next().unwrap()).expect("could not create output file");
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];
    let ciphertext = aes_cbc::encrypt(key, &iv, &plaintext);
    out_file.write_all(&ciphertext).expect("could not write to output file");
}