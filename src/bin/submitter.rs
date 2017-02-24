use std::env::args;

fn main() {
    println!("{}", args().next().unwrap());
}