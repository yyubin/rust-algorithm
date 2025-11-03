use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut result =  String::new();
    for i in 0..n {
        let tmp: String = std::iter::repeat("*").take(i+1).collect();
        result.push_str(&format!("{}\n", tmp));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", result).unwrap();

}