use std::io::{self, Write};
use std::collections::{HashMap};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let mut map: HashMap<String, String> = HashMap::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|s| s.to_owned());;
        let (k, v) = (iter.next().unwrap(), iter.next().unwrap());
        map.insert(k, v);
    }

    let mut output = String::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let k = input.trim();
        let v = map.get(k).unwrap();
        output.push_str(&format!("{}\n", v));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}