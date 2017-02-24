/// Reads a finite number of input tokens from stdin, divided by spaces.
///
/// The variables become immutable to supress Rust warnings.
/// Note that the implementation is suboptimal for reading only 1 item,
/// but it is left such for the sake of clarity.
///
/// # Examples
///
/// ```rust,no_run
/// # #[macro_use]
/// # extern crate olympic_lib;
/// # fn main() {
/// readln!(x: i64, y: f64, z: String);
/// # }
/// ```
/// Given an input `12 3.4 hello`, sets `x = 12, y = 3.4, z = "hello"`.
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

/// Reads a vector of given type from stdin, using spaces as separators.
///
/// Unlike readln!, the resulting Vec is mutable.
///
/// # Examples
///
/// ```rust,no_run
/// # #[macro_use]
/// # extern crate olympic_lib;
/// # fn main() {
/// readvec!(x: i64);
/// # }
/// ```
/// Given an input `12 34 56 78`, sets `x: Vec<i64> = vec![12, 34, 56, 78]`.
#[macro_export]
macro_rules! readvec {
    ($x:ident: $t:ty) => (
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to readvec!");
        let mut $x = buffer.trim().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<$t>>();
    );
}

/// Reads a 2d vector of chars from stdin, assuming every line is an dimension 1 element.
///
/// Separating '\n' signs are dropped.
/// The resulting vector is mutable.
/// Width is used only to set initial Vec capacity appropriately.
///
/// # Examples
///
/// ```rust,no_run
/// # #[macro_use]
/// # extern crate olympic_lib;
/// # fn main() {
/// read2dchar!(2, 3, x);
/// # }
/// ```
/// Given an input `.*\n**\n..`, produces
/// `x: Vec<Vec<char>> = vec![vec!['.', '*'], vec!['*', '*'], vec!['.', '.']]`
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