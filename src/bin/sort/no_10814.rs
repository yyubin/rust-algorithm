use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let mut v: Vec<(i32, String)> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.split_whitespace();
        let age: i32 = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().to_string();

        v.push((age, name));
    }

    v.sort_by_key(|x| (x.0));

    let mut output = String::new();

    for (age, name) in v {
        output.push_str(&format!("{} {}\n", age, name));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}