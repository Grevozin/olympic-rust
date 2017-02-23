#[macro_use]
mod utils; // Test change

fn main() {
    readln!(x: i64);
    for _ in 0..x {
        readln!(y: i64);
        println!("{}", y * (y + 1) / 2 - 2_i64.pow(((y as f64).log2() as u32) + 2) + 2);
    }
}