mod no_10814;

use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut v: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        v.push(s);
    }

    v.sort_by_key(|x| (x[0], x[1]));

    let mut output = String::new();

    for i in v {
        output.push_str(&format!("{} {}\n", i[0], i[1]));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}