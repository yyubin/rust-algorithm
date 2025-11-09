use std::io::{self, Write};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut nv: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut mv: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut s: HashSet<i32> = HashSet::new();
    for i in nv {
        s.insert(i);
    }

    let mut output = String::new();
    for i in mv {
        if s.contains(&i) {
            output.push_str(&format!("{}\n", 1));
            continue;
        }
        output.push_str(&format!("{}\n", 0));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}