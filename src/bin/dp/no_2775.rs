mod no_9095;

use std::io::{self, Write};

fn main() {
    let mut v: Vec<Vec<i32>> = vec![vec![0; 15]; 15];
    for i in 0..15 {
        v[0][i] = i as i32;
    }

    for i in 1..15 {
        for j in 1..15 {
            v[i][j] = v[i-1][j] + v[i][j-1];
        }
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut output = String::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let k1: i32 = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let k2: i32 = input.trim().parse().unwrap();

        output.push_str(&format!("{}\n", v[k1 as usize][k2 as usize]));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}