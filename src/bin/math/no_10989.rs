use std::io::{self, BufWriter, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();

    let mut counts = vec![0u32; 10001];

    for _ in 0..num {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        counts[n] += 1;
    }

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 1..=10000 {
        for _ in 0..counts[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}