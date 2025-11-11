use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());
    let mut nmap: HashMap<String, i32> = HashMap::new();
    let mut smap: HashMap<i32, String> = HashMap::new();

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();

        nmap.insert(s.to_string(), i+1);
        smap.insert(i+1, s.to_string());
    }

    let mut output = String::new();

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();

        if nmap.contains_key(s) {
            let i = nmap.get(s).unwrap();
            output.push_str(&format!("{}\n", i));
        } else {
            let i: i32 = s.trim().parse().unwrap();
            let ii = smap.get(&i).unwrap();
            output.push_str(&format!("{}\n", ii));
        }
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}