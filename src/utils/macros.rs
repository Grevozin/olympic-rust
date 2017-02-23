pub use std::cmp::Ordering;
pub use std::collections::VecDeque;
pub use std::fmt::{Display, Debug, Formatter, Error};
pub use std::ops::{Mul, Sub, Add, Index, IndexMut, Range};
pub use std::str::FromStr;
pub use std::string::ParseError;

/// Reads a finite number of input tokens from stdin, divided by spaces:
/// ``````
/// readln!(x: i64, y: f64, z: String);
/// ``````
/// Given an input "12 3.4 adfjk", sets x = 12, y = 3.4, z = "adfjk".
/// Note that the variables become immutable to supress Rust warnings.
/// I didn't saw a case where this was inconvenient.
/// Note that the implementation is rather suboptimal for reading only 1 item,
/// but it is left such for the sake of clarity.
#[macro_export]
macro_rules! readln {
    ( $( $x:ident: $t:ty ),* ) => (
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to readln!");
        let mut list = buffer.trim().split(" ");
        $(
            let $x: $t = list.next().unwrap().parse().expect("Incorrect input format");
        )*
    );
}

/// Reads a vector of given type from stdin, using spaces as separators:
/// ``````
/// readvec!(x: i64);
/// ``````
/// Given an input "12 34 56 78", sets x: Vec<i64> = vec![12, 34, 56, 78].
/// Unlike readln!, the resulting vector is mutable.
/// I didn't saw a case where this was inconvenient.
#[macro_export]
macro_rules! readvec {
    ($x:ident: $t:ty) => (
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to readvec!");
        let mut $x = buffer.trim().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<$t>>();
    );
}

/// Reads a 2d vector of chars from stdin, assuming every line is an dimension 1 element.
/// Separating '\n' signs are dropped.
/// ``````
/// read2dchar!(2, 3, x);
/// ``````
/// Given an input ".*\n**\n..", produces
/// x: Vec<Vec<char>> = vec![vec!['.', '*'], vec!['*', '*'], vec!['.', '.']]
/// Notes:
/// 1) The resulting vector is mutable.
/// 2) Width is used only to set initial Vec capacity appropriately.
#[macro_export]
macro_rules! read2dchar {
    ($width:expr, $height:expr, $ans:ident) => (
        let mut $ans = vec![Vec::with_capacity($width); $height];
        let mut buffer = String::new();
        for i in 0..$height {
            std::io::stdin().read_line(&mut buffer).expect("Failed to read2dchar!");
            $ans[i] = buffer.trim().chars().collect(); // TODO: Remove index checking!
            buffer.clear();
        }
    );
}