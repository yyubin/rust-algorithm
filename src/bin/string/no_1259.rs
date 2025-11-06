use std::io::{self, Write};

fn main() {
    let mut output = String::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let s = input.trim();

        if s == "0" {
            break
        }

        let rev_s: String = s.chars().rev().collect();
        let res: &str = if s == rev_s {"yes"} else {"no"};
        output.push_str(&format!("{}\n", res));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}