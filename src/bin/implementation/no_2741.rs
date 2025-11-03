use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();

    let n: i32 = n.trim().parse().unwrap();

    let mut output = String::new();
    for i in 1..=n {
        output.push_str(&format!("{}\n", i));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}