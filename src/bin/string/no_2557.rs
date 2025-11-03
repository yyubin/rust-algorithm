use std::io::{self, Write};

fn main() {
    let mut result: i64 = 1;
    for i in 0..3 {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: i64 = n.trim().parse().unwrap();
        result *= n;
    }

    let result_str = result.to_string();
    let mut output = String::new();

    for s in 0..10 {
        let digit = s.to_string();
        let cnt = result_str.matches(&digit).count();
        output.push_str(&format!("{}\n", cnt));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}
