extern crate base64;
extern crate hex;
use std::env;

fn main() {
    if env::args().len() != 2 {
        panic!("wrong number of arguments");
    }
    let hex_str = env::args().next_back().unwrap();
    let bytes = hex::decode(hex_str).expect("could not parse argument as hex");
    let b64 = base64::encode(bytes);
    println!("{}", b64);
}
