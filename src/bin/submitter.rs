extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;

fn main() {
    let args: Vec<String> = args().collect();
    let (mut main, mut compiled): (&str, &str) = ("src/main.rs", "src/compiled.rs");
    if args.len() > 1 { main = &args[1];     }
    if args.len() > 2 { compiled = &args[2]; }

    let mut input = File::open(main).unwrap();
    let mut input_content = vec![];
    input.read_to_end(&mut input_content).expect(&format!("Failed to read {}", &main));
    let content = String::from_utf8(input_content).unwrap();

    let crate_re = Regex::new(r"extern crate [a-zA-Z_0-9]+;").unwrap();
    let out_content = crate_re.replace_all(&content, "// Crate definition was here!");

    let mut output = File::create(compiled).unwrap();
    output.write_all(&out_content.as_bytes()).expect(&format!("Failed to write into {}", &compiled));
}