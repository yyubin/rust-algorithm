use std::io::{self, Write};

fn main() {
    let mut result = String::new();

    loop {
        let mut l = String::new();
        io::stdin().read_line(&mut l).unwrap();
        let nums: Vec<&str> = l.split_whitespace().collect();
        let n1: i32 = nums[0].parse().unwrap();
        let n2: i32 = nums[1].parse().unwrap();

        if n1 == 0 && n2 == 0 {
            break;
        }

        result.push_str(&format!("{}\n", n1+n2));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", result).unwrap();
}