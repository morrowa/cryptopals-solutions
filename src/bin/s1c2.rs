extern crate hex;
use std::env;

fn main() {
    if env::args().len() != 3 {
        panic!("wrong number of args");
    }
    let byte_strs: Vec<Vec<u8>> = env::args().collect::<Vec<String>>()[1..].iter()
    .map(|x| hex::decode(x).expect(&format!("could not parse as hex: {}", x))).collect();
    let first = &byte_strs[0];
    let second = &byte_strs[1];
    assert_eq!(first.len(), second.len(), "Argument strings were not the same length");
    let result: Vec<u8> = first.iter().zip(second.iter()).map(|(x, y)| x ^ y).collect();
    println!("{}", hex::encode(result));
}