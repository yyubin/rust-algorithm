use std::io::{self, Write};
use std::collections::HashMap;

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

    let mut map = HashMap::new();

    for v in nv {
        let val = map.entry(v).or_insert(0);
        *val += 1;
    }

    let mut output = String::new();

    for v in mv {
        let count = map.get(&v).unwrap_or(&0);
        output.push_str(&format!("{} ", count));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}