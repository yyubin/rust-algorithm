use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut output = String::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let m: i32 = input.trim().parse().unwrap();

        let mut map: HashMap<String, i32> = HashMap::new();
        for _ in 0..m {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let v: Vec<&str> = input.split_whitespace().collect();

            let key = v[1].to_string();
            *map.entry(key).or_insert(0) += 1;
        }

        let mut res = 1;
        for v in map.values() {
            res *= (v+1);
        }
        output.push_str(&format!("{}\n", res-1));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}