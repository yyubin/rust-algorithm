use std::io::{self, Write};
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut heap = BinaryHeap::new();
    let mut output = String::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();

        if x == 0 {
            if heap.is_empty() {
                output.push('0');
                output.push('\n');
            } else {
                output.push_str(&format!("{}\n", heap.pop().unwrap()));
            }
            continue;
        }
        heap.push(x);
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}