use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut result: i32 = 1;
    for i in 2..=n {
        result *= i
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", result).unwrap();
}