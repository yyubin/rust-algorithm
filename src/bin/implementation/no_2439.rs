use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut result = String::new();
    for i in 0..n{
        for j in (0..n-i-1).rev() {
            result.push(' ');
        }
        for k in 0..i+1 {
            result.push('*');
        }
        result.push_str("\n");
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", result).unwrap();
}