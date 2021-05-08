use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    if env::args().len() != 3 {
        panic!("wrong number of arguments");
    }
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap();
    let mut file = File::open(filename).expect("could not open file");

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let mut counts: HashMap<char, usize> = HashMap::new();
    for mut chr in text.chars().flat_map(char::to_uppercase) {
        if chr == '\u{2013}' || chr == '\u{2014}' {
            // map all dashes to simple ASCII dashes
            chr = '-';
        } else if chr == '\u{2018}' || chr == '\u{2019}' {
            chr = '\'';
        } else if chr == '\u{201c}' || chr == '\u{201d}' {
            chr = '"';
        } else if chr == '\r' || chr > '~' {
            // ignoring everything past ASCII
            continue;
        }
        let x = *counts.get(&chr).unwrap_or(&0);
        counts.insert(chr, x + 1);
    }

    let mut counts: Vec<(char, usize)> = counts.into_iter().collect();
    counts.sort_by_key(|x| x.0);

    let mut out_file = File::create(args.next().unwrap()).expect("unable to open output file");
    let mut buf: [u8; 1] = [0];
    for (chr, cnt) in counts {
        // println!("\t({}) {}", if chr.is_ascii() { "A" } else { "U" }, cnt);
        println!("{}\t({}) {}", chr.escape_default(), chr.escape_unicode(), cnt);
        // out_file.write_fmt(format_args!("\"{}\",{}\n", chr.escape_default(), cnt)).unwrap();
        assert_eq!(chr.encode_utf8(&mut buf).len(), 1);
        out_file.write_fmt(format_args!("{},{}\n", buf[0], cnt)).unwrap();
    }
}
