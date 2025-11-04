use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut stack: Vec<i32> = Vec::new();
    let mut output = String::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        match parts.next().unwrap() {
            "push" => stack.push(parts.next().unwrap().parse().unwrap()),
            "pop" => output.push_str(&format!("{}\n", stack.pop().unwrap_or(-1))),
            "size" => output.push_str(&format!("{}\n", stack.len())),
            "empty" => output.push_str(&format!("{}\n", stack.is_empty() as i32)),
            "top" => output.push_str(&format!("{}\n", stack.last().copied().unwrap_or(-1))),
            _ => {}
        }
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}
