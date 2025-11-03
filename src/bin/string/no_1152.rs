mod no_2557;

use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let ss: Vec<&str> = s.split_whitespace().collect();
    println!("{}", ss.len());
}