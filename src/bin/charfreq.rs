use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    if env::args().len() != 2 {
        panic!("wrong number of arguments");
    }
    let filename = env::args().next_back().unwrap();
    let mut file = File::open(filename).expect("could not open file");

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut counts: HashMap<char, usize> = HashMap::new();
    for chr in text.chars() {
        let x = *counts.get(&chr).unwrap_or(&0);
        counts.insert(chr, x + 1);
    }

    let mut counts: Vec<(char, usize)> = counts.into_iter().collect();
    counts.sort_by_key(|x| x.0);

    for (chr, cnt) in counts {
        if chr.is_control() {
            print!("{}", chr.escape_default());
        }
        else {
            print!("'{}'", chr);
        }
        println!("\t({}) {}", if chr.is_ascii() { "A" } else { "U" }, cnt);
    }
}