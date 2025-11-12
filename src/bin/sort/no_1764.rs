use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let mut nv: HashMap<String, i32> = HashMap::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();
        nv.insert(s.to_string(), 0);
    }

    let mut nm: Vec<String> = Vec::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();
        if nv.contains_key(s) {
            nm.push(s.to_string());
        }
    }

    let mut output = String::new();
    output.push_str(&format!("{}\n", nm.len()));
    
    nm.sort();
    for i in nm {
        output.push_str(&format!("{}\n", i));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();

}